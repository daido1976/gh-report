mod adapter;
mod gh;
mod graphql;
mod presenter;
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let github_api_token = gh::fetch_token();
    let contributions_collection = graphql::exec(github_api_token).await?;
    let my_contributions = adapter::combine(contributions_collection);
    println!("{}", presenter::to_string_pretty(my_contributions));
    Ok(())
}
