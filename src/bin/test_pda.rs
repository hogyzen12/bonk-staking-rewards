// Comprehensive PDA derivation test
use solana_sdk::pubkey::Pubkey;
use std::str::FromStr;

const STAKE_PROGRAM_ID: &str = "STAKEkKzbdeKkqzKpLkNQD3SUuLgshDKCD7U8duxAbB";
const BONK_STAKE_POOL: &str = "9AdEE8AAm1XgJrPEs4zkTPozr3o4U5iGbgvPwkNdLDJ3";
const OWNER: &str = "6tBou5MHL5aWpDy6cgf3wiwGGK2mR8qs68ujtpaoWrf2";

// Known working receipts
const KNOWN_RECEIPT_NONCE_1: &str = "7ACZ6QNW4sR3v8ooQzvUrr4ZZ13wg4Dj4ouQSdEknWhj";
const KNOWN_RECEIPT_NONCE_2: &str = "Do2sHbcqswaLupdvjGiTZHh4U9GB3xF3HztsZoeLBmHh";
const EXPECTED_NONCE_0: &str = "FNHvqdzkoZwsPVhpeivksQjhVSDTWPuTjUqHpXW8Z5G2";

fn test_seeds(label: &str, seeds: &[&[u8]], program_id: &Pubkey, expected: &Pubkey) -> bool {
    let (pda, _bump) = Pubkey::find_program_address(seeds, program_id);
    let matches = pda == *expected;
    if matches {
        println!("✓ MATCH FOUND: {}", label);
        println!("  PDA: {}", pda);
        println!("  Seeds: {:?}", seeds.iter().map(|s| 
            if s.len() <= 8 { format!("{:?}", s) } else { format!("Pubkey({}...)", &s[0..4].iter().map(|b| format!("{:02x}", b)).collect::<String>()) }
        ).collect::<Vec<_>>());
    }
    matches
}

fn main() {
    let program_id = Pubkey::from_str(STAKE_PROGRAM_ID).unwrap();
    let stake_pool = Pubkey::from_str(BONK_STAKE_POOL).unwrap();
    let owner = Pubkey::from_str(OWNER).unwrap();
    let expected_1 = Pubkey::from_str(KNOWN_RECEIPT_NONCE_1).unwrap();
    let expected_2 = Pubkey::from_str(KNOWN_RECEIPT_NONCE_2).unwrap();
    let expected_0 = Pubkey::from_str(EXPECTED_NONCE_0).unwrap();
    
    println!("=== Searching for correct PDA derivation ===\n");
    println!("Target receipts:");
    println!("  Nonce 0: {}", expected_0);
    println!("  Nonce 1: {}", expected_1);
    println!("  Nonce 2: {}", expected_2);
    println!();
    
    let prefixes = [
        b"stake_deposit_receipt".as_ref(),
        b"deposit_receipt".as_ref(),
        b"receipt".as_ref(),
        b"stake_receipt".as_ref(),
        b"deposit".as_ref(),
    ];
    
    // Test different nonce encodings
    let nonce_0_u8 = 0u8.to_le_bytes();
    let nonce_1_u8 = 1u8.to_le_bytes();
    let nonce_2_u8 = 2u8.to_le_bytes();
    
    let nonce_0_u16 = 0u16.to_le_bytes();
    let nonce_1_u16 = 1u16.to_le_bytes();
    let nonce_2_u16 = 2u16.to_le_bytes();
    
    let nonce_0_u32_le = 0u32.to_le_bytes();
    let nonce_1_u32_le = 1u32.to_le_bytes();
    let nonce_2_u32_le = 2u32.to_le_bytes();
    
    let nonce_0_u32_be = 0u32.to_be_bytes();
    let nonce_1_u32_be = 1u32.to_be_bytes();
    let nonce_2_u32_be = 2u32.to_be_bytes();
    
    let nonce_encodings = vec![
        ("u8", vec![&nonce_0_u8[..], &nonce_1_u8[..], &nonce_2_u8[..]]),
        ("u16_le", vec![&nonce_0_u16[..], &nonce_1_u16[..], &nonce_2_u16[..]]),
        ("u32_le", vec![&nonce_0_u32_le[..], &nonce_1_u32_le[..], &nonce_2_u32_le[..]]),
        ("u32_be", vec![&nonce_0_u32_be[..], &nonce_1_u32_be[..], &nonce_2_u32_be[..]]),
    ];
    
    let mut found = false;
    
    for prefix in &prefixes {
        for (encoding_name, nonces) in &nonce_encodings {
            // Test: prefix + owner + stake_pool + nonce
            let label = format!("{:?} + owner + stake_pool + {}", 
                String::from_utf8_lossy(prefix), encoding_name);
            if test_seeds(&label, &[prefix, owner.as_ref(), stake_pool.as_ref(), nonces[1]], &program_id, &expected_1) {
                if test_seeds(&label, &[prefix, owner.as_ref(), stake_pool.as_ref(), nonces[2]], &program_id, &expected_2) {
                    found = true;
                    break;
                }
            }
            
            // Test: prefix + stake_pool + owner + nonce
            let label = format!("{:?} + stake_pool + owner + {}", 
                String::from_utf8_lossy(prefix), encoding_name);
            if test_seeds(&label, &[prefix, stake_pool.as_ref(), owner.as_ref(), nonces[1]], &program_id, &expected_1) {
                if test_seeds(&label, &[prefix, stake_pool.as_ref(), owner.as_ref(), nonces[2]], &program_id, &expected_2) {
                    found = true;
                    break;
                }
            }
            
            // Test: prefix + owner + nonce (no stake pool)
            let label = format!("{:?} + owner + {}", 
                String::from_utf8_lossy(prefix), encoding_name);
            if test_seeds(&label, &[prefix, owner.as_ref(), nonces[1]], &program_id, &expected_1) {
                if test_seeds(&label, &[prefix, owner.as_ref(), nonces[2]], &program_id, &expected_2) {
                    found = true;
                    break;
                }
            }
            
            // Test: prefix + nonce + owner
            let label = format!("{:?} + nonce + owner ({})", 
                String::from_utf8_lossy(prefix), encoding_name);
            if test_seeds(&label, &[prefix, nonces[1], owner.as_ref()], &program_id, &expected_1) {
                if test_seeds(&label, &[prefix, nonces[2], owner.as_ref()], &program_id, &expected_2) {
                    found = true;
                    break;
                }
            }
        }
        if found { break; }
    }
    
    if !found {
        println!("\n⚠ No matching seed pattern found!");
        println!("The PDA might use a different structure or encoding.");
    }
}