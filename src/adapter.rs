use crate::graphql::{me::MeViewerContributionsCollection, NameWithOwner, URI};
use serde::Deserialize;
use std::collections::HashMap;

pub type MyContributions = HashMap<NameWithOwner, Vec<IssueOrPr>>;
#[derive(Debug, Deserialize)]
pub struct IssueOrPr {
    pub title: String,
    pub url: URI,
    pub state: String,
}

pub fn combine(contributions_collection: MeViewerContributionsCollection) -> MyContributions {
    let mut result: MyContributions = HashMap::new();
    if let Some(issue_contributions) = contributions_collection.issue_contributions.edges {
        for contribution in issue_contributions.into_iter().flatten() {
            let issue = match contribution.node {
                Some(x) => x.issue,
                None => continue,
            };
            let owner: NameWithOwner = issue.repository.name_with_owner;
            let issue = IssueOrPr {
                title: issue.title,
                url: issue.url,
                state: issue.state.to_string(),
            };
            result.entry(owner).or_insert_with(Vec::new).push(issue);
        }
    }

    if let Some(pr_contributions) = contributions_collection.pull_request_contributions.edges {
        for contribution in pr_contributions.into_iter().flatten() {
            let pr = match contribution.node {
                Some(x) => x.pull_request,
                None => continue,
            };
            let owner: NameWithOwner = pr.repository.name_with_owner;
            let pr = IssueOrPr {
                title: pr.title,
                url: pr.url,
                state: pr.state.to_string(),
            };
            result.entry(owner).or_insert_with(Vec::new).push(pr);
        }
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
        assert_debug_snapshot!(
            actual,
            @r###"
        {
            "rust-lang/rust": [
                IssueOrPr {
                    title: "Cannot build on Fedora: wrong CPUTYPE?",
                    url: "https://github.com/rust-lang/rust/issues/1218",
                    state: "open",
                },
                IssueOrPr {
                    title: "rustc: Fix position of diagnostic highlight lines",
                    url: "https://github.com/rust-lang/rust/issues/1219",
                    state: "closed",
                },
                IssueOrPr {
                    title: "Add float support to #fmt.",
                    url: "https://github.com/rust-lang/rust/pull/1168",
                    state: "merged",
                },
                IssueOrPr {
                    title: "Prohibit in-scope consts from use as variable names in binders, like nullary tags",
                    url: "https://github.com/rust-lang/rust/pull/1193",
                    state: "merged",
                },
            ],
            "daido1976/terakoya": [
                IssueOrPr {
                    title: "From now to Vercel",
                    url: "https://github.com/daido1976/terakoya/pull/52",
                    state: "merged",
                },
            ],
        }
        "###
        );
    }
}
