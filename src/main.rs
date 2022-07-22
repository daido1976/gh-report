mod adapter;
mod gh;
mod graphql;

#[tokio::main]
async fn main() {
    let github_api_token = gh::fetch_token();
    let contributions_collection = graphql::exec(github_api_token).await;
    let my_contributions = adapter::combine(contributions_collection);
    puts(my_contributions);
}

fn puts(my_contributions: adapter::MyContributions) {
    for (owner, issue_or_prs) in my_contributions {
        println!("\n### {}\n", owner);
        for issue_or_pr in issue_or_prs {
            println!(
                "- [{}]({}) **{}!**",
                issue_or_pr.title, issue_or_pr.url, issue_or_pr.state
            )
        }
    }
}
