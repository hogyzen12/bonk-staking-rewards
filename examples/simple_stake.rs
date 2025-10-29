// examples/simple_stake.rs
//
/// Example: Simple staking with custom configuration
/// 
/// Run with: cargo run --example simple_stake

use bonk_staking_rewards::{
    build_deposit_transaction, 
    derive_stake_deposit_receipt,
    StakeConfig, 
    STAKE_PROGRAM_ID,
    BONK_STAKE_POOL,
};
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    commitment_config::CommitmentConfig,
    signature::{read_keypair_file, Keypair, Signer},
};
use std::path::Path;
use std::str::FromStr;

const KEYPAIR_PATH: &str = "/Users/hogyzen12/.config/solana/6tBou5MHL5aWpDy6cgf3wiwGGK2mR8qs68ujtpaoWrf2.json";
const RPC_URL: &str = "https://mainnet.helius-rpc.com/?api-key=93812d12-f56f-4624-97c9-9a4d242db974";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔧 Simple Stake Example\n");

    // Load keypair
    let payer = read_keypair_file(Path::new(KEYPAIR_PATH))?;
    println!("Wallet: {}", payer.pubkey());

    // Setup client
    let client = RpcClient::new_with_commitment(
        RPC_URL.to_string(), 
        CommitmentConfig::confirmed()
    );

    // Example 1: Quick stake with defaults
    println!("\n📍 Example 1: Stake with default config (100 BONK, 90 days)");
    let default_config = StakeConfig::default();
    stake_with_config(&client, &payer, &default_config)?;

    // Example 2: Custom amount and duration
    println!("\n📍 Example 2: Stake 500 BONK for 180 days");
    let custom_config = StakeConfig {
        amount: 50_000_000,      // 500 BONK
        lockup_duration: 15_552_000, // 180 days
        nonce: 1,                // Different nonce for second stake
    };
    // Uncomment to actually stake:
    // stake_with_config(&client, &payer, &custom_config)?;

    // Example 3: Check what your stake receipt address would be
    println!("\n📍 Example 3: Derive stake receipt address");
    let program_id = solana_sdk::pubkey::Pubkey::from_str(STAKE_PROGRAM_ID)?;
    let stake_pool = solana_sdk::pubkey::Pubkey::from_str(BONK_STAKE_POOL)?;
    let (receipt_address, bump) = derive_stake_deposit_receipt(
        &payer.pubkey(),
        &stake_pool,
        0, // nonce
        &program_id,
    );
    println!("Your stake receipt address: {}", receipt_address);
    println!("Bump seed: {}", bump);

    println!("\n✅ Examples complete!");
    
    Ok(())
}

fn stake_with_config(
    client: &RpcClient,
    payer: &Keypair,
    config: &StakeConfig,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("  Amount: {} BONK", config.amount as f64 / 100_000.0);
    println!("  Duration: {} days", config.lockup_duration / 86400);
    println!("  Nonce: {}", config.nonce);

    // Get recent blockhash
    let recent_blockhash = client.get_latest_blockhash()?;

    // Build transaction (but don't send in this example)
    let _transaction = build_deposit_transaction(payer, config, recent_blockhash)?;
    
    println!("  ✓ Transaction built successfully");
    println!("  (Uncomment code to actually send the transaction)");

    // To actually send:
    // let signature = client.send_and_confirm_transaction(&transaction)?;
    // println!("  🎉 Staked! Signature: {}", signature);

    Ok(())
}