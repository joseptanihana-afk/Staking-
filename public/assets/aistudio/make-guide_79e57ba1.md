# Make Guide

This guide covers building, testing, packaging, and deploying the **two separate** Solana staking packages:

| Repository | Package | On-chain? |
|---|---|---|
| `solana-staking-protect` | `solana-staking-protect` | Yes - on-chain staking program |
| `solana-staking-simulation` | `solana-staking-simulation` | Yes - real on-chain 100% protection protocol |

Each repository is self-contained and deployable independently.

---

## Prerequisites

- Rust toolchain (edition 2021)
- Solana toolchain / `solana-program-test` and `solana-sdk` dev-dependencies
- `cargo` and `rustup` with the `bpfel-unknown-unknown` target (for compiling to BPF)

```bash
rustup target add bpfel-unknown-unknown
```

---

## 1. Build the on-chain staking program (`solana-staking-protect`)

```bash
cd solana-staking-protect
cargo build --release --target bpfel-unknown-unknown
```

or with the legacy Solana toolchain:

```bash
cargo build-bpf --release
```

Output:

```text
target/bpfel-unknown-unknown/release/libsolana_staking_protect.so
```

---

## 2. Build the real on-chain protection protocol (`solana-staking-simulation`)

```bash
cd solana-staking-simulation
cargo build --release --target bpfel-unknown-unknown
```

or:

```bash
cargo build-bpf --release
```

Output:

```text
target/bpfel-unknown-unknown/release/libsolana_staking_simulation.so
```

---

## 3. Run tests

```bash
cargo test --release
```

Run the tests for a specific package:

```bash
cd solana-staking-protect
cargo test

cd ../solana-staking-simulation
cargo test
```

---

## 4. Package and release

Build the release artifacts:

```bash
cd solana-staking-protect
cargo build --release --target bpfel-unknown-unknown

cd ../solana-staking-simulation
cargo build --release --target bpfel-unknown-unknown
```

Verify the produced binaries:

```bash
ls target/bpfel-unknown-unknown/release/
```

---

## 5. Install as a dependency

### From another project's `Cargo.toml`

#### Install `solana-staking-protect`

```toml
[dependencies]
solana-staking-protect = { path = "../solana-staking-protect" }
```

Or from git:

```toml
[dependencies]
solana-staking-protect = { git = "https://github.com/solana-staking-protect/solana-staking-protect", package = "solana-staking-protect" }
```

#### Install `solana-staking-simulation`

```toml
[dependencies]
solana-staking-simulation = { path = "../solana-staking-simulation" }
```

Or from git:

```toml
[dependencies]
solana-staking-simulation = { git = "https://github.com/solana-staking-protect/solana-staking-protect", package = "solana-staking-simulation" }
```

---

## 6. Deploy to a blockchain

1. Build the release artifact:
   ```bash
   cargo build --release --target bpfel-unknown-unknown
   ```
2. Deploy the `cdylib` account with your Solana CLI or Anchor tooling.
3. Verify the deployed program ID matches the compiled program.

Both packages compile to `cdylib` and are BPF-compatible with Solana mainnet/devnet.

---

## Environment variables / config

Add any deployment-specific configuration through your own project:

```bash
SECRETS_DIR=${SECRETS_DIR:-./secrets}
SOLANA_RPC_URL=${SOLANA_RPC_URL:-https://api.mainnet-beta.solana.com}
```

---

## Cleanup

Remove the old local zip artifact if it is no longer needed:

```bash
rm -f solana-staking-protect.zip
```

---

## References

- `README.md` - package overview and install instructions
- `solana-staking-protect/README.md` - on-chain staking program documentation
- `solana-staking-simulation/README.md` - real on-chain protection protocol documentation
