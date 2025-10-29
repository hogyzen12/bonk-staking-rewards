// src/bin/check_accounts.rs
// Add this to Cargo.toml under [[bin]]:
// [[bin]]
// name = "check-accounts"
// path = "src/bin/check_accounts.rs"

use bonk_staking_rewards::{
    derive_stake_deposit_receipt,
    STAKE_PROGRAM_ID, BONK_STAKE_POOL, BONK_MINT, BONK_STAKE_MINT,
    BONK_VAULT, BONK_REWARD_VAULT,
};
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

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔍 BONK Staking Account Diagnostics");
    println!("=====================================\n");

    // Load keypair
    let payer = read_keypair_file(Path::new(KEYPAIR_PATH))?;
    let owner = payer.pubkey();
    println!("👤 Your wallet: {}\n", owner);

    // Create RPC client
    let client = RpcClient::new_with_commitment(RPC_URL.to_string(), CommitmentConfig::confirmed());

    // Parse all the addresses
    let program_id = Pubkey::from_str(STAKE_PROGRAM_ID)?;
    let stake_pool = Pubkey::from_str(BONK_STAKE_POOL)?;
    let bonk_mint = Pubkey::from_str(BONK_MINT)?;
    let stake_mint = Pubkey::from_str(BONK_STAKE_MINT)?;
    let vault = Pubkey::from_str(BONK_VAULT)?;
    let reward_vault = Pubkey::from_str(BONK_REWARD_VAULT)?;

    println!("📋 Program Addresses:");
    println!("   Program ID:    {}", program_id);
    println!("   Stake Pool:    {}", stake_pool);
    println!("   BONK Mint:     {}", bonk_mint);
    println!("   Stake Mint:    {}", stake_mint);
    println!("   Vault:         {}", vault);
    println!("   Reward Vault:  {}\n", reward_vault);

    // Check SOL balance
    let sol_balance = client.get_balance(&owner)?;
    println!("💰 SOL Balance: {} SOL", sol_balance as f64 / 1_000_000_000.0);
    
    // Check BONK token account
    let bonk_ata = get_associated_token_address(&owner, &bonk_mint);
    println!("\n🪙  BONK Token Account: {}", bonk_ata);
    match client.get_token_account_balance(&bonk_ata) {
        Ok(balance) => {
            println!("   ✅ Exists");
            println!("   Balance: {} BONK", balance.ui_amount_string);
        }
        Err(_) => {
            println!("   ❌ Does not exist");
            println!("   You need to have BONK tokens first!");
        }
    }

    // Check stake token account (sBONK)
    let stake_ata = get_associated_token_address(&owner, &stake_mint);
    println!("\n📊 Stake Token Account (sBONK): {}", stake_ata);
    match client.get_token_account_balance(&stake_ata) {
        Ok(balance) => {
            println!("   ✅ Exists");
            println!("   Balance: {} sBONK", balance.ui_amount_string);
            println!("   This account already exists and can receive stake tokens");
        }
        Err(_) => {
            println!("   ❌ Does not exist");
            println!("   Will be created automatically when you stake for the first time");
        }
    }

    // Check stake deposit receipts for different nonces
    println!("\n🎫 Stake Deposit Receipts:");
    for nonce in 0..3 {
        let (receipt_pda, bump) = derive_stake_deposit_receipt(
            &owner,
            &stake_pool,
            nonce,
            &program_id,
        );
        
        print!("   Nonce {}: {} (bump: {})", nonce, receipt_pda, bump);
        
        // Check if this PDA exists
        match client.get_account(&receipt_pda) {
            Ok(account) => {
                println!(" ✅ EXISTS");
                println!("      Owner: {}", account.owner);
                println!("      Data len: {}", account.data.len());
                println!("      Lamports: {}", account.lamports);
                
                if nonce == 0 {
                    println!("\n   ⚠️  Nonce 0 already used! Use a different nonce for new stakes.");
                }
            }
            Err(_) => {
                println!(" ❌ Not initialized");
                if nonce == 0 {
                    println!("      This will be created when you stake with nonce 0");
                }
            }
        }
    }

    // Check if the vault accounts exist and are valid
    println!("\n🏦 Vault Accounts:");
    
    // Check BONK vault
    print!("   BONK Vault: ");
    match client.get_token_account_balance(&vault) {
        Ok(balance) => {
            println!("✅ Valid token account");
            println!("      Balance: {} BONK", balance.ui_amount_string);
        }
        Err(_) => {
            println!("❌ Invalid or doesn't exist");
        }
    }

    // Check reward vault
    print!("   Reward Vault: ");
    match client.get_token_account_balance(&reward_vault) {
        Ok(balance) => {
            println!("✅ Valid token account");
            println!("      Balance: {} BONK", balance.ui_amount_string);
        }
        Err(_) => {
            println!("❌ Invalid or doesn't exist");
        }
    }

    // Summary
    println!("\n📝 Summary:");
    if sol_balance < 10_000_000 {
        println!("   ❌ You need more SOL for transaction fees (at least 0.01 SOL)");
    } else {
        println!("   ✅ Sufficient SOL for fees");
    }

    match client.get_token_account_balance(&bonk_ata) {
        Ok(balance) => {
            if let Some(amount) = balance.ui_amount {
                if amount >= 100.0 {
                    println!("   ✅ Sufficient BONK balance for staking");
                } else {
                    println!("   ❌ Insufficient BONK (need at least 100 BONK)");
                }
            }
        }
        Err(_) => {
            println!("   ❌ No BONK tokens found");
        }
    }

    // Check if nonce 0 is available
    let (receipt_pda_0, _) = derive_stake_deposit_receipt(
        &owner,
        &stake_pool,
        0,
        &program_id,
    );
    
    if client.get_account(&receipt_pda_0).is_ok() {
        println!("   ⚠️  Nonce 0 is already used, try using nonce: 1 in your stake config");
    } else {
        println!("   ✅ Nonce 0 is available for staking");
    }

    println!("\n✨ Diagnostic complete!");

    Ok(())
}