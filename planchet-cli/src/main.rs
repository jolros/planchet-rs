//! A command-line interface for the `planchet` library.
//!
//! This tool provides commands to fetch and display a user's coin collection
//! data from the Numista API.
//!
//! # Usage
//!
//! The `--api-key` argument can be omitted if the `NUMISTA_API_KEY` environment variable is set.
//!
//! ```bash
//! planchet-cli --api-key <YOUR_API_KEY> --user-id <USER_ID> <COMMAND>
//! ```
//!
//! # Commands
//!
//! ## `dump`
//!
//! Dumps the user's collection to the console, sorted by issuer name, year, and title.
//!
//! ```bash
//! $ planchet-cli --api-key my-secret-key --user-id 123 dump
//! Canada - 5 Cents - Victoria (1858)
//! Canada - 1 Cent - George V (1920)
//! ```
//!
//! ## `summarize`
//!
//! Summarizes the user's collection by issuer, showing the total number of items,
//! the oldest item, and the newest item.
//!
//! ```bash
//! $ planchet-cli --api-key my-secret-key --user-id 123 summarize
//! +--------+-------------+-------------+-------------+
//! | Issuer | Total Items | Oldest Item | Newest Item |
//! +--------+-------------+-------------+-------------+
//! | Canada | 2           | 1858        | 1920        |
//! +--------+-------------+-------------+-------------+
//! ```
use anyhow::Result;
use clap::{Parser, Subcommand};
use planchet::{
    models::{CollectedItem, GrantType},
    Client, ClientBuilder, GetCollectedItemsParams, OAuthTokenParams,
};
use std::collections::HashMap;
use std::env;
use tabled::{Table, Tabled};

// Client creation helper
fn build_client(api_key: String, bearer_token: Option<String>) -> Result<Client> {
    let mut client_builder = ClientBuilder::new().api_key(api_key);
    if let Some(token) = bearer_token {
        client_builder = client_builder.bearer_token(token);
    }
    if let Ok(url) = env::var("NUMISTA_API_URL") {
        client_builder = client_builder.base_url(url);
    }
    Ok(client_builder.build()?)
}

async fn fetch_collection(api_key: String, user_id: i64) -> Result<Vec<CollectedItem>> {
    let client = build_client(api_key.clone(), None)?;
    let token_params = OAuthTokenParams {
        grant_type: GrantType::ClientCredentials,
        client_id: None,
        client_secret: None,
        code: None,
        redirect_uri: None,
        scope: Some("view_collection".to_string()),
    };
    let token = client.get_oauth_token(&token_params).await?;
    let client = build_client(api_key, Some(token.access_token))?;

    let params = GetCollectedItemsParams::new();
    let response = client.get_collected_items(user_id, &params).await?;
    Ok(response.items)
}

// CLI definition
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Your Numista API key. Can also be provided via the NUMISTA_API_KEY environment variable.
    #[arg(short, long, env = "NUMISTA_API_KEY")]
    api_key: String,

    /// The ID of the user to fetch the collection for.
    #[arg(long)]
    user_id: i64,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Dump the user's collection to the console.
    Dump,
    /// Summarize the user's collection by issuer.
    Summarize,
}

// Data structures and helpers for formatting
#[derive(Tabled)]
struct IssuerSummary {
    #[tabled(rename = "Issuer")]
    issuer: String,
    #[tabled(rename = "Total Items")]
    total_items: usize,
    #[tabled(rename = "Oldest Item")]
    oldest_item: String,
    #[tabled(rename = "Newest Item")]
    newest_item: String,
}

fn get_issuer_name(item: &CollectedItem) -> String {
    item.type_info
        .issuer
        .as_ref()
        .map(|i| i.name.clone())
        .unwrap_or_else(|| "<Unknown>".to_string())
}

fn get_year(item: &CollectedItem) -> Option<i32> {
    item.issue.as_ref().and_then(|i| i.year)
}

fn get_gregorian_year(item: &CollectedItem) -> Option<i32> {
    item.issue.as_ref().and_then(|i| i.gregorian_year)
}

// Command handlers
async fn dump_collection(api_key: String, user_id: i64) -> Result<()> {
    let mut items = fetch_collection(api_key, user_id).await?;

    items.sort_by(|a, b| {
        let a_issuer = get_issuer_name(a);
        let b_issuer = get_issuer_name(b);
        let a_year = get_gregorian_year(a);
        let b_year = get_gregorian_year(b);
        let a_title = &a.type_info.title;
        let b_title = &b.type_info.title;

        a_issuer
            .cmp(&b_issuer)
            .then_with(|| a_year.cmp(&b_year))
            .then_with(|| a_title.cmp(b_title))
    });

    for item in items {
        let issuer_name = get_issuer_name(&item);
        let year_str = get_year(&item)
            .map(|y| y.to_string())
            .unwrap_or_else(|| "<Unknown>".to_string());

        println!(
            "{} - {} ({})",
            issuer_name, item.type_info.title, year_str
        );
    }

    Ok(())
}

async fn summarize_collection(api_key: String, user_id: i64) -> Result<()> {
    let items = fetch_collection(api_key, user_id).await?;

    let mut by_issuer: HashMap<String, Vec<CollectedItem>> = HashMap::new();
    for item in items {
        let issuer_name = get_issuer_name(&item);
        by_issuer.entry(issuer_name).or_default().push(item);
    }

    let mut summaries = by_issuer
        .into_iter()
        .map(|(issuer, items)| {
            let total_items = items.len();
            let mut years: Vec<i32> = items.iter().filter_map(get_gregorian_year).collect();
            years.sort_unstable();
            let oldest_item = years
                .first()
                .map(|y| y.to_string())
                .unwrap_or_else(|| "<Unknown>".to_string());
            let newest_item = years
                .last()
                .map(|y| y.to_string())
                .unwrap_or_else(|| "<Unknown>".to_string());

            IssuerSummary {
                issuer,
                total_items,
                oldest_item,
                newest_item,
            }
        })
        .collect::<Vec<_>>();

    summaries.sort_by(|a, b| a.issuer.cmp(&b.issuer));

    let table = Table::new(summaries).to_string();
    println!("{}", table);

    Ok(())
}

// Main entrypoint
#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Dump => dump_collection(cli.api_key, cli.user_id).await?,
        Commands::Summarize => summarize_collection(cli.api_key, cli.user_id).await?,
    }

    Ok(())
}
