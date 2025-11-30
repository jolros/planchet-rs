use anyhow::Result;
use clap::Parser;
use planchet::{models::GrantType, ClientBuilder, GetCollectedItemsParams, OAuthTokenParams};

/// A manuel tool for verifying the deserialization of all read-only API responses.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// The user ID to use for authentication.
    #[arg(long)]
    user_id: i64,

    /// The Numista API key.
    #[arg(long, env = "NUMISTA_API_KEY")]
    api_key: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    let pre_auth_client = ClientBuilder::new().api_key(&cli.api_key).build()?;
    let params = OAuthTokenParams {
        grant_type: GrantType::ClientCredentials,
        code: None,
        client_id: None,
        client_secret: None,
        redirect_uri: None,
        scope: None,
    };
    let token = pre_auth_client.get_oauth_token(&params).await?;

    let client = ClientBuilder::new()
        .api_key(&cli.api_key)
        .bearer_token(&token.access_token)
        .build()?;

    println!("Successfully authenticated!");

    let issuers = client.get_issuers(None).await?;
    println!("{:#?}", issuers);

    let mints = client.get_mints(None).await?;
    println!("{:#?}", mints);
    if let Some(mint) = mints.mints.first() {
        println!("{:#?}", client.get_mint(mint.id, None).await?);
    }

    let catalogues = client.get_catalogues().await?;
    println!("{:#?}", catalogues);

    let user_collections = client.get_user_collections(cli.user_id).await?;
    println!("{:#?}", user_collections);

    let collected_items = client
        .get_collected_items(cli.user_id, &GetCollectedItemsParams::new())
        .await?;
    println!("{:#?}", collected_items);
    if let Some(item) = collected_items.items.first() {
        println!(
            "{:#?}",
            client.get_collected_item(cli.user_id, item.id).await?
        );
        let r#type = client.get_type(item.type_info.id, None).await?;
        println!("{:#?}", r#type);

        let issues = client.get_issues(item.type_info.id, None).await?;
        println!("{:#?}", issues);

        if let Some(issue) = &item.issue {
            println!(
                "{:#?}",
                client
                    .get_prices(item.type_info.id, issue.id, None, None)
                    .await?
            );
        }
    }

    println!("{:#?}", client.get_user(cli.user_id, None).await?);
    println!("{:#?}", client.get_publication("L106610").await?);

    Ok(())
}
