//! PDA (Program Derived Address) utilities for BONK staking

use solana_sdk::pubkey::Pubkey;

use crate::BONK_STAKE_PROGRAM_ID;

/// Derive the stake deposit receipt PDA
///
/// The correct seed structure from the spl-token-staking program is:
/// - owner pubkey
/// - stake_pool pubkey
/// - nonce as u32 little-endian bytes
/// - "stakeDepositReceipt" (camelCase)
///
/// # Arguments
/// * `owner` - The owner/user public key
/// * `stake_pool` - The stake pool public key
/// * `nonce` - The nonce for this stake
///
/// # Returns
/// A tuple of (PDA address, bump seed)
pub fn derive_stake_deposit_receipt(
    owner: &Pubkey,
    stake_pool: &Pubkey,
    nonce: u32,
) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[
            owner.as_ref(),
            stake_pool.as_ref(),
            &nonce.to_le_bytes(),
            b"stakeDepositReceipt", // camelCase, not snake_case!
        ],
        &BONK_STAKE_PROGRAM_ID,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_stake_deposit_receipt_derivation() {
        let owner = Pubkey::from_str("6tBou5MHL5aWpDy6cgf3wiwGGK2mR8qs68ujtpaoWrf2").unwrap();
        let stake_pool = Pubkey::from_str("9AdEE8AAm1XgJrPEs4zkTPozr3o4U5iGbgvPwkNdLDJ3").unwrap();
        
        // Test known working receipt from mainnet
        let (receipt1, _) = derive_stake_deposit_receipt(&owner, &stake_pool, 1);
        assert_eq!(
            receipt1.to_string(),
            "7ACZ6QNW4sR3v8ooQzvUrr4ZZ13wg4Dj4ouQSdEknWhj"
        );

        let (receipt2, _) = derive_stake_deposit_receipt(&owner, &stake_pool, 2);
        assert_eq!(
            receipt2.to_string(),
            "Do2sHbcqswaLupdvjGiTZHh4U9GB3xF3HztsZoeLBmHh"
        );
    }
}