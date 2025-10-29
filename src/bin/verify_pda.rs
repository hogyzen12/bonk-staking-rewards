// Quick test to verify the PDA derivation matches known working receipts
use solana_sdk::pubkey::Pubkey;
use std::str::FromStr;
use bonk_staking_rewards::derive_stake_deposit_receipt;

const STAKE_PROGRAM_ID: &str = "STAKEkKzbdeKkqzKpLkNQD3SUuLgshDKCD7U8duxAbB";
const BONK_STAKE_POOL: &str = "9AdEE8AAm1XgJrPEs4zkTPozr3o4U5iGbgvPwkNdLDJ3";
const OWNER: &str = "6tBou5MHL5aWpDy6cgf3wiwGGK2mR8qs68ujtpaoWrf2";

// Known working receipts from successful transactions
const KNOWN_RECEIPT_NONCE_1: &str = "7ACZ6QNW4sR3v8ooQzvUrr4ZZ13wg4Dj4ouQSdEknWhj";
const KNOWN_RECEIPT_NONCE_2: &str = "Do2sHbcqswaLupdvjGiTZHh4U9GB3xF3HztsZoeLBmHh";

fn main() {
    let program_id = Pubkey::from_str(STAKE_PROGRAM_ID).unwrap();
    let stake_pool = Pubkey::from_str(BONK_STAKE_POOL).unwrap();
    let owner = Pubkey::from_str(OWNER).unwrap();
    
    let expected_1 = Pubkey::from_str(KNOWN_RECEIPT_NONCE_1).unwrap();
    let expected_2 = Pubkey::from_str(KNOWN_RECEIPT_NONCE_2).unwrap();
    
    println!("=== Verifying PDA Derivation ===\n");
    
    // Test nonce 1
    let (receipt_1, bump_1) = derive_stake_deposit_receipt(&owner, &stake_pool, 1, &program_id);
    println!("Nonce 1:");
    println!("  Expected: {}", expected_1);
    println!("  Derived:  {}", receipt_1);
    println!("  Bump:     {}", bump_1);
    println!("  Match:    {}\n", receipt_1 == expected_1);
    
    // Test nonce 2
    let (receipt_2, bump_2) = derive_stake_deposit_receipt(&owner, &stake_pool, 2, &program_id);
    println!("Nonce 2:");
    println!("  Expected: {}", expected_2);
    println!("  Derived:  {}", receipt_2);
    println!("  Bump:     {}", bump_2);
    println!("  Match:    {}\n", receipt_2 == expected_2);
    
    if receipt_1 == expected_1 && receipt_2 == expected_2 {
        println!("SUCCESS! PDA derivation is correct!");
    } else {
        println!("FAILED! PDA derivation does not match.");
    }
}