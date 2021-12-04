mod graphql;
use std::env;

#[tokio::main]
async fn main() {
    let github_api_token = env::var("GITHUB_API_TOKEN").expect("Missing GITHUB_API_TOKEN env var");
    println!("{:?}", github_api_token);
    graphql::exec(github_api_token).await;
}
