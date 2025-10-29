// src/bin/stake_manager.rs
// Tool to view and manage multiple stake positions
// Add to Cargo.toml:
// [[bin]]
// name = "stake-manager"
// path = "src/bin/stake_manager.rs"

use bonk_staking_rewards::{
    derive_stake_deposit_receipt,
    STAKE_PROGRAM_ID, BONK_STAKE_POOL, BONK_STAKE_MINT,
};
use borsh::{BorshDeserialize, BorshSerialize};
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    commitment_config::CommitmentConfig,
    pubkey::Pubkey,
    signature::{read_keypair_file, Signer},
};
use spl_associated_token_account::get_associated_token_address;
use std::path::Path;
use std::str::FromStr;

const KEYPAIR_PATH: &str = "/Users/hogyzen12/.config/solana/6tBou5MHL5aWpDy6cgf3wiwGGK2mR8qs68ujtpaoWrf2.json";
const RPC_URL: &str = "https://mainnet.helius-rpc.com/?api-key=93812d12-f56f-4624-97c9-9a4d242db974";

// Stake receipt data structure (based on the program's IDL)
#[derive(BorshSerialize, BorshDeserialize, Debug)]
struct StakeDepositReceipt {
    pub payer: Pubkey,
    pub stake_pool: Pubkey,
    pub lock_up_duration: u64,
    pub deposit_timestamp: i64,
    pub stake_mint_claimed: u64,
    pub vault_claimed: u64,
    pub effective_stake: u128,
    pub effective_stake_pda_bump: u8,
}

fn format_duration(seconds: u64) -> String {
    let days = seconds / 86400;
    if days >= 365 {
        format!("{} year(s)", days / 365)
    } else if days >= 30 {
        format!("{} month(s)", days / 30)
    } else {
        format!("{} day(s)", days)
    }
}

fn format_timestamp(timestamp: i64) -> String {
    use std::time::{SystemTime, UNIX_EPOCH, Duration};
    
    let d = UNIX_EPOCH + Duration::from_secs(timestamp as u64);
    let datetime = SystemTime::from(d);
    
    // Simple date formatting
    let elapsed = datetime.duration_since(UNIX_EPOCH).unwrap();
    let seconds = elapsed.as_secs();
    let days_since_epoch = seconds / 86400;
    
    // Approximate date (this is simplified - for production use chrono crate)
    let year = 1970 + (days_since_epoch / 365);
    let remaining_days = days_since_epoch % 365;
    let month = (remaining_days / 30) + 1;
    let day = (remaining_days % 30) + 1;
    
    format!("{:04}-{:02}-{:02}", year, month, day)
}

fn calculate_unlock_date(deposit_timestamp: i64, lockup_duration: u64) -> String {
    let unlock_timestamp = deposit_timestamp + lockup_duration as i64;
    let current_timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64;
    
    let remaining_seconds = unlock_timestamp - current_timestamp;
    
    if remaining_seconds <= 0 {
        "🔓 Unlocked - Ready to withdraw!".to_string()
    } else {
        let remaining_days = remaining_seconds / 86400;
        format!("🔒 {} days remaining", remaining_days)
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("📊 BONK Stake Manager");
    println!("=====================\n");

    // Load keypair
    let payer = read_keypair_file(Path::new(KEYPAIR_PATH))?;
    let owner = payer.pubkey();
    println!("👤 Wallet: {}\n", owner);

    // Create RPC client
    let client = RpcClient::new_with_commitment(RPC_URL.to_string(), CommitmentConfig::confirmed());

    // Get program and pool addresses
    let program_id = Pubkey::from_str(STAKE_PROGRAM_ID)?;
    let stake_pool = Pubkey::from_str(BONK_STAKE_POOL)?;

    // Check sBONK balance
    let stake_mint = Pubkey::from_str(BONK_STAKE_MINT)?;
    let stake_token_account = get_associated_token_address(&owner, &stake_mint);
    
    match client.get_token_account_balance(&stake_token_account) {
        Ok(balance) => {
            println!("💎 Total sBONK Balance: {} sBONK\n", balance.ui_amount_string);
        }
        Err(_) => {
            println!("💎 Total sBONK Balance: 0 sBONK (no stake token account)\n");
        }
    }

    // Check for stake positions
    println!("🔍 Scanning for stake positions (checking nonces 0-20)...\n");
    println!("═══════════════════════════════════════════════════════════");
    
    let mut total_staked = 0u64;
    let mut active_stakes = 0;
    
    for nonce in 0..20 {
        let (receipt_pda, _bump) = derive_stake_deposit_receipt(
            &owner,
            &stake_pool,
            nonce,
            &program_id,
        );
        
        match client.get_account(&receipt_pda) {
            Ok(account) => {
                active_stakes += 1;
                
                // Try to deserialize the account data
                match StakeDepositReceipt::try_from_slice(&account.data[8..]) {
                    Ok(receipt) => {
                        println!("\n📌 Stake Position #{}", nonce);
                        println!("   Receipt: {}", receipt_pda);
                        println!("   ────────────────────────────────");
                        println!("   Amount staked:    {} BONK", receipt.vault_claimed as f64 / 100_000.0);
                        println!("   sBONK received:   {} sBONK", receipt.stake_mint_claimed as f64 / 100_000.0);
                        println!("   Effective stake:  {}", receipt.effective_stake);
                        println!("   Lock duration:    {}", format_duration(receipt.lock_up_duration));
                        println!("   Staked on:        {}", format_timestamp(receipt.deposit_timestamp));
                        println!("   Status:           {}", calculate_unlock_date(receipt.deposit_timestamp, receipt.lock_up_duration));
                        
                        total_staked += receipt.vault_claimed;
                        
                        // Calculate multiplier based on duration
                        let multiplier = match receipt.lock_up_duration {
                            d if d <= 2_592_000 => "1.0x",
                            d if d <= 7_776_000 => "1.5x",
                            d if d <= 15_552_000 => "2.25x",
                            _ => "3.2x",
                        };
                        println!("   Multiplier:       {}", multiplier);
                    }
                    Err(e) => {
                        println!("\n📌 Stake Position #{}", nonce);
                        println!("   Receipt: {}", receipt_pda);
                        println!("   ⚠️  Could not decode stake data: {}", e);
                        println!("   (Account exists but data format may have changed)");
                    }
                }
                println!("   ────────────────────────────────");
            }
            Err(_) => {
                // Account doesn't exist, skip silently
            }
        }
    }
    
    if active_stakes == 0 {
        println!("\n❌ No active stake positions found");
        println!("   Use 'cargo run --bin stake-bonk' to create your first stake!");
    } else {
        println!("\n═══════════════════════════════════════════════════════════");
        println!("\n📈 Summary:");
        println!("   Active positions: {}", active_stakes);
        println!("   Total BONK staked: {:.2} BONK", total_staked as f64 / 100_000.0);
        
        // Provide instructions
        println!("\n💡 Tips:");
        println!("   • Each stake position has its own unlock timer");
        println!("   • You can have multiple stakes with different durations");
        println!("   • Unlocked stakes can be withdrawn after the lock period");
        println!("   • Use different nonces to create additional stake positions");
    }
    
    println!("\n✨ Done!");

    Ok(())
}