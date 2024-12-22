WINTER Token Staking Smart Contract
This project implements a staking smart contract for WINTER tokens on the MultiversX blockchain. It features an automated reward distribution system that mints SNOW tokens as rewards for stakers. The contract is written in Rust and compiles to WebAssembly (Wasm) for deployment.

Features
Stake WINTER Tokens: Users can stake their WINTER tokens and earn rewards.
Automated Rewards: The contract mints 1% of the staked tokens as SNOW rewards every 24 hours.
Claim Rewards: Stakers can claim their SNOW token rewards every 24 hours.
Transfer Rewards: Users can transfer their SNOW token rewards to another account.
SNOW Tokens: Reward tokens have 8 decimals and are minted by the contract.
Project Structure
Copy
winter-staking/
│
├── Cargo.toml
├── src/
│   ├── lib.rs
│   └── contract.rs
├── README.md
├── LICENSE
└── backend/
    ├── Cargo.toml
    └── src/
        └── main.rs
Cargo.toml: Manages Rust project dependencies and metadata for the smart contract.
src/lib.rs: Defines the public interface of the smart contract.
src/contract.rs: Contains the core logic for staking and reward distribution.
README.md: This file, providing an overview and instructions for the project.
LICENSE: The project's open-source license.
backend/: Contains the backend service that interacts with the MultiversX API.
Setup
Prerequisites
Rust and Wasm Toolchain: Install Rust and add the wasm32-unknown-unknown target for compiling to WebAssembly.
MultiversX SDK: Required for deploying and interacting with the blockchain.
Installation
Clone the Repository:

git clone https://github.com/yourusername/winter-staking.git
cd winter-staking
Add Wasm Target:

rustup target add wasm32-unknown-unknown
Build the Contract:

cargo build --target wasm32-unknown-unknown --release
Contract Overview
Code Explanation
lib.rs
Trait Definition: The WinterTokenStaking trait defines endpoints for staking, claiming rewards, transferring rewards, and viewing staked tokens.
Initialization: The init function sets the SNOW token identifier for minting rewards.
Endpoints:
stake_token_winter: Allows users to stake WINTER tokens.
claim_rewards: Enables users to claim their SNOW token rewards.
transfer_rewards: Allows users to transfer SNOW rewards to another account.
View Function: get_staked_tokens allows querying of staked token details.
contract.rs
StakedTokens Structure: Tracks each user's staked tokens, lock duration, and last reward claim.
Staking Logic: Validates token type and amount, updates staking data.
Reward Logic: Calculates and mints SNOW rewards, ensuring rewards can be claimed or transferred every 24 hours.
Backend Service (Off-Chain)
The backend service interacts with the MultiversX API to fetch data and potentially send transactions to the blockchain.

backend/main.rs
HTTP Client: Uses reqwest to make HTTP requests to the MultiversX API.
Data Fetching: Fetches account details and network economics as examples.
Asynchronous Execution: Utilizes the Tokio runtime for asynchronous operations.
Interacting with the Contract
Deploy the Contract: Use the MultiversX SDK to deploy the compiled .wasm file to the blockchain.
Stake Tokens: Call the stake_token_winter endpoint with the appropriate token and amount.
Claim Rewards: Use the claim_rewards endpoint after 24 hours to collect SNOW rewards.
Transfer Rewards: Use the transfer_rewards endpoint to send SNOW rewards to another account.
Future Enhancements
Advanced Reward Mechanisms: Implement more complex reward calculations and distribution strategies.
Withdrawal Functionality: Allow users to withdraw their staked tokens post-lock period.
Security Improvements: Enhance security measures to protect against potential vulnerabilities.
