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
//! planchet-cli --api-key <YOUR_API_KEY> <COMMAND>
//! ```
//!
//! # Commands
//!
//! ## `dump`
//!
//! Dumps the user's collection to the console, sorted by issuer name, year, and title.
//!
//! ```bash
//! $ planchet-cli --api-key my-secret-key dump --user-id 123
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
//! $ planchet-cli --api-key my-secret-key summarize --user-id 123
//! +--------+-------------+-------------+-------------+
//! | Issuer | Total Items | Oldest Item | Newest Item |
//! +--------+-------------+-------------+-------------+
//! | Canada | 2           | 1858        | 1920        |
//! +--------+-------------+-------------+-------------+
//! ```
//!
//! ## `types`
//!
//! Searches the catalogue by types using a keyword and an optional year.
//!
//! ```bash
//! $ planchet-cli --api-key my-secret-key types --query "Victoria" --year 1858
//! Found 1 results for query: 'Victoria', year: 1858.
//! +----+--------------------+----------+--------+----------+----------+
//! | ID | Title              | Category | Issuer | Min Year | Max Year |
//! +----+--------------------+----------+--------+----------+----------+
//! | 42 | 5 Cents - Victoria | coin     | Canada | 1858     | 1901     |
//! +----+--------------------+----------+--------+----------+----------+
//! ```
use anyhow::Result;
use clap::{Parser, Subcommand};
use futures::stream::TryStreamExt;
use planchet::{
    models::{CollectedItem, GrantType, SearchTypeResult},
    Client, ClientBuilder, GetCollectedItemsParams, OAuthTokenParams, SearchTypesParams,
};
use std::collections::HashMap;
use std::env;
use std::io::{self, Write};
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

    /// Enable debug logging.
    #[arg(long, global = true)]
    debug: bool,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Dump the user's collection to the console.
    Dump {
        /// The ID of the user to fetch the collection for.
        #[arg(long)]
        user_id: i64,
    },
    /// Summarize the user's collection by issuer.
    Summarize {
        /// The ID of the user to fetch the collection for.
        #[arg(long)]
        user_id: i64,
    },
    /// Search the catalogue by types.
    Types {
        /// The search query.
        #[arg(long)]
        query: String,

        /// The year to search for.
        #[arg(long)]
        year: Option<i32>,

        /// Retrieve all items at once.
        #[arg(long)]
        all: bool,
    },
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

#[derive(Tabled)]
struct TypeResult {
    #[tabled(rename = "ID")]
    id: i64,
    #[tabled(rename = "Title")]
    title: String,
    #[tabled(rename = "Category")]
    category: String,
    #[tabled(rename = "Issuer")]
    issuer: String,
    #[tabled(rename = "Min Year")]
    min_year: String,
    #[tabled(rename = "Max Year")]
    max_year: String,
}

impl From<SearchTypeResult> for TypeResult {
    fn from(t: SearchTypeResult) -> Self {
        Self {
            id: t.id,
            title: t.title,
            category: t.category.to_string(),
            issuer: t.issuer.name,
            min_year: t
                .min_year
                .map(|y| y.to_string())
                .unwrap_or_else(|| "<unknown>".to_string()),
            max_year: t
                .max_year
                .map(|y| y.to_string())
                .unwrap_or_else(|| "<unknown>".to_string()),
        }
    }
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

fn print_search_header(count: i64, query: &str, year: Option<i32>) {
    let search_details = format!(
        "query: '{}'{}",
        query,
        year.map(|y| format!(", year: {}", y))
            .unwrap_or_else(|| "".to_string())
    );
    println!("Found {} results for {}.", count, search_details);
}

async fn search_types(api_key: String, query: String, year: Option<i32>, all: bool) -> Result<()> {
    let client = build_client(api_key, None)?;
    let mut params = SearchTypesParams::new().q(&query);
    if let Some(y) = year {
        params = params.date(y);
    }

    if all {
        let types = client
            .stream_all_types(params)
            .try_collect::<Vec<_>>()
            .await?;
        print_search_header(types.len() as i64, &query, year);
        let results: Vec<TypeResult> = types.into_iter().map(TypeResult::from).collect();
        let table = Table::new(results).to_string();
        println!("{}", table);
    } else {
        let mut page = 1;
        let count = 25;
        loop {
            let response = client
                .search_types(&params.clone().page(page).count(count))
                .await?;

            if page == 1 {
                print_search_header(response.count, &query, year);
            }

            if response.types.is_empty() {
                break;
            }

            let results: Vec<TypeResult> =
                response.types.into_iter().map(TypeResult::from).collect();
            let table = Table::new(results).to_string();
            println!("{}", table);

            if page * count >= response.count {
                break;
            }

            print!("Press 'n' or space for the next page, 'q' to quit: ");
            io::stdout().flush()?;

            let mut input = String::new();
            io::stdin().read_line(&mut input)?;

            match input.trim() {
                "n" | "" => page += 1,
                "q" => break,
                _ => {
                    println!("Invalid input. Quitting.");
                    break;
                }
            }
        }
    }

    Ok(())
}

// Main entrypoint
#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    let log_level = if cli.debug { "trace" } else { "info" };
    let env_filter = tracing_subscriber::EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| {
            tracing_subscriber::EnvFilter::new(format!(
                "planchet={},planchet_cli={},reqwest={}",
                log_level, log_level, log_level
            ))
        });

    tracing_subscriber::fmt()
        .with_env_filter(env_filter)
        .with_writer(std::io::stderr)
        .with_ansi(false)
        .init();

    match cli.command {
        Commands::Dump { user_id } => dump_collection(cli.api_key, user_id).await?,
        Commands::Summarize { user_id } => summarize_collection(cli.api_key, user_id).await?,
        Commands::Types { query, year, all } => search_types(cli.api_key, query, year, all).await?,
    }

    Ok(())
}
