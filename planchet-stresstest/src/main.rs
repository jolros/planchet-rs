use anyhow::Result;
use clap::Parser;
use planchet::{
    models::GrantType, ClientBuilder, GetCollectedItemsParams, OAuthTokenParams, SearchTypesParams,
};
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

/// A manual tool for verifying the deserialization of all read-only API responses.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// The user ID to use for authentication.
    #[arg(long)]
    user_id: i64,

    /// The Numista API key.
    #[arg(long, env = "NUMISTA_API_KEY")]
    api_key: String,

    /// Enable debug logging.
    #[arg(long)]
    debug: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    let subscriber = FmtSubscriber::builder()
        .with_max_level(if cli.debug {
            Level::TRACE
        } else {
            Level::INFO
        })
        .with_ansi(false)
        .finish();
    tracing::subscriber::set_global_default(subscriber)
        .expect("Failed to set tracing subscriber");

    println!("Calling get_oauth_token()");
    let pre_auth_client = ClientBuilder::new().api_key(&cli.api_key).build()?;
    let params = OAuthTokenParams {
        grant_type: GrantType::ClientCredentials,
        code: None,
        client_id: None,
        client_secret: None,
        redirect_uri: None,
        scope: Some("view_collection".to_string()),
    };
    let token = pre_auth_client.get_oauth_token(&params).await?;

    let client = ClientBuilder::new()
        .api_key(&cli.api_key)
        .bearer_token(&token.access_token)
        .build()?;

    println!("Successfully authenticated!");

    println!("Calling get_issuers()");
    let issuers = client.get_issuers().await?;
    println!("{:#?}", issuers);

    println!("Calling get_mints()");
    let mints = client.get_mints().await?;
    println!("{:#?}", mints);
    if let Some(mint) = mints.mints.first() {
        println!("Calling get_mint()");
        println!("{:#?}", client.get_mint(mint.id).await?);
    }

    println!("Calling get_catalogues()");
    let catalogues = client.get_catalogues().await?;
    println!("{:#?}", catalogues);

    println!("Calling get_user_collections()");
    let user_collections = client.get_user_collections(cli.user_id).await?;
    println!("{:#?}", user_collections);

    println!("Calling get_collected_items()");
    let collected_items = client
        .get_collected_items(cli.user_id, &GetCollectedItemsParams::new())
        .await?;
    println!("{:#?}", collected_items);
    if let Some(item) = collected_items.items.first() {
        println!("Calling get_collected_item()");
        println!(
            "{:#?}",
            client.get_collected_item(cli.user_id, item.id).await?
        );
        println!("Calling get_type()");
        let r#type = client.get_type(item.type_info.id).await?;
        println!("{:#?}", r#type);

        println!("Calling get_issues()");
        let issues = client.get_issues(item.type_info.id).await?;
        println!("{:#?}", issues);

        if let Some(issue) = &item.issue {
            println!("Calling get_prices()");
            println!(
                "{:#?}",
                client
                    .get_prices(item.type_info.id, issue.id, None)
                    .await?
            );
        }
    }

    println!("Calling get_user()");
    println!("{:#?}", client.get_user(cli.user_id).await?);
    println!("Calling get_publication()");
    println!("{:#?}", client.get_publication("L106610").await?);
    println!("Calling search_types()");
    let params = SearchTypesParams::new().q("victoria");
    println!("{:#?}", client.search_types(&params).await?);

    Ok(())
}
