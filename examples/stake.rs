// examples/stake.rs
//
/// Example: Comprehensive staking demonstration using BonkStakingClient
/// 
/// This example demonstrates the high-level client API for staking BONK tokens.
/// It shows how to:
/// - Create a client
/// - Check balances
/// - Stake tokens with different durations
/// - View active stakes
///
/// Run with: cargo run --example stake

use bonk_staking_rewards::{
    BonkStakingClient, 
    DURATION_1_MONTH,
    DURATION_3_MONTHS,
    DURATION_6_MONTHS,
    DURATION_12_MONTHS,
};
use solana_sdk::signature::{read_keypair_file, Signer};
use std::path::Path;

const KEYPAIR_PATH: &str = "/Users/hogyzen12/.config/solana/6tBou5MHL5aWpDy6cgf3wiwGGK2mR8qs68ujtpaoWrf2.json";
const RPC_URL: &str = "https://mainnet.helius-rpc.com/?api-key=93812d12-f56f-4624-97c9-9a4d242db974";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("BONK Staking Example\n");
    println!("=====================\n");

    // Load user keypair
    let user = read_keypair_file(Path::new(KEYPAIR_PATH))?;
    println!("Wallet Address: {}\n", user.pubkey());

    // Create the staking client
    let client = BonkStakingClient::new(RPC_URL.to_string());

    // Check current balances
    println!("Current Balances:");
    println!("-----------------");
    
    let bonk_balance = client.get_bonk_balance(&user.pubkey())?;
    let bonk_ui_balance = bonk_balance as f64 / 100_000.0; // BONK has 5 decimals
    println!("BONK Balance: {:.2} BONK ({} lamports)", bonk_ui_balance, bonk_balance);
    
    let stake_balance = client.get_stake_balance(&user.pubkey())?;
    let stake_ui_balance = stake_balance as f64 / 100_000.0;
    println!("Stake Token Balance: {:.2} ({} lamports)", stake_ui_balance, stake_balance);
    println!();

    // View existing stakes
    println!("Active Stakes:");
    println!("--------------");
    let stakes = client.get_user_stakes(&user.pubkey())?;
    if stakes.is_empty() {
        println!("No active stakes found");
    } else {
        for stake in &stakes {
            println!("  Nonce {}: Receipt {}", stake.nonce, stake.receipt_address);
        }
    }
    println!();

    // Example staking scenarios
    println!("Staking Examples:");
    println!("-----------------\n");

    // Example 1: Stake 100 BONK for 30 days
    println!("1. Stake 100 BONK for 30 days (1 month)");
    let amount_1 = 10_000_000u64; // 100 BONK
    println!("   Amount: {:.2} BONK", amount_1 as f64 / 100_000.0);
    println!("   Duration: {} days", DURATION_1_MONTH);
    println!("   Executing stake...");
    let sig = client.stake(&user, amount_1, DURATION_1_MONTH, None)?;
    println!("   Transaction: {}", sig);
    println!("   Success!");
    println!();

    // Example 2: Stake 500 BONK for 90 days
    println!("2. Stake 500 BONK for 90 days (3 months)");
    let amount_2 = 50_000_000u64; // 500 BONK
    println!("   Amount: {:.2} BONK", amount_2 as f64 / 100_000.0);
    println!("   Duration: {} days", DURATION_3_MONTHS);
    println!("   (Uncomment to execute)");
    // Uncomment to actually stake:
    // let sig = client.stake(&user, amount_2, DURATION_3_MONTHS, None)?;
    // println!("   Transaction: {}\n", sig);
    println!();

    // Example 3: Stake 1000 BONK for 180 days
    println!("3. Stake 1000 BONK for 180 days (6 months)");
    let amount_3 = 100_000_000u64; // 1000 BONK
    println!("   Amount: {:.2} BONK", amount_3 as f64 / 100_000.0);
    println!("   Duration: {} days", DURATION_6_MONTHS);
    println!("   (Uncomment to execute)");
    // Uncomment to actually stake:
    // let sig = client.stake(&user, amount_3, DURATION_6_MONTHS, None)?;
    // println!("   Transaction: {}\n", sig);
    println!();

    // Example 4: Stake 2000 BONK for 365 days with specific nonce
    println!("4. Stake 2000 BONK for 365 days (12 months) with nonce 5");
    let amount_4 = 200_000_000u64; // 2000 BONK
    println!("   Amount: {:.2} BONK", amount_4 as f64 / 100_000.0);
    println!("   Duration: {} days", DURATION_12_MONTHS);
    println!("   Nonce: 5 (manually specified)");
    println!("   (Uncomment to execute)");
    // Uncomment to actually stake:
    // let sig = client.stake(&user, amount_4, DURATION_12_MONTHS, Some(5))?;
    // println!("   Transaction: {}\n", sig);
    println!();

    println!("Examples Complete!");
    println!("\nNote: All staking operations are commented out for safety.");
    println!("Uncomment the stake() calls to actually execute transactions.");

    Ok(())
}