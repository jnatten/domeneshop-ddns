mod config;

use crate::config::Config;
use futures::future::try_join_all;
use std::io::Result;
use tokio::time::{interval, Duration};
use tracing_log::log;

const DYNDNS_ENDPOINT: &str = "https://api.domeneshop.no/v0/dyndns/update";

async fn update_domain(domain: &str, config: &Config) -> anyhow::Result<()> {
    log::info!("Updating domain: '{}'", domain);
    let mut query: Vec<(&str, &str)> = vec![("hostname", domain)];
    if let Some(myip) = &config.myip {
        query.push(("myip", myip));
    }

    let result = reqwest::Client::new()
        .get(DYNDNS_ENDPOINT)
        .query(&query)
        .basic_auth(&config.token, Some(&config.secret))
        .send()
        .await;

    match result {
        Ok(response) => {
            if response.status().is_success() {
                log::info!("Successfully updated domain: '{}'", domain);
            } else {
                log::error!("Failed to update domain: '{}'", domain);
                log::error!("Response: {:?}", response);
            }
        }
        Err(err) => {
            log::error!("Failed to update domain: '{}'", domain);
            log::error!("Error: {:?}", err);
        }
    }

    Ok(())
}

async fn update_domains(config: &Config) {
    log::info!("---- Updating domains ----");
    let futures = config
        .domains
        .iter()
        .map(|domain| update_domain(domain, config));
    if let Err(e) = try_join_all(futures).await {
        log::error!("Failed to update domains: {}", e)
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter("domeneshop_ddns=debug")
        .init();

    let config = Config::from_env();
    let mut interval = interval(Duration::from_secs(config.interval));
    interval.tick().await;
    loop {
        update_domains(&config).await;
        interval.tick().await;
    }
}
