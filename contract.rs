// Import essential types and traits from the elrond_wasm crate.
// These imports are necessary for working with large numbers, tokens, addresses, and contract storage.
use elrond_wasm::types::{BigUint, TokenIdentifier, ManagedAddress};
use elrond_wasm::storage::mappers::SingleValueMapper;
use elrond_wasm::api::ManagedTypeApi;

// Define a structure to represent the staked tokens of a user.
// This structure includes fields to track the amount of tokens staked, the lock period, and the last epoch when rewards were claimed.
pub struct StakedTokens<M: ManagedTypeApi> {
    pub amount: BigUint<M>,          // The total amount of tokens staked by the user.
    pub lock_until_epoch: u64,       // The epoch until which the tokens are locked.
    pub last_claimed_epoch: u64,     // The last epoch when the user claimed their rewards.
}

// Implement the logic for staking tokens.
// This function is called when a user interacts with the contract to stake their WINTER tokens.
pub fn stake_token_winter<M: ManagedTypeApi>(
    contract: &impl WinterTokenStaking<M>, // A reference to the contract implementation.
    token: TokenIdentifier,                // The identifier of the token being staked.
    amount: BigUint<M>,                    // The amount of tokens to be staked.
) {
    // Get the caller's address (the account initiating the transaction).
    let caller = contract.blockchain().get_caller();
    // Get the current epoch (a unit of time on the blockchain).
    let current_epoch = contract.blockchain().get_block_epoch();

    // Validate that the correct token is being staked and that the staking amount is greater than zero.
    require!(token == contract.winter_token_id().get(), "Invalid token");
    require!(amount > 0, "Staking amount must be greater than zero");

    // Calculate the epoch until which the tokens will be locked.
    let lock_until_epoch = current_epoch + contract.min_staking_epochs().get();

    // Update the user's staked token data in the contract's storage.
    contract.staked_tokens(&caller).update(|staked| {
        staked.amount += amount;                // Increase the staked amount by the new amount.
        staked.lock_until_epoch = lock_until_epoch; // Set the lock expiration epoch.
        staked.last_claimed_epoch = current_epoch; // Initialize the last claimed epoch to the current epoch.
    });
}

// Implement the logic for claiming rewards.
// This function allows users to claim their rewards, which are distributed based on the amount of tokens they have staked.
pub fn claim_rewards<M: ManagedTypeApi>(contract: &impl WinterTokenStaking<M>) {
    // Get the caller's address and the current epoch from the blockchain.
    let caller = contract.blockchain().get_caller();
    let current_epoch = contract.blockchain().get_block_epoch();
    let reward_epoch_interval = 1; // Define the interval for claiming rewards as 1 epoch (24 hours).

    // Update the user's staked token data to account for claimed rewards.
    contract.staked_tokens(&caller).update(|staked| {
        // Calculate the number of epochs since the last reward claim.
        let epochs_since_last_claim = current_epoch - staked.last_claimed_epoch;
        
        // Ensure that enough time has passed to claim rewards.
        require!(epochs_since_last_claim >= reward_epoch_interval, "Rewards can only be claimed every 24 hours");

        // Calculate the reward amount as 1% of the staked amount.
        let reward_amount = &staked.amount / BigUint::from(100u64);
        
        // Update the last claimed epoch to the current epoch.
        staked.last_claimed_epoch = current_epoch;

        // Mint SNOW tokens as rewards for the user.
        contract.mint_reward_tokens(&caller, reward_amount);
    });
}

// Implement the logic for transferring rewards to another account.
// This function allows users to transfer their reward tokens to a specified recipient.
pub fn transfer_rewards<M: ManagedTypeApi>(
    contract: &impl WinterTokenStaking<M>, // A reference to the contract implementation.
    recipient: ManagedAddress,             // The address of the recipient to whom the rewards will be transferred.
) {
    // Get the caller's address and the current epoch from the blockchain.
    let caller = contract.blockchain().get_caller();
    let current_epoch = contract.blockchain().get_block_epoch();
    let reward_epoch_interval = 1; // Define the interval for transferring rewards as 1 epoch (24 hours).

    // Update the user's staked token data to account for transferred rewards.
    contract.staked_tokens(&caller).update(|staked| {
        // Calculate the number of epochs since the last reward claim or transfer.
        let epochs_since_last_claim = current_epoch - staked.last_claimed_epoch;
        
        // Ensure that enough time has passed to transfer rewards.
        require!(epochs_since_last_claim >= reward_epoch_interval, "Rewards can only be transferred every 24 hours");

        // Calculate the reward amount as 1% of the staked amount.
        let reward_amount = &staked.amount / BigUint::from(100u64);
        
        // Update the last claimed epoch to the current epoch.
        staked.last_claimed_epoch = current_epoch;

        // Mint SNOW tokens as rewards and transfer them to the recipient.
        contract.mint_reward_tokens(&recipient, reward_amount);
    });
}

// Define a trait that outlines the necessary methods and associated types for the staking contract.
// This trait serves as an interface for the contract, ensuring it implements essential functionality.
pub trait WinterTokenStaking<M: ManagedTypeApi> {
    // Method to access blockchain-related functionalities, such as getting the caller and current epoch.
    fn blockchain(&self) -> &dyn elrond_wasm::api::BlockchainApi<M>;

    // Method to access the storage mapper for a user's staked tokens.
    fn staked_tokens(&self, address: &ManagedAddress) -> SingleValueMapper<StakedTokens<M>>;

    // Method to access the storage mapper for the token identifier used in staking.
    fn winter_token_id(&self) -> SingleValueMapper<TokenIdentifier>;

    // Method to access the storage mapper for the minimum number of epochs tokens should be staked.
    fn min_staking_epochs(&self) -> SingleValueMapper<u64>;

    // Method to access the storage mapper for the reward token identifier (SNOW tokens).
    fn snow_token_id(&self) -> SingleValueMapper<TokenIdentifier>;

    // Function to mint reward tokens and send them to the specified address.
    fn mint_reward_tokens(&self, to: &ManagedAddress, amount: BigUint<M>) {
        // Retrieve the SNOW token identifier from storage.
        let token_id = self.snow_token_id().get();

        // Mint and send the reward tokens to the user's address.
        self.send().direct(to, &token_id, 0, &amount, &[]);
    }
}