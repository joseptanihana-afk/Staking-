# Make Guide - solana-staking-protect

Build, test, package, and deploy the on-chain staking program.

## Prerequisites

```bash
rustup target add bpfel-unknown-unknown
```

## Build (BPF)

```bash
cargo build --release --target bpfel-unknown-unknown
# or
cargo build-bpf --release
```

## Test

```bash
cargo test --release
```

## Package and release

```bash
cargo build --release --target bpfel-unknown-unknown
ls target/bpfel-unknown-unknown/release/
```

## Install as a dependency

In your Solana program's `Cargo.toml`:

```toml
[dependencies]
solana-staking-protect = { path = "../solana-staking-protect" }
```

## Deploy to a blockchain

1. Build the release artifact.
2. Deploy the `cdylib` account with your Solana CLI or Anchor tooling.
3. Verify the deployed program ID matches the compiled program.
