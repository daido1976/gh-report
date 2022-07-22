mod gh;
mod graphql;
use graphql::{NameWithOwner, URI};

use crate::graphql::me::MeViewerContributionsCollection;
use std::collections::HashMap;

type MyContributions = HashMap<NameWithOwner, Vec<IssueOrPr>>;
#[derive(Debug)]
struct IssueOrPr {
    title: String,
    url: URI,
    state: String,
}

#[tokio::main]
async fn main() {
    let github_api_token = gh::fetch_token();
    let contributions_collection = graphql::exec(github_api_token).await;
    let my_contributions = combine(contributions_collection);
    puts(my_contributions);
}

fn combine(contributions_collection: MeViewerContributionsCollection) -> MyContributions {
    let issue_contributions = contributions_collection.issue_contributions.edges.unwrap();
    let pr_contributions = contributions_collection
        .pull_request_contributions
        .edges
        .unwrap();

    let mut result: MyContributions = HashMap::new();
    for contribution in issue_contributions.into_iter().flatten() {
        let issue = contribution.node.unwrap().issue;
        let owner: NameWithOwner = issue.repository.name_with_owner;
        let issue = IssueOrPr {
            title: issue.title,
            url: issue.url,
            state: issue.state.to_string(),
        };
        result.entry(owner).or_insert_with(Vec::new).push(issue);
    }

    for contribution in pr_contributions.into_iter().flatten() {
        let pr = contribution.node.unwrap().pull_request;
        let owner: NameWithOwner = pr.repository.name_with_owner;
        let pr = IssueOrPr {
            title: pr.title,
            url: pr.url,
            state: pr.state.to_string(),
        };
        result.entry(owner).or_insert_with(Vec::new).push(pr);
    }
    result
}

fn puts(my_contributions: MyContributions) {
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
