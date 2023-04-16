// Import required crates and modules
use base64;
use reqwest::header::{HeaderValue, AUTHORIZATION};
use serde_derive::Deserialize;
use std::env;
use std::fs;

// Define the main function as asynchronous
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Read the Azure DevOps PAT from the environment variable
    let organization = env::var("AZURE_DEVOPS_ORGANIZATION").expect("Set the AZURE_DEVOPS_ORGANIZATION environment variable");
    let project = env::var("AZURE_DEVOPS_PROJECT").expect("Set the AZURE_DEVOPS_PROJECT environment variable");
    let api_version = env::var("AZURE_DEVOPS_API_VERSION").expect("Set the AZURE_DEVOPS_API_VERSION environment variable");
    let pat = env::var("AZURE_DEVOPS_PAT").expect("Set the AZURE_DEVOPS_PAT environment variable");

    // Build the base URL for the Azure DevOps API
    let base_url = format!(
        "https://{}/{}/_apis/git/repositories?api-version={}",
        organization, project, api_version
    );
    println!("Base URL: {}", base_url);

    // Encode the PAT and create the authorization token
    let token = format!("Basic {}", base64::encode(format!("PAT:{}", pat)));

    // Create a new reqwest client
    let client = reqwest::Client::new();

    // Send a GET request to the Azure DevOps API
    let response = client
        .get(&base_url)
        .header(AUTHORIZATION, HeaderValue::from_str(&token)?)
        .send()
        .await?;

    // Get the JSON response text
    let raw_json = response.text().await?;

    // Deserialize the JSON response into AzureReposResponse struct
    let AzureReposResponse { value, .. }: AzureReposResponse = serde_json::from_str(&raw_json)?;

    // Extract the repository ids and names
    let repo_info: Vec<String> = value
        .into_iter()
        .map(|repo| format!("{} {}", repo.id, repo.name))
        .collect();

    // Define the output file name
    let output_file = "repo_info.txt";

    // Write the repository names to the output file, separated by newlines
    fs::write(output_file, repo_info.join("\n"))?;

    // Print the output file name
    println!("Repository info have been saved to: {}", output_file);

    // Return Ok result
    Ok(())
}

// Define a struct for deserializing the Azure DevOps API response
#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct AzureReposResponse {
    count: usize,
    value: Vec<AzureRepo>,
}

// Define a struct for deserializing the individual Azure repositories
#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct AzureRepo {
    id: String,
    name: String,
}
