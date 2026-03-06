use reqwest::Client;
use runpod_sdk::RunpodConfig;
use runpod_sdk::model::{ComputeType, ListEndpointsQuery};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct EndpointSummary {
    id: String,
    name: Option<String>,
    #[serde(default)]
    compute_type: Option<ComputeType>,
    #[serde(default)]
    workers_min: i32,
    #[serde(default)]
    workers_max: i32,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = RunpodConfig::from_env()?;
    let client = Client::builder().timeout(config.timeout()).build()?;
    let url = format!("{}/endpoints", config.rest_url().trim_end_matches('/'));

    let endpoints = client
        .get(url)
        .bearer_auth(config.api_key())
        .query(&ListEndpointsQuery {
            include_template: Some(false),
            include_workers: Some(false),
        })
        .send()
        .await?
        .error_for_status()?
        .json::<Vec<EndpointSummary>>()
        .await?;

    if endpoints.is_empty() {
        println!("No endpoints found for this account.");
        return Ok(());
    }

    println!("id\tname\tcompute_type\tworkers(min/max)");
    for endpoint in endpoints {
        let name = endpoint.name.unwrap_or_else(|| "-".to_string());
        let compute_type = endpoint
            .compute_type
            .map(|value| format!("{value:?}"))
            .unwrap_or_else(|| "-".to_string());
        println!(
            "{}\t{}\t{}\t{}/{}",
            endpoint.id, name, compute_type, endpoint.workers_min, endpoint.workers_max,
        );
    }

    Ok(())
}
