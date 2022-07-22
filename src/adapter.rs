use std::collections::HashMap;

use crate::graphql::{me::MeViewerContributionsCollection, NameWithOwner, URI};
use serde::Deserialize;

pub type MyContributions = HashMap<NameWithOwner, Vec<IssueOrPr>>;
#[derive(Debug, Deserialize)]
pub struct IssueOrPr {
    pub title: String,
    pub url: URI,
    pub state: String,
}

pub fn combine(contributions_collection: MeViewerContributionsCollection) -> MyContributions {
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

#[cfg(test)]
mod test {
    use super::*;
    use insta::assert_debug_snapshot;

    #[test]
    fn test_combine() {
        let cc =
            serde_json::from_str(include_str!("fixtures/contributionsCollection.json")).unwrap();
        let actual = combine(cc);
        assert_debug_snapshot!(actual);
    }
}
