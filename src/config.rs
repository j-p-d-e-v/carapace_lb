use std::fs::read_to_string;
use serde::Deserialize;


#[derive(Debug,Deserialize,Clone)]
pub struct LoadBalancer {
    pub host: String,
    pub port: u16
}

#[derive(Debug,Deserialize,Clone)]
pub struct ProxyService {
    pub container_label_key: String,
    pub container_label_value: String,
    pub container_path: String,
    pub container_port: u16 
}

#[derive(Debug,Deserialize,Clone)]
pub struct HealthCheck {
    pub health_check_frequency: u16,
    pub update_frequency: u16,
    pub parallel_health_check: bool
}

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