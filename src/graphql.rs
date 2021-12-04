use graphql_client::{GraphQLQuery, Response};
use std::error::Error;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/gql/schema.docs.graphql",
    query_path = "src/gql/myquery.gql",
    response_derives = "Debug"
)]
pub struct MyQuery;

pub async fn exec(github_api_token: String) {
    println!("Hello, graphql!");
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

    let variables = my_query::Variables {};

    let response_body = graphql_client::reqwest::post_graphql::<MyQuery, _>(
        &client,
        "https://api.github.com/graphql",
        variables,
    )
    .await
    .unwrap();

    println!("{:?}", response_body);
    let issues = response_body.data.unwrap().repository.unwrap().issues;
    println!("\n issues!!! {:?}", issues);
}
