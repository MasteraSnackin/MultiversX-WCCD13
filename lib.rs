// Import necessary macros and components from the elrond_wasm crate.
// This macro is crucial for setting up the environment for writing smart contracts on the MultiversX blockchain.
elrond_wasm::imports!();

// Declare the `contract` module, which contains the core logic for staking and reward distribution.
// This separation allows for cleaner organization, keeping the interface and implementation distinct.
mod contract;

// Define the main trait for the smart contract using the `elrond_wasm::contract` attribute macro.
// This trait specifies the contract's public interface, including its initialization, endpoints, and view functions.
#[elrond_wasm::contract]
pub trait WinterTokenStaking {
    // The `init` function is a special function that is called once during contract deployment.
    // It is used to initialize the contract's state, such as setting the SNOW token identifier.
    #[init]
    fn init(&self, snow_token_id: TokenIdentifier) {
        // Set the SNOW token identifier in the contract's storage.
        // This identifier will be used to mint SNOW tokens as rewards for stakers.
        self.snow_token_id().set(&snow_token_id);
    }

    // Define an endpoint for staking WINTER tokens.
    // Endpoints are functions that users or other contracts can call to interact with this contract.
    #[endpoint(stakeTokenWinter)]
    fn stake_token_winter(
        &self, 
        // The `#[payment_token]` attribute indicates that this parameter should match the token being sent with the transaction.
        #[payment_token] token: TokenIdentifier, 
        // The `#[payment_amount]` attribute ensures that this parameter corresponds to the amount of tokens being transferred.
        #[payment_amount] amount: BigUint
    ) {
        // Delegate the staking logic to the `stake_token_winter` function in the `contract` module.
        // This keeps the trait focused on defining the interface and delegates implementation details to the module.
        contract::stake_token_winter(self, token, amount);
    }

    // Define an endpoint for claiming rewards.
    // This endpoint allows users to claim their SNOW token rewards based on their staked amounts.
    #[endpoint(claimRewards)]
    fn claim_rewards(&self) {
        // Delegate the reward claiming logic to the `claim_rewards` function in the `contract` module.
        contract::claim_rewards(self);
    }

    // Define an endpoint to transfer rewards to another account.
    // This allows users to send their SNOW token rewards to a specified recipient.
    #[endpoint(transferRewards)]
    fn transfer_rewards(&self, recipient: ManagedAddress) {
        // Delegate the reward transfer logic to the `transfer_rewards` function in the `contract` module.
        contract::transfer_rewards(self, recipient);
    }

    // Define a view function to retrieve the staked tokens for a specific address.
    // View functions are read-only and do not modify the contract's state. They are used to query data.
    #[view(getStakedTokens)]
    fn get_staked_tokens(
        &self, 
        // The address parameter specifies the account whose staked tokens are being queried.
        address: ManagedAddress
    ) -> contract::StakedTokens<Self::Api> {
        // Delegate the logic to retrieve staked tokens to the `get_staked_tokens function in the `contract` module.
        contract::get_staked_tokens(self, address)
    }
}