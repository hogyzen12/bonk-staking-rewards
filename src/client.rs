//! High-level client for BONK staking operations

use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    commitment_config::CommitmentConfig,
    instruction::Instruction,
    pubkey::Pubkey,
    signature::{Keypair, Signature},
    signer::Signer,
    transaction::Transaction,
};
use spl_associated_token_account::instruction::create_associated_token_account_idempotent;

use crate::{
    accounts::{get_user_bonk_ata, get_user_stake_ata, StakeInfo},
    error::{BonkStakingError, Result},
    instructions::{build_compute_budget_price_instruction, build_stake_instruction},
    pda::derive_stake_deposit_receipt,
    BONK_MINT, BONK_STAKE_MINT, BONK_STAKE_POOL,
};

/// High-level client for BONK staking operations
pub struct BonkStakingClient {
    /// RPC client for communicating with Solana
    rpc: RpcClient,
}

impl BonkStakingClient {
    /// Create a new BonkStakingClient
    ///
    /// # Arguments
    /// * `rpc_url` - The Solana RPC endpoint URL
    ///
    /// # Example
    /// ```
    /// use bonk_staking_rewards::BonkStakingClient;
    ///
    /// let client = BonkStakingClient::new(
    ///     "https://mainnet.helius-rpc.com/?api-key=YOUR_KEY".to_string()
    /// );
    /// ```
    pub fn new(rpc_url: String) -> Self {
        let rpc = RpcClient::new_with_commitment(rpc_url, CommitmentConfig::confirmed());
        Self { rpc }
    }

    /// Stake BONK tokens
    ///
    /// # Arguments
    /// * `user` - The user's keypair
    /// * `amount` - Amount of BONK to stake (in lamports, not UI amount)
    /// * `lock_duration_days` - Lock duration in days (30, 90, 180, or 365)
    /// * `nonce` - Nonce for the stake deposit receipt (use None for auto-select)
    ///
    /// # Returns
    /// Transaction signature
    ///
    /// # Example
    /// ```no_run
    /// use bonk_staking_rewards::BonkStakingClient;
    /// use solana_sdk::signature::{Keypair, Signer};
    ///
    /// let client = BonkStakingClient::new("https://api.mainnet-beta.solana.com".to_string());
    /// let user = Keypair::new();
    /// let amount = 10_000_000; // 100 BONK (5 decimals)
    /// let signature = client.stake(&user, amount, 180, None).unwrap();
    /// ```
    pub fn stake(
        &self,
        user: &Keypair,
        amount: u64,
        lock_duration_days: u64,
        nonce: Option<u32>,
    ) -> Result<Signature> {
        let user_pubkey = user.pubkey();

        // Validate amount
        if amount == 0 {
            return Err(BonkStakingError::InvalidAmount(
                "Amount must be greater than 0".to_string(),
            ));
        }

        // Validate and convert duration
        let lock_duration_seconds = match lock_duration_days {
            30 => 30 * 24 * 60 * 60,      // 1 month
            90 => 90 * 24 * 60 * 60,      // 3 months
            180 => 180 * 24 * 60 * 60,    // 6 months
            365 => 365 * 24 * 60 * 60,    // 12 months
            _ => {
                return Err(BonkStakingError::InvalidDuration(
                    "Duration must be 30, 90, 180, or 365 days".to_string(),
                ))
            }
        };

        // Get or auto-select nonce
        let stake_nonce = match nonce {
            Some(n) => n,
            None => self.find_next_available_nonce(&user_pubkey)?,
        };

        // Check BONK balance
        let bonk_balance = self.get_bonk_balance(&user_pubkey)?;
        if bonk_balance < amount {
            return Err(BonkStakingError::InsufficientBalance {
                required: amount,
                available: bonk_balance,
            });
        }

        // Build instructions
        let mut instructions = Vec::new();

        // Add compute budget (matching successful transactions)
        instructions.push(build_compute_budget_price_instruction(5045));

        // Create stake token ATA if needed (idempotent)
        let create_stake_ata_ix = create_associated_token_account_idempotent(
            &user_pubkey,
            &user_pubkey,
            &BONK_STAKE_MINT,
            &spl_token::id(),
        );
        instructions.push(create_stake_ata_ix);

        // Build stake instruction
        let stake_ix = build_stake_instruction(&user_pubkey, amount, lock_duration_seconds, stake_nonce);
        instructions.push(stake_ix);

        // Send transaction
        self.send_transaction(&instructions, user)
    }

    /// Get user's BONK balance
    ///
    /// # Arguments
    /// * `user` - The user's public key
    ///
    /// # Returns
    /// BONK balance in lamports
    pub fn get_bonk_balance(&self, user: &Pubkey) -> Result<u64> {
        let bonk_ata = get_user_bonk_ata(user);

        match self.rpc.get_token_account_balance(&bonk_ata) {
            Ok(balance) => Ok(balance.amount.parse().unwrap_or(0)),
            Err(_) => Ok(0), // Account doesn't exist yet
        }
    }

    /// Get user's stake token balance
    ///
    /// # Arguments
    /// * `user` - The user's public key
    ///
    /// # Returns
    /// Stake token balance in lamports
    pub fn get_stake_balance(&self, user: &Pubkey) -> Result<u64> {
        let stake_ata = get_user_stake_ata(user);

        match self.rpc.get_token_account_balance(&stake_ata) {
            Ok(balance) => Ok(balance.amount.parse().unwrap_or(0)),
            Err(_) => Ok(0), // Account doesn't exist yet
        }
    }

    /// Find the next available nonce for a user
    ///
    /// Checks nonces 0-99 and returns the first one without an existing account
    fn find_next_available_nonce(&self, user: &Pubkey) -> Result<u32> {
        for nonce in 0..100 {
            let (receipt_pda, _) = derive_stake_deposit_receipt(user, &BONK_STAKE_POOL, nonce);
            
            // If account doesn't exist, this nonce is available
            if self.rpc.get_account(&receipt_pda).is_err() {
                return Ok(nonce);
            }
        }

        Err(BonkStakingError::InvalidNonce(
            "No available nonce found (0-99 all in use)".to_string(),
        ))
    }

    /// Get user's active stakes
    ///
    /// Scans nonces 0-99 for existing stake deposit receipts
    ///
    /// # Arguments
    /// * `user` - The user's public key
    ///
    /// # Returns
    /// Vector of active stakes
    pub fn get_user_stakes(&self, user: &Pubkey) -> Result<Vec<StakeInfo>> {
        let mut stakes = Vec::new();

        for nonce in 0..100 {
            let (receipt_pda, _) = derive_stake_deposit_receipt(user, &BONK_STAKE_POOL, nonce);
            
            if let Ok(account) = self.rpc.get_account(&receipt_pda) {
                // Account exists, parse stake info
                // Note: This is simplified - you'd need to deserialize the actual account data
                stakes.push(StakeInfo {
                    receipt_address: receipt_pda,
                    nonce,
                    amount: 0, // Would parse from account data
                    lock_duration: 0, // Would parse from account data
                    created_at: 0, // Would parse from account data
                    unlock_at: 0, // Would parse from account data
                });
            }
        }

        Ok(stakes)
    }

    /// Send a transaction with the given instructions
    fn send_transaction(&self, instructions: &[Instruction], signer: &Keypair) -> Result<Signature> {
        let recent_blockhash = self.rpc.get_latest_blockhash()?;

        let transaction = Transaction::new_signed_with_payer(
            instructions,
            Some(&signer.pubkey()),
            &[signer],
            recent_blockhash,
        );

        let signature = self
            .rpc
            .send_and_confirm_transaction(&transaction)
            .map_err(|e| BonkStakingError::TransactionFailed(e.to_string()))?;

        Ok(signature)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_creation() {
        let client = BonkStakingClient::new("https://api.mainnet-beta.solana.com".to_string());
        // Just verify it constructs without panic
        let _ = client;
    }
}