# Bonk Staking Rewards 🎯

A composable Rust crate for interacting with the Bonk Stake Program on Solana.

## Features

- ✨ Simple API for staking BONK tokens
- 🔧 Fully composable - use as a library in your Rust apps
- 🚀 CLI tool for quick staking operations
- 📦 Compatible with your existing Dioxus app dependencies

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
bonk-staking-rewards = "0.1.0"
```

## Quick Start

### As a CLI Tool

```bash
# Build the project
cargo build --release

# Run the staking tool
cargo run --bin stake-bonk

# Or with logging
RUST_LOG=info cargo run --bin stake-bonk
```

### As a Library

```rust
use bonk_staking_rewards::{build_deposit_transaction, StakeConfig};
use solana_client::rpc_client::RpcClient;
use solana_sdk::signature::Keypair;

// Configure your stake
let config = StakeConfig {
    amount: 10_000_000,      // 100 BONK (5 decimals)
    lockup_duration: 7_776_000, // 90 days
    nonce: 0,
};

// Build and send transaction
let client = RpcClient::new("https://api.mainnet-beta.solana.com".to_string());
let payer = Keypair::new(); // Your keypair
let recent_blockhash = client.get_latest_blockhash()?;
let tx = build_deposit_transaction(&payer, &config, recent_blockhash)?;
let signature = client.send_and_confirm_transaction(&tx)?;
```

## Configuration

The Bonk Staking Program uses these constants:
- **Program ID**: `STAKEkKzbdeKkqzKpLkNQD3SUuLgshDKCD7U8duxAbB`
- **Stake Pool**: `BcKdLUv1xJ3hFyZptPiMoN5bC5c3bpFqZPNAv9F4mUg8`

Configure your own keypair and RPC URL when using the library in your application.

## API Reference

### StakeConfig

Configure your staking parameters:

```rust
pub struct StakeConfig {
    pub amount: u64,           // Amount in smallest units (5 decimals for BONK)
    pub lockup_duration: u64,  // Duration in seconds
    pub nonce: u32,           // Nonce for multiple stakes
}
```

### Functions

#### `build_deposit_instruction`

Build a raw deposit instruction for the staking program.

```rust
pub fn build_deposit_instruction(
    payer: &Pubkey,
    owner: &Pubkey,
    from_token_account: &Pubkey,
    config: &StakeConfig,
) -> Result<Instruction, StakingError>
```

#### `build_deposit_transaction`

Build a complete transaction with compute budget and deposit instruction.

```rust
pub fn build_deposit_transaction(
    payer: &Keypair,
    config: &StakeConfig,
    recent_blockhash: Hash,
) -> Result<Transaction, StakingError>
```

#### PDA Derivation Helpers

```rust
// Derive stake deposit receipt PDA
pub fn derive_stake_deposit_receipt(
    owner: &Pubkey,
    stake_pool: &Pubkey,
    nonce: u32,
    program_id: &Pubkey,
) -> (Pubkey, u8)

// Derive vault PDA
pub fn derive_vault(
    stake_pool: &Pubkey,
    program_id: &Pubkey,
) -> (Pubkey, u8)

// Derive stake mint PDA
pub fn derive_stake_mint(
    stake_pool: &Pubkey,
    program_id: &Pubkey,
) -> (Pubkey, u8)
```

## Examples

### Stake 100 BONK for 90 days

```rust
let config = StakeConfig {
    amount: 10_000_000,      // 100 BONK
    lockup_duration: 7_776_000, // 90 days
    nonce: 0,
};
```

### Stake 1000 BONK for 180 days

```rust
let config = StakeConfig {
    amount: 100_000_000,     // 1000 BONK
    lockup_duration: 15_552_000, // 180 days
    nonce: 0,
};
```

### Multiple stakes with different nonces

```rust
// First stake
let config1 = StakeConfig {
    amount: 10_000_000,
    lockup_duration: 7_776_000,
    nonce: 0,  // First deposit
};

// Second stake
let config2 = StakeConfig {
    amount: 20_000_000,
    lockup_duration: 15_552_000,
    nonce: 1,  // Second deposit
};
```

## Integration with Your App

You can easily integrate this crate into your Rust application:

```toml
# In your app's Cargo.toml
[dependencies]
bonk-staking-rewards = "0.1.0"

# Compatible with Solana SDK 2.3.x
# solana-sdk = "2.3.1"
# solana-client = "2.3.2"
# spl-token = "8.0.0"
# borsh = "1.5.7"
# etc.
```

Then use it in your Dioxus app:

```rust
use bonk_staking_rewards::{build_deposit_transaction, StakeConfig};

async fn stake_bonk(amount: u64, duration: u64) -> Result<String, Box<dyn Error>> {
    let config = StakeConfig {
        amount,
        lockup_duration: duration,
        nonce: 0,
    };
    
    // Build and send transaction
    // ... your logic here
    
    Ok(signature.to_string())
}
```

## Testing

Run tests:

```bash
cargo test
```

Run with logging:

```bash
RUST_LOG=debug cargo test -- --nocapture
```

## Error Handling

The crate provides a comprehensive `StakingError` enum:

```rust
pub enum StakingError {
    InvalidProgramId,
    InvalidStakePool,
    InvalidMint,
    InvalidRewardVault,
    SerializationError(std::io::Error),
    ClientError(solana_client::client_error::ClientError),
    KeypairError(String),
    InvalidKeypair,
}
```

## Contributing

This crate is designed to be composable and extensible. Feel free to add:
- Withdrawal functionality
- Reward claiming
- Status checking
- Multiple stake pool support

## License

MIT

## Resources

- [Bonk Stake Program](https://solscan.io/account/STAKEkKzbdeKkqzKpLkNQD3SUuLgshDKCD7U8duxAbB)
- [Solana Documentation](https://docs.solana.com/)
- [SPL Token Documentation](https://spl.solana.com/token)

---

Made with 🔥 by hogyzen12