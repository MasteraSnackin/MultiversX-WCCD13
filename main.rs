// Import necessary modules from external crates.
// `reqwest::Client` is used for making HTTP requests.
// `serde_json::Value` is used for parsing JSON responses.
// `std::error::Error` is a trait for error handling.
use reqwest::Client;
use serde_json::Value;
use std::error::Error;

// The main function is marked as asynchronous using the `#[tokio::main]` attribute.
// This allows us to use asynchronous features provided by the Tokio runtime, such as `await`.
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Create a new instance of the HTTP client using `reqwest`.
    // This client will be used to send HTTP requests to the MultiversX API.
    let client = Client::new();
    
    // Example 1: Fetch account details from the MultiversX API.
    // Replace "your-account-address-here" with the actual account address you want to query.
    let address = "your-account-address-here";
    // Construct the URL for the account endpoint using the provided address.
    let url = format!("https://devnet-api.multiversx.com/accounts/{}", address);
    
    // Send a GET request to the constructed URL and await the response.
    // The `?` operator is used for error handling, propagating any errors that occur.
    let response = client.get(&url)
        .send()
        .await?
        // Parse the response body as JSON into a `Value` type using `serde_json`.
        .json::<Value>()
        .await?;
    
    // Print the JSON response containing the account details to the console.
    // The `:?` format specifier is used for pretty-printing the JSON structure.
    println!("Account Details: {:?}", response);
    
    // Example 2: Fetch network economics data from the MultiversX API.
    // Define the URL for the economics endpoint.
    let economics_url = "https://devnet-api.multiversx.com/economics";
    // Send a GET request to the economics endpoint and await the response.
    let economics_response = client.get(economics_url)
        .send()
        .await?
        // Parse the response body as JSON into a `Value` type.
        .json::<Value>()
        .await?;
    
    // Print the JSON response containing network economics data to the console.
    println!("Network Economics: {:?}", economics_response);
    
    // Return an `Ok` result to indicate successful execution.
    Ok(())
}