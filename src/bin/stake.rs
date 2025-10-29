// src/bin/stake.rs
// Minimal BONK staking CLI - streamlined version

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
use spl_associated_token_account::get_associated_token_address;
use std::io::{self, Write};
use std::path::Path;
use std::str::FromStr;

const KEYPAIR_PATH: &str = "/Users/hogyzen12/.config/solana/6tBou5MHL5aWpDy6cgf3wiwGGK2mR8qs68ujtpaoWrf2.json";
const RPC_URL: &str = "https://mainnet.helius-rpc.com/?api-key=93812d12-f56f-4624-97c9-9a4d242db974";

fn load_keypair(path: &str) -> Result<Keypair, StakingError> {
    read_keypair_file(Path::new(path))
        .map_err(|e| StakingError::KeypairError(e.to_string()))
}

fn get_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn find_available_nonce(client: &RpcClient, owner: &Pubkey) -> u32 {
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
    println!("\n=== BONK STAKING CLI ===\n");

    // Load wallet
    println!("Loading wallet...");
    let payer = load_keypair(KEYPAIR_PATH)?;
    println!("Wallet: {}", payer.pubkey());

    // Connect to RPC
    let client = RpcClient::new_with_commitment(RPC_URL.to_string(), CommitmentConfig::confirmed());

    // Check balances
    let sol_balance = client.get_balance(&payer.pubkey())?;
    let bonk_mint = Pubkey::from_str("DezXAZ8z7PnrnRJjz3wXBoRgixCa6xjnB7YaB1pPB263")?;
    let bonk_account = get_associated_token_address(&payer.pubkey(), &bonk_mint);
    
    let bonk_balance = match client.get_token_account_balance(&bonk_account) {
        Ok(balance_info) => balance_info.ui_amount.unwrap_or(0.0),
        Err(_) => 0.0,
    };

    println!("\nBalances:");
    println!("  SOL:  {:.6} SOL", sol_balance as f64 / 1_000_000_000.0);
    println!("  BONK: {:.2} BONK", bonk_balance);
    
    if sol_balance < 10_000_000 {
        println!("\nError: Insufficient SOL for transaction fees");
        return Ok(());
    }
    
    if bonk_balance < 1.0 {
        println!("\nError: Insufficient BONK balance");
        return Ok(());
    }

    // Get stake amount
    println!("\n--- STAKE SETUP ---");
    let amount_str = get_input(&format!("Amount to stake (max {:.2} BONK): ", bonk_balance));
    let amount_bonk: f64 = amount_str.parse().unwrap_or(0.0);
    
    if amount_bonk <= 0.0 || amount_bonk > bonk_balance {
        println!("Error: Invalid amount");
        return Ok(());
    }
    
    // Select duration
    println!("\nLock Duration:");
    println!("  1) 1 month   (30 days)");
    println!("  2) 3 months  (90 days)");
    println!("  3) 6 months  (180 days)");
    println!("  4) 12 months (365 days)");
    
    let choice = get_input("Choice (1-4): ");
    let duration_seconds = match choice.as_str() {
        "1" => 30 * 24 * 60 * 60,
        "2" => 90 * 24 * 60 * 60,
        "3" => 180 * 24 * 60 * 60,
        "4" => 365 * 24 * 60 * 60,
        _ => {
            println!("Error: Invalid choice");
            return Ok(());
        }
    };

    // Find nonce
    let nonce = find_available_nonce(&client, &payer.pubkey());
    
    // Convert to lamports (BONK has 5 decimals)
    let amount_lamports = (amount_bonk * 100_000.0) as u64;

    // Summary
    println!("\n--- SUMMARY ---");
    println!("Amount:   {:.2} BONK ({} lamports)", amount_bonk, amount_lamports);
    println!("Duration: {} days", duration_seconds / 86400);
    println!("Nonce:    {}", nonce);

    let confirm = get_input("\nProceed? (yes/no): ");
    if !confirm.eq_ignore_ascii_case("yes") && !confirm.eq_ignore_ascii_case("y") {
        println!("Cancelled");
        return Ok(());
    }

    // Build transaction
    let config = StakeConfig {
        amount: amount_lamports,
        lockup_duration: duration_seconds,
        nonce,
    };

    println!("\nPreparing transaction...");
    let recent_blockhash = client.get_latest_blockhash()?;
    let transaction = build_deposit_transaction(&payer, &config, recent_blockhash)?;
    
    println!("Sending transaction...");
    match client.send_and_confirm_transaction(&transaction) {
        Ok(signature) => {
            println!("\n=== SUCCESS ===");
            println!("Transaction: {}", signature);
            println!("View: https://solscan.io/tx/{}", signature);
            println!("\nStake Details:");
            println!("  Amount: {:.2} BONK", amount_bonk);
            println!("  Unlock: {} days from now", duration_seconds / 86400);
            println!("  Nonce: {}", nonce);
        }
        Err(e) => {
            println!("\n=== FAILED ===");
            println!("Error: {}", e);
            
            // Provide helpful debugging info
            if e.to_string().contains("0xbbf") || e.to_string().contains("3007") {
                println!("\nThis error indicates an account ownership issue.");
                println!("The stake pool address has been corrected to:");
                println!("  {}", BONK_STAKE_POOL);
            }
        }
    }

    Ok(())
}