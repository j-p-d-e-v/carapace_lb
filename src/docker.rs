use std::os::unix::net::SocketAddr;
use bollard::{container, Docker};
use bollard::container::ListContainersOptions;
use std::default::Default;
use std::collections::HashMap;
use bollard::models::ContainerSummary;

#[derive(Debug)]
pub struct DockerService {
    docker: Docker,
}

impl DockerService {
    pub async fn new() -> Self {
        match Docker::connect_with_local_defaults() {
            Ok(docker) => {
                Self {
                    docker
                }  
            }
            Err(error) => {
                panic!("DOCKER_CONNECT_FAILED: {:?}",error);
            }
        }      
    }
    pub async fn containers(&self, filters: HashMap<String,Vec<String>>) -> Vec<ContainerSummary> {
        let options: Option<ListContainersOptions<_>> = Some(ListContainersOptions {
            all: false,
            filters: filters,
            ..Default::default()
        });
        match self.docker.list_containers(options).await {
            Ok(containers) => {
                containers
            }
            Err(error) => {
                panic!("DOCKER_LIST_CONTAINERS_FAILED: {:?}",error);
            }
        }
    }

    pub async fn container_ip_address(&self,container: &ContainerSummary) -> String {
        if let Some(container_id) = &container.id {
            if let Some(host_config) = &container.host_config {
                if let Some(network_mode) = &host_config.network_mode {
                    match &container.network_settings {
                        Some(ns) => {
                            if let Some(networks) = &ns.networks {
                                if let Some(endpoint_settings) = networks.get_key_value(&network_mode.clone()) {
                                    if let Some(ip_address) = &endpoint_settings.1.ip_address {
                                        return ip_address.clone();
                                    }

                                }
                            }
                        }
                        None => {                        
                            eprint!("no network settings found in network mode {} for container {}",network_mode,container_id);
                        }
                    }
                }
                else{
                    eprint!("no network mode found for container {}",container_id);
                }
            }
            else{
                eprint!("no host config for container {}",container_id);
            }
        }
        String::new()
    }
}