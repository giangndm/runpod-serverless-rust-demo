use runpod_sdk::model::ListEndpointsQuery;
use runpod_sdk::service::EndpointsService;
use runpod_sdk::{Result, RunpodClient};

#[tokio::main]
async fn main() -> Result<()> {
    let client = RunpodClient::from_env()?;

    let endpoints = client
        .list_endpoints(ListEndpointsQuery {
            include_template: Some(false),
            include_workers: Some(false),
        })
        .await?;

    if endpoints.is_empty() {
        println!("No endpoints found for this account.");
        return Ok(());
    }

    println!("id\tname\tcompute_type\tworkers(min/max)");
    for endpoint in endpoints {
        let name = endpoint.name.unwrap_or_else(|| "-".to_string());
        println!(
            "{}\t{}\t{:?}\t{}/{}",
            endpoint.id,
            name,
            endpoint.compute_type,
            endpoint.workers_min,
            endpoint.workers_max,
        );
    }

    Ok(())
}
