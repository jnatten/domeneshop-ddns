use tracing_log::log;

const DEFAULT_INTERVAL: u64 = 60;

fn get_sleep_interval() -> u64 {
    let maybe_env_value = std::env::var("SLEEP_INTERVAL_SECONDS");
    let Ok(env_value) = maybe_env_value else {
        log::info!(
            "SLEEP_INTERVAL_SECONDS not set. Using default of {} seconds.",
            DEFAULT_INTERVAL
        );
        return DEFAULT_INTERVAL;
    };

    let Ok(interval) = env_value.parse::<u64>() else {
        log::error!(
            "Failed to parse SLEEP_INTERVAL_SECONDS: '{}', must be some integer in seconds.",
            env_value
        );
        return DEFAULT_INTERVAL;
    };

    log::info!("Using interval of {} seconds", interval);
    interval
}

fn get_domains_to_update() -> Vec<String> {
    let maybe_env_value = std::env::var("DDNS_DOMAINS");
    let Ok(env_value) = maybe_env_value else {
        log::error!("DDNS_DOMAINS not set. Must be a comma-separated string of domains.");
        std::process::exit(1);
    };

    env_value.split(",").map(|s| s.to_string()).collect()
}

fn get_required_env_var(var_name: &str) -> String {
    match get_env_var_opt(var_name) {
        Some(value) => value,
        None => {
            log::error!("'{}' not set, but is required.", var_name);
            std::process::exit(1);
        }
    }
}

fn get_env_var_opt(var_name: &str) -> Option<String> {
    std::env::var(var_name).ok()
}

pub struct Config {
    pub domains: Vec<String>,
    pub token: String,
    pub secret: String,
    pub interval: u64,
    pub myip: Option<String>,
}

impl Config {
    pub fn from_env() -> Self {
        Self {
            domains: get_domains_to_update(),
            interval: get_sleep_interval(),
            token: get_required_env_var("DOMENESHOP_TOKEN"),
            secret: get_required_env_var("DOMENESHOP_SECRET"),
            myip: get_env_var_opt("MY_IP"),
        }
    }
}
