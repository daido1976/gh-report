use graphql_client::GraphQLQuery;

// Not used in GraphQLQuery
pub type NameWithOwner = String;

#[allow(clippy::upper_case_acronyms)]
pub type URI = String;
type DateTime = chrono::DateTime<chrono::Utc>;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/gql/schema.docs.graphql",
    query_path = "src/gql/me.gql",
    response_derives = "Debug"
)]
pub struct Me;

pub async fn exec(github_api_token: String) -> me::MeViewerContributionsCollection {
    let client = reqwest::Client::builder()
        .user_agent("graphql-rust/0.10.0")
        .default_headers(
            std::iter::once((
                reqwest::header::AUTHORIZATION,
                reqwest::header::HeaderValue::from_str(&format!("Bearer {}", github_api_token))
                    .unwrap(),
            ))
            .collect(),
        )
        .build()
        .unwrap();

    // TODO: Pass variables from command line arguments
    let variables = me::Variables {
        from_date_time: chrono::Utc::now() - chrono::Duration::days(10),
        to_date_time: chrono::Utc::now(),
    };

    let response_body = graphql_client::reqwest::post_graphql::<Me, _>(
        &client,
        "https://api.github.com/graphql",
        variables,
    )
    .await
    .unwrap();

    response_body.data.unwrap().viewer.contributions_collection
}

impl std::fmt::Display for me::IssueState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            me::IssueState::OPEN => write!(f, "open"),
            me::IssueState::CLOSED => write!(f, "closed"),
            _ => write!(f, "unknown"),
        }
    }
}

impl std::fmt::Display for me::PullRequestState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            me::PullRequestState::OPEN => write!(f, "open"),
            me::PullRequestState::CLOSED => write!(f, "closed"),
            me::PullRequestState::MERGED => write!(f, "merged"),
            _ => write!(f, "unknown"),
        }
    }
}
