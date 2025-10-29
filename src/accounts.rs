//! Account types and utilities for BONK staking

use solana_sdk::pubkey::Pubkey;
use spl_associated_token_account::get_associated_token_address;

use crate::{BONK_MINT, BONK_STAKE_MINT};

/// Get the user's BONK token account (ATA)
pub fn get_user_bonk_ata(user: &Pubkey) -> Pubkey {
    get_associated_token_address(user, &BONK_MINT)
}

/// Get the user's stake token account (ATA) for the stake mint
pub fn get_user_stake_ata(user: &Pubkey) -> Pubkey {
    get_associated_token_address(user, &BONK_STAKE_MINT)
}

/// Information about a user's stake
#[derive(Debug, Clone)]
pub struct StakeInfo {
    /// The stake deposit receipt address
    pub receipt_address: Pubkey,
    /// The nonce used for this stake
    pub nonce: u32,
    /// Amount of BONK staked (in lamports)
    pub amount: u64,
    /// Lock duration in seconds
    pub lock_duration: u64,
    /// When the stake was created (Unix timestamp)
    pub created_at: i64,
    /// When the stake unlocks (Unix timestamp)
    pub unlock_at: i64,
}

impl StakeInfo {
    /// Check if the stake is currently locked
    pub fn is_locked(&self, current_time: i64) -> bool {
        current_time < self.unlock_at
    }

    /// Get the remaining lock time in seconds
    pub fn remaining_lock_time(&self, current_time: i64) -> i64 {
        (self.unlock_at - current_time).max(0)
    }
}