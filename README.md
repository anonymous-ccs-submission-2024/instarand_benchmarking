# InstaRand Benchmarking
Library benchmarking VRF, dVRF, FlexiRand, and InstaRand.
Artifact submission for CCS 2024.
Contains off-chain benchmarking written in Rust and on-chain benchmarking written in Solidity.
## Rust
#### Rust Benchmarks Overview
- Goldberg VRF on Secp256k1 Curve
- GLOW dVRF on BN254 Curve
- FlexiRand on BN254 Curve
- InstaRand Client using Goldberg VRF on Secp2566k1 Curve
- InstaRand Centralized Server using BLS VRF on BN254 Curve
- InstaRand Decentralized Server using GLOW dVRF on BN254 Curve
#### Rust Version / Package Details Details
- Cargo (Rust) version 1.75.0
- [k256](https://crates.io/crates/k256) version 0.13.3 used for Secp256k1 Curve
- [substrate-bn](https://crates.io/crates/substrate-bn) version 0.6.0 was used for BN254 Curve
- [criterion](https://crates.io/crates/criterion/0.5.1/dependencies) version 0.4.0 was used for benchmarking
  - each benchmark consisted of at least 20 repetitions
  - experiments can be run with `cargo bench`
  - all experiments have R<sup>2</sup> >= 0.999
  - [`cargo bench` CLI Output](./data/rust/cli_benchmark_output.pdf)
  - [Full Report](./data/rust/report.zip)
#### Machine Information
- Model: MacBook Pro 18.3
- Chip: Apple M1 Pro
- Cores: 8 (6 performance and 2 efficiency)
- Memory: 16 GB
- System Firmware Version: 8422.141.2
- OS Loader Version: 8422.141.2
- OS: macOS Ventura 13.5.1
- Additional Information:
  - Wi-Fi and Bluetooth disabled
  - All other processes closed
  - Screen reduced to minimum brightness
  - No external devices connected (mouse, monitor, etc.)
#### Build Instructions
- install [Rust and Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html) minimum version 1.72.1
- open a terminal in [`./off-chain/`](./off-chain/) (or enter directory using `cd off-chain`)
- run `cargo bench`
## Contracts
#### Smart Contracts Overview ([`./on-chain/contracts/`](./on-chain/contracts/))
- [`BLS.sol`](./on-chain/contracts/BLS.sol) and [`DDH.sol`](./on-chain/contracts/DDH.sol) contain implementations for verification of BLS Signatures on BN254 and Goldberg VRF on Secp256k1 respectively.
- They contain the Supra and Chainlink implementations respectively, with any modifications denoted by a `FLAG_TODO` comment which can be easily parsed.
  - `BLS.sol` was taken from the [Supra dVRF Smart contract](https://etherscan.io/address/0xaeef3c744e07b4ceeb7469460f220c697b8fb8bc#code) currently deployed on ethereum mainnet.
  - `DDH.sol` was taken from [Chainlink's Github](https://github.com/smartcontractkit/chainlink/blob/develop/contracts/src/v0.8/vrf/VRF.sol) rather than the [Chainlink VRF Smart Contract](https://etherscan.io/address/0xf0d54349aDdcf704F77AE15b96510dEA15cb7952#code#L275) that is currently deployed on ethereum mainnet because it compiles with the same solidity compiler versions as `BLS.sol`
- [`vrf.sol`](./on-chain/contracts/vrf.sol), [`dvrf.sol`](./on-chain/contracts/dvrf.sol), [`flexirand.sol`](./on-chain/contracts/flexirand.sol), and [`instarand.sol`](./on-chain/contracts/instarand.sol) contain smart contracts which implement their respective protocols in their entirety.
- [`crypto_operations.sol`](./on-chain/contracts/crypto_operations.sol) and [`storage_operations.sol`](./on-chain/contracts/storage_operations.sol) contain code for benchmarking the gas used to execute individual operations such as signature verification and storing data on-chain.
#### Software Details
- Compiled using Solidity Compiler 0.8.19
- Deployed and run on Remix VM (London) to enable using javascript to interact with SC's
- Gas costs should approximate ethereum mainnet
#### Build Instructions
- Open [Remix IDE](https://remix.ethereum.org/)
- clear default contents of `contracts` and `scripts` folders
- copy contents of [`./on-chain/contracts/`](./on-chain/contracts/) to Remix IDE `./contracts`. Do the same for [`./on-chain/scripts/`](./on-chain/scripts/).
- Set compiler version to `0.8.19` and compile each contract in `./contracts/` other than `DDH.sol` and `BLS.sol` (these will compile automatically as dependencies)
- Set Environment to `Remix EVM (London)` and deploy each SC.
- copy the address of each deployed SC in `./scripts/addresses.json`
- run each script by right-clicking on it and clicking `run`.
#### Gas Measurement
- [`~/transcripts/`](./data/on-chain/transcripts/) contains the raw output from running the experiments
  - Due to the constraints of Remix IDE we ran experiments in 10 batches of 10.
  - We always begin with a single batch of 1 to initialize any on-chain variables. This batch is not counted towards our data as it is more expensive than any other round.
- storage_operations was benched twice since constant csot
- [`results.xlsx`](./data/on-chain/results.xlsx) contains data which has been parsed and made more presentable.
  - The left half of the sheet contains final numbers and the right side contains the average from each of the 10 runs.