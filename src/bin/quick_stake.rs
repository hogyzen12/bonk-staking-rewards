// src/bin/quick_stake.rs
// Quick staking with preset configurations
// Usage: cargo run --bin quick-stake [amount] [duration-months]
// Example: cargo run --bin quick-stake 100 3

use bonk_staking_rewards::{
    build_deposit_transaction, derive_stake_deposit_receipt,
    StakeConfig, StakingError, STAKE_PROGRAM_ID, BONK_STAKE_POOL
};
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    commitment_config::CommitmentConfig,
    pubkey::Pubkey,
    signature::{read_keypair_file, Keypair, Signer},
};
use std::env;
use std::path::Path;
use std::str::FromStr;

const KEYPAIR_PATH: &str = "/Users/hogyzen12/.config/solana/6tBou5MHL5aWpDy6cgf3wiwGGK2mR8qs68ujtpaoWrf2.json";
const RPC_URL: &str = "https://mainnet.helius-rpc.com/?api-key=93812d12-f56f-4624-97c9-9a4d242db974";

fn load_keypair(path: &str) -> Result<Keypair, StakingError> {
    read_keypair_file(Path::new(path))
        .map_err(|e| StakingError::KeypairError(e.to_string()))
}

fn find_next_available_nonce(client: &RpcClient, owner: &Pubkey) -> u32 {
    let program_id = Pubkey::from_str(STAKE_PROGRAM_ID).unwrap();
    let stake_pool = Pubkey::from_str(BONK_STAKE_POOL).unwrap();
    
    for nonce in 0..100 {
        let (receipt_pda, _) = derive_stake_deposit_receipt(
            owner,
            &stake_pool,
            nonce,
            &program_id,
        );
        
        if client.get_account(&receipt_pda).is_err() {
            return nonce;
        }
    }
    100
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    
    // Parse command line arguments
    let (amount_bonk, duration_months) = if args.len() >= 3 {
        let amt = args[1].parse::<f64>().unwrap_or(100.0);
        let dur = args[2].parse::<u32>().unwrap_or(3);
        (amt, dur)
    } else {
        // Default: 100 BONK for 3 months
        println!("💡 Usage: cargo run --bin quick-stake [amount] [duration-months]");
        println!("   Example: cargo run --bin quick-stake 500 12");
        println!("\n📌 Using defaults: 100 BONK for 3 months\n");
        (100.0, 3)
    };

    // Validate duration
    let duration_seconds = match duration_months {
        1 => 30 * 24 * 60 * 60,    // 1 month
        3 => 90 * 24 * 60 * 60,    // 3 months
        6 => 180 * 24 * 60 * 60,   // 6 months
        12 => 365 * 24 * 60 * 60,  // 12 months
        _ => {
            println!("❌ Invalid duration! Use 1, 3, 6, or 12 months");
            return Ok(());
        }
    };

    println!("🚀 Quick Stake - {} BONK for {} month(s)", amount_bonk, duration_months);
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    // Load wallet
    let payer = load_keypair(KEYPAIR_PATH)?;
    println!("👤 Wallet: {}", payer.pubkey());

    // Connect to RPC
    let client = RpcClient::new_with_commitment(RPC_URL.to_string(), CommitmentConfig::confirmed());

    // Find next available nonce
    let nonce = find_next_available_nonce(&client, &payer.pubkey());
    println!("📍 Using nonce: {}", nonce);

    // Build config
    let config = StakeConfig {
        amount: (amount_bonk * 100_000.0) as u64, // Convert to lamports (5 decimals)
        lockup_duration: duration_seconds as u64,
        nonce,
    };

    // Get blockhash and send
    println!("📤 Sending transaction...");
    let recent_blockhash = client.get_latest_blockhash()?;
    let transaction = build_deposit_transaction(&payer, &config, recent_blockhash)?;
    
    match client.send_and_confirm_transaction(&transaction) {
        Ok(signature) => {
            println!("\n✅ SUCCESS!");
            println!("🔗 https://solscan.io/tx/{}", signature);
            println!("📅 Unlocks in {} days", duration_seconds / 86400);
        }
        Err(e) => {
            println!("\n❌ Failed: {}", e);
        }
    }

    Ok(())
}