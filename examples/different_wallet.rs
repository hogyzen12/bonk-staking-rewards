// examples/different_wallet.rs
//
/// Example: Staking with a different wallet
/// 
/// This example demonstrates how to stake BONK tokens using a different
/// wallet than the default one. This is useful when you want to:
/// - Stake from multiple wallets
/// - Use a different keypair for testing
/// - Manage stakes across different accounts
///
/// Run with: cargo run --example different_wallet

use bonk_staking_rewards::{BonkStakingClient, DURATION_6_MONTHS};
use solana_sdk::signature::{read_keypair_file, Keypair, Signer};
use std::path::Path;

// Configuration
const RPC_URL: &str = "https://mainnet.helius-rpc.com/?api-key=93812d12-f56f-4624-97c9-9a4d242db974";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("BONK Staking - Different Wallet Example\n");
    println!("=========================================\n");

    // Example 1: Load from a different keypair file
    println!("Example 1: Using a different keypair file");
    println!("------------------------------------------");
    
    // Replace this with your actual keypair path
    let custom_keypair_path = "/path/to/your/custom/wallet.json";
    
    println!("Note: Update the keypair path in the code to use your wallet");
    println!("Current path: {}", custom_keypair_path);
    
    // Uncomment when you have a valid keypair path:
    // let custom_wallet = read_keypair_file(Path::new(custom_keypair_path))?;
    // println!("Loaded wallet: {}\n", custom_wallet.pubkey());
    
    // Example 2: Generate a new keypair (for testing)
    println!("\nExample 2: Using a freshly generated keypair (testing only)");
    println!("------------------------------------------------------------");
    
    let test_wallet = Keypair::new();
    println!("Generated wallet: {}", test_wallet.pubkey());
    println!("Warning: This is a new wallet with no funds!\n");

    // Create client
    let client = BonkStakingClient::new(RPC_URL.to_string());

    // Check balance of the test wallet
    let bonk_balance = client.get_bonk_balance(&test_wallet.pubkey())?;
    println!("BONK Balance: {:.2} BONK", bonk_balance as f64 / 100_000.0);
    
    if bonk_balance == 0 {
        println!("\nThis wallet has no BONK tokens.");
        println!("To stake, you would need to:");
        println!("  1. Send some SOL to this address for transaction fees");
        println!("  2. Send BONK tokens to this address");
        println!("  3. Then call client.stake()");
    } else {
        println!("\nThis wallet has BONK! You can stake:");
        let amount = 10_000_000u64; // 100 BONK
        println!("  Amount: {:.2} BONK", amount as f64 / 100_000.0);
        println!("  Duration: {} days", DURATION_6_MONTHS);
        
        // Uncomment to actually stake:
        // let sig = client.stake(&test_wallet, amount, DURATION_6_MONTHS, None)?;
        // println!("  Transaction: {}", sig);
    }

    // Example 3: Using environment variable for keypair path
    println!("\n\nExample 3: Using environment variable for keypair");
    println!("---------------------------------------------------");
    
    match std::env::var("BONK_WALLET_PATH") {
        Ok(path) => {
            println!("Found BONK_WALLET_PATH: {}", path);
            
            // Uncomment to load from environment:
            // let env_wallet = read_keypair_file(Path::new(&path))?;
            // println!("Loaded wallet: {}", env_wallet.pubkey());
            
            // let balance = client.get_bonk_balance(&env_wallet.pubkey())?;
            // println!("Balance: {:.2} BONK", balance as f64 / 100_000.0);
        }
        Err(_) => {
            println!("No BONK_WALLET_PATH environment variable set");
            println!("You can set it with:");
            println!("  export BONK_WALLET_PATH=/path/to/your/wallet.json");
        }
    }

    // Example 4: Comparing multiple wallets
    println!("\n\nExample 4: Comparing balances across wallets");
    println!("----------------------------------------------");
    
    let wallet_paths = vec![
        "/Users/hogyzen12/.config/solana/6tBou5MHL5aWpDy6cgf3wiwGGK2mR8qs68ujtpaoWrf2.json",
        // Add more wallet paths here
    ];

    for (i, path) in wallet_paths.iter().enumerate() {
        match read_keypair_file(Path::new(path)) {
            Ok(wallet) => {
                let balance = client.get_bonk_balance(&wallet.pubkey())?;
                let stake_balance = client.get_stake_balance(&wallet.pubkey())?;
                
                println!("\nWallet {}: {}", i + 1, wallet.pubkey());
                println!("  BONK: {:.2}", balance as f64 / 100_000.0);
                println!("  Stake Tokens: {:.2}", stake_balance as f64 / 100_000.0);
                
                let stakes = client.get_user_stakes(&wallet.pubkey())?;
                println!("  Active Stakes: {}", stakes.len());
            }
            Err(e) => {
                println!("\nWallet {}: Failed to load ({})", i + 1, e);
            }
        }
    }

    // Example 5: Actually stake with the default wallet (if it has funds)
    println!("\n\nExample 5: Executing a stake transaction");
    println!("------------------------------------------");
    
    let default_wallet_path = "/Users/hogyzen12/.config/solana/6tBou5MHL5aWpDy6cgf3wiwGGK2mR8qs68ujtpaoWrf2.json";
    match read_keypair_file(Path::new(default_wallet_path)) {
        Ok(wallet) => {
            let balance = client.get_bonk_balance(&wallet.pubkey())?;
            println!("Wallet: {}", wallet.pubkey());
            println!("Balance: {:.2} BONK", balance as f64 / 100_000.0);
            
            if balance >= 10_000_000 {
                println!("\nStaking 100 BONK for 180 days...");
                let amount = 10_000_000u64; // 100 BONK
                let sig = client.stake(&wallet, amount, DURATION_6_MONTHS, None)?;
                println!("Transaction: {}", sig);
                println!("Success!");
            } else {
                println!("\nInsufficient BONK balance to stake (need at least 100 BONK)");
            }
        }
        Err(e) => {
            println!("Could not load default wallet: {}", e);
        }
    }

    println!("\n\nComplete!");
    println!("\nKey Takeaways:");
    println!("  - You can use any Keypair with the BonkStakingClient");
    println!("  - Load from files with read_keypair_file()");
    println!("  - Generate new ones with Keypair::new()");
    println!("  - Use environment variables for flexibility");
    println!("  - Each wallet maintains its own stakes independently");

    Ok(())
}