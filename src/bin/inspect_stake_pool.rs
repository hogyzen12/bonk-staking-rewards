// src/bin/inspect_stake_pool.rs
// Quick program to inspect the StakePool account and extract reward vaults

use borsh::BorshDeserialize;
use solana_client::rpc_client::RpcClient;
use solana_sdk::{commitment_config::CommitmentConfig, pubkey::Pubkey};
use std::str::FromStr;

const BONK_STAKE_POOL: &str = "9AdEE8AAm1XgJrPEs4zkTPozr3o4U5iGbgvPwkNdLDJ3";
const RPC_URL: &str = "https://mainnet.helius-rpc.com/?api-key=93812d12-f56f-4624-97c9-9a4d242db974";

#[derive(BorshDeserialize, Debug)]
struct RewardPool {
    reward_vault: Pubkey,
    rewards_per_effective_stake: u128,
    last_amount: u64,
    padding0: [u8; 8],
}

#[derive(BorshDeserialize, Debug)]
struct StakePool {
    authority: Pubkey,
    total_weighted_stake: u128,
    vault: Pubkey,
    mint: Pubkey,
    stake_mint: Pubkey,
    reward_pools: [RewardPool; 10],
    base_weight: u64,
    max_weight: u64,
    min_duration: u64,
    max_duration: u64,
    nonce: u8,
    bump_seed: u8,
    padding0: [u8; 6],
    reserved0: [u8; 8],
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Inspecting BONK StakePool...\n");

    let client = RpcClient::new_with_commitment(RPC_URL.to_string(), CommitmentConfig::confirmed());
    let stake_pool_pubkey = Pubkey::from_str(BONK_STAKE_POOL)?;

    println!("Fetching account data...");
    let account = client.get_account(&stake_pool_pubkey)?;
    
    println!("Account owner: {}", account.owner);
    println!("Data length: {} bytes\n", account.data.len());

    // Skip 8-byte discriminator
    let data = &account.data[8..];
    
    println!("Deserializing StakePool...");
    let stake_pool = StakePool::try_from_slice(data)?;

    println!("\nStakePool Details:");
    println!("  Authority: {}", stake_pool.authority);
    println!("  Total Weighted Stake: {}", stake_pool.total_weighted_stake);
    println!("  Vault: {}", stake_pool.vault);
    println!("  Mint: {}", stake_pool.mint);
    println!("  Stake Mint: {}", stake_pool.stake_mint);
    println!("  Base Weight: {}", stake_pool.base_weight);
    println!("  Max Weight: {}", stake_pool.max_weight);
    println!("  Min Duration: {} seconds ({} days)", stake_pool.min_duration, stake_pool.min_duration / 86400);
    println!("  Max Duration: {} seconds ({} days)", stake_pool.max_duration, stake_pool.max_duration / 86400);
    
    println!("\nReward Pools:");
    for (i, pool) in stake_pool.reward_pools.iter().enumerate() {
        // Check if this reward pool is initialized (non-default pubkey)
        if pool.reward_vault != Pubkey::default() {
            println!("  Pool {}: {}", i, pool.reward_vault);
            println!("    Rewards per stake: {}", pool.rewards_per_effective_stake);
            println!("    Last amount: {}", pool.last_amount);
        }
    }

    // Print Rust constant format for easy copy-paste
    println!("\n--- Constants for lib.rs ---");
    for (i, pool) in stake_pool.reward_pools.iter().enumerate() {
        if pool.reward_vault != Pubkey::default() {
            println!("pub const BONK_REWARD_VAULT_{}: Pubkey = solana_sdk::pubkey!(\"{}\");", i, pool.reward_vault);
        }
    }

    Ok(())
}