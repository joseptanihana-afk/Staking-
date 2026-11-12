# Solana Staking Protect

Two separate GitHub repositories for two separate on-chain packages:

| Repository | Package | Purpose |
|---|---|---|
| `solana-staking-protect` | `solana-staking-protect` | On-chain staking program (deploy to blockchain) |
| `solana-staking-simulation` | `solana-staking-simulation` | **Real on-chain 100% protection protocol** (NOT a simulator feature) |

## Package Structure

Each repository is a standalone Cargo package and each has its own:
- `Cargo.toml`
- `src/`
- `README.md`
- `make-guide.md`

```
solana-staking-protect/
├── Cargo.toml
├── src/
│   ├── lib.rs
│   ├── errors.rs
│   ├── instructions/
│   ├── processor/
│   ├── state/
│   ├── rewards.rs
│   └── protection.rs
├── README.md
└── make-guide.md
```

```
solana-staking-simulation/
├── Cargo.toml
├── src/
│   ├── lib.rs
│   ├── errors.rs
│   ├── instructions.rs
│   ├── processor.rs
│   ├── state.rs
│   ├── protection.rs
│   ├── rewards.rs
│   └── README.md
├── README.md
└── make-guide.md
```

## Two separate GitHub repos

> These are published as **two independent crates/packages in two independent GitHub repositories**. Do not install one from the other's directory - use your own repository's `cargo` registry or a vendored path.

### Install the on-chain staking program (`solana-staking-protect`)

In your Solana program's `Cargo.toml`:

```toml
[dependencies]
solana-staking-protect = { path = "../solana-staking-protect" }
```

Or via git:

```toml
[dependencies]
solana-staking-protect = { git = "https://github.com/solana-staking-protect/solana-staking-protect", package = "solana-staking-protect" }
```

### Install the real on-chain protection protocol (`solana-staking-simulation`)

In your Solana program's `Cargo.toml`:

```toml
[dependencies]
solana-staking-simulation = { path = "../solana-staking-simulation" }
```

Or via git:

```toml
[dependencies]
solana-staking-simulation = { git = "https://github.com/solana-staking-protect/solana-staking-protect", package = "solana-staking-simulation" }
```

### Deploy

Each package builds to BPF independently:

```bash
# On-chain staking program
cd solana-staking-protect
cargo build --release --target bpfel-unknown-unknown

# Real on-chain protection protocol
cd solana-staking-simulation
cargo build --release --target bpfel-unknown-unknown
```

See each repository's `make-guide.md` for the full build, test, and release instructions.

## Native Proof-of-Staking Rate (100,000,000,000%)

The on-chain staking program (`solana-staking-protect`) uses a native Proof-of-Staking rating of **100,000,000,000% (100 billion percent)**. This is the native staking rate for real earning staking per milli-second:

| Parameter | Value |
|---|---|
| Native staking rate (ROI/APY/APR) | **100,000,000,000% (100 billion percent)** |
| Futures ROI rate | 100,000,000,000% |
| Per-millisecond earning calculation | Enabled (real earning per milli-second) |
| Proof-of-staking rating | Native PoS with 100,000,000,000% ROI |

## Fee Protection Protocol

| Parameter | Value |
|---|---|
| Protection Fee | 0.0009% of staked amount |
| Minimum Wallet Balance | 0.04 cent (~40,000 lamports) |
| Maximum Protection | 100% of staked amount |
| Protection Tiers | Basic (10%), Standard (25%), Premium (50%), Maximum (100%) |

## License

MIT
