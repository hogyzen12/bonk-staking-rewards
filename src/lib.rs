//! # BONK Staking Rewards SDK
//!
//! A Rust client library for interacting with the BONK staking program on Solana.
//! This library provides a clean, type-safe interface for staking BONK tokens and
//! managing stake positions.
//!
//! ## Features
//!
//! - **Simple API**: Easy-to-use client for staking operations
//! - **Type-Safe**: Strongly typed interfaces for all operations
//! - **PDA Utilities**: Helper functions for deriving program-derived addresses
//! - **Error Handling**: Comprehensive error types with descriptive messages
//!
//! ## Usage
//!
//! ```no_run
//! use bonk_staking_rewards::BonkStakingClient;
//! use solana_sdk::signature::{read_keypair_file, Signer};
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! // Create client
//! let client = BonkStakingClient::new(
//!     "https://mainnet.helius-rpc.com/?api-key=YOUR_KEY".to_string()
//! );
//!
//! // Load user keypair
//! let user = read_keypair_file("path/to/keypair.json")?;
//!
//! // Stake 100 BONK for 180 days
//! let amount = 10_000_000; // 100 BONK (5 decimals)
//! let signature = client.stake(&user, amount, 180, None)?;
//!
//! println!("Staked! Transaction: {}", signature);
//! # Ok(())
//! # }
//! ```

pub mod accounts;
pub mod client;
pub mod error;
pub mod instructions;
pub mod pda;

// Re-export commonly used types
pub use client::BonkStakingClient;
pub use error::{BonkStakingError, Result};
pub use accounts::StakeInfo;

use solana_sdk::pubkey::Pubkey;

/// BONK token mint address
pub const BONK_MINT: Pubkey = solana_sdk::pubkey!("DezXAZ8z7PnrnRJjz3wXBoRgixCa6xjnB7YaB1pPB263");

/// BONK Stake Program ID
pub const BONK_STAKE_PROGRAM_ID: Pubkey = solana_sdk::pubkey!("STAKEkKzbdeKkqzKpLkNQD3SUuLgshDKCD7U8duxAbB");

/// BONK Stake Pool address
pub const BONK_STAKE_POOL: Pubkey = solana_sdk::pubkey!("9AdEE8AAm1XgJrPEs4zkTPozr3o4U5iGbgvPwkNdLDJ3");

/// BONK Vault (where staked BONK is held)
pub const BONK_VAULT: Pubkey = solana_sdk::pubkey!("4XHP9YQeeXPXHAjNXuKio1na1ypcxFSqFYBHtptQticd");

/// BONK Stake Mint (the token you receive when staking)
pub const BONK_STAKE_MINT: Pubkey = solana_sdk::pubkey!("FYUjeMAFjbTzdMG91RSW5P4HT2sT7qzJQgDPiPG9ez9o");

/// BONK Reward Vault (reward pool 0)
pub const BONK_REWARD_VAULT_0: Pubkey = solana_sdk::pubkey!("2PPAJ8P5JgKZjkxq4h3kFSwLcuakFYr4fbV68jGghWxi");

/// Lock duration for 1 month in days
pub const DURATION_1_MONTH: u64 = 30;

/// Lock duration for 3 months in days
pub const DURATION_3_MONTHS: u64 = 90;

/// Lock duration for 6 months in days
pub const DURATION_6_MONTHS: u64 = 180;

/// Lock duration for 12 months in days
pub const DURATION_12_MONTHS: u64 = 365;