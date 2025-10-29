//! Instruction builders for BONK staking operations

use solana_sdk::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    system_program, sysvar,
};
use spl_token;

use crate::{
    accounts::{get_user_bonk_ata, get_user_stake_ata},
    pda::derive_stake_deposit_receipt,
    BONK_REWARD_VAULT_0, BONK_STAKE_MINT, BONK_STAKE_POOL, BONK_STAKE_PROGRAM_ID, BONK_VAULT,
};

/// Build the deposit (stake) instruction
///
/// # Arguments
/// * `user` - The user's public key
/// * `amount` - Amount of BONK to stake (in lamports, not UI amount)
/// * `lock_duration` - Lock duration in seconds
/// * `nonce` - Nonce for the stake deposit receipt PDA
///
/// # Returns
/// The stake deposit instruction
pub fn build_stake_instruction(
    user: &Pubkey,
    amount: u64,
    lock_duration: u64,
    nonce: u32,
) -> Instruction {
    // Derive the stake deposit receipt PDA
    let (stake_deposit_receipt, _) = derive_stake_deposit_receipt(user, &BONK_STAKE_POOL, nonce);

    // Get token accounts
    let user_bonk_ata = get_user_bonk_ata(user);
    let user_stake_ata = get_user_stake_ata(user);

    // Build instruction data
    // Format: [discriminator(8), nonce(4), amount(8), lockupDuration(8)]
    let mut data = Vec::with_capacity(28);
    
    // Discriminator for "deposit" instruction (from IDL)
    data.extend_from_slice(&[242, 35, 198, 137, 82, 225, 242, 182]);
    
    // Nonce (u32 little-endian)
    data.extend_from_slice(&nonce.to_le_bytes());
    
    // Amount (u64 little-endian)
    data.extend_from_slice(&amount.to_le_bytes());
    
    // Lockup duration (u64 little-endian)
    data.extend_from_slice(&lock_duration.to_le_bytes());

    // Build accounts list
    let mut accounts = vec![
        AccountMeta::new(*user, true),                      // payer
        AccountMeta::new(*user, true),                      // owner
        AccountMeta::new(user_bonk_ata, false),            // from (user's BONK ATA)
        AccountMeta::new(BONK_VAULT, false),               // vault
        AccountMeta::new(BONK_STAKE_MINT, false),          // stake_mint
        AccountMeta::new(user_stake_ata, false),           // destination (user's stake ATA)
        AccountMeta::new(BONK_STAKE_POOL, false),          // stake_pool
        AccountMeta::new(stake_deposit_receipt, false),    // stake_deposit_receipt
        AccountMeta::new_readonly(spl_token::id(), false), // token_program
        AccountMeta::new_readonly(sysvar::rent::id(), false), // rent
        AccountMeta::new_readonly(system_program::id(), false), // system_program
    ];
    
    // Add remaining accounts: reward pool vaults (required by the program)
    // These must be in the same order as StakePool.reward_pools
    accounts.push(AccountMeta::new(BONK_REWARD_VAULT_0, false));

    Instruction {
        program_id: BONK_STAKE_PROGRAM_ID,
        accounts,
        data,
    }
}

/// Build compute budget set compute unit price instruction
///
/// # Arguments
/// * `micro_lamports` - Price per compute unit in micro-lamports
pub fn build_compute_budget_price_instruction(micro_lamports: u64) -> Instruction {
    let data = [3u8]
        .iter()
        .chain(&micro_lamports.to_le_bytes())
        .copied()
        .collect::<Vec<u8>>();

    Instruction {
        program_id: solana_sdk::pubkey!("ComputeBudget111111111111111111111111111111"),
        accounts: vec![],
        data,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_build_stake_instruction() {
        let user = Pubkey::from_str("6tBou5MHL5aWpDy6cgf3wiwGGK2mR8qs68ujtpaoWrf2").unwrap();
        let amount = 1_000_000u64; // 10 BONK (with 5 decimals)
        let duration = 15_552_000u64; // 180 days in seconds
        let nonce = 1u32;

        let ix = build_stake_instruction(&user, amount, duration, nonce);

        assert_eq!(ix.program_id, BONK_STAKE_PROGRAM_ID);
        assert_eq!(ix.accounts.len(), 12);
        
        // Verify instruction data format
        assert_eq!(ix.data.len(), 28); // 8 + 4 + 8 + 8
        
        // Verify discriminator
        assert_eq!(&ix.data[0..8], &[242, 35, 198, 137, 82, 225, 242, 182]);
    }
}