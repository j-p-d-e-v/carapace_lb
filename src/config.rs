use std::fs::read_to_string;
use serde::Deserialize;

/// The struct for Load Balancer configuration.
#[derive(Debug,Deserialize,Clone)]
pub struct LoadBalancer {
    pub host: String,
    pub port: u16,
    pub routes_path: String,
    #[serde(default)]
    pub enable_tls_ssl: bool,
    #[serde(default)]
    pub ca_crt_path: String,
    #[serde(default)]
    pub ca_pem_path: String,
    #[serde(default)]
    pub allow_file_types_pattern: String,
    #[serde(default)]
    pub debug: String
}
/// The struct for Proxy Service configuration.
#[derive(Debug,Deserialize,Clone)]
pub struct ProxyService {
    #[serde(default)]
    pub container_label_key: String,
    #[serde(default)]
    pub container_label_value: String,
    #[serde(default)]
    pub path: String,
    pub port: u16,
    #[serde(default)]
    pub use_container: bool,
    #[serde(default)]
    pub host: String,
}
/// The struct for Health Check configuration.
#[derive(Debug,Deserialize,Clone)]
pub struct HealthCheck {
    pub health_check_frequency: u64,
    pub update_frequency: u64,
    pub parallel_health_check: bool
}
/// The struct that holds the configurations.
#[derive(Debug,Deserialize,Clone)]
pub struct Config {
    pub load_balancer: LoadBalancer,
    pub proxy_services: Vec<ProxyService>,
    pub health_check: HealthCheck
}

impl Config {
    pub fn new(path: String) -> Self {
        let contents: String = read_to_string(path).unwrap_or_else(|error| { 
            panic!("CONFIG_LOAD_FAILED: {:?}",error);
        });
        let config: Config = toml::from_str(&contents).unwrap_or_else(|error| {
            panic!("CONFIG_CONVERSION_FAILED: {:?}",error);
        });
        return config
    }
}

impl Default for HealthCheck {
    fn default() -> Self {
        Self {
            health_check_frequency: 30,
            update_frequency: 60,
            parallel_health_check: false
        }
    }
}