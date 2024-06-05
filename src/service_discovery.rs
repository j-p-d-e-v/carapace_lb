use async_trait::async_trait;
use pingora::lb::discovery::ServiceDiscovery;
use pingora::tls::error;
use std::collections::BTreeSet;
use std::collections::HashMap;
use pingora::protocols::l4::socket::SocketAddr;
use pingora::lb::Backend;
use pingora::prelude::Result;
use crate::docker::DockerService;
use crate::config::{Config, ProxyService};

pub struct SD {
    config: Config
}

#[async_trait]
impl ServiceDiscovery for SD {
    async fn discover(&self) -> Result<(BTreeSet<Backend>, HashMap<u64,bool>)> {
        let docker_service: DockerService = DockerService::new().await;
        let proxy_services: Vec<ProxyService> = self.config.proxy_services.clone();
        let mut backends: BTreeSet<Backend> = BTreeSet::new();
        for ps in proxy_services {  
            let port: u16 = ps.container_port;      
            let containers = docker_service.containers(HashMap::from([(
                "label".to_string(),
                Vec::from([format!("{}={}",ps.container_label_key,ps.container_label_value)])
            )])).await;            
            for container in containers {
                let ip_address: String = docker_service.container_ip_address(&container).await;
                let addr: SocketAddr = format!("{}:{}",ip_address,port).parse::<SocketAddr>().unwrap_or_else(|error| {
                    panic!("PROXY_SERVICE_ADDR_FAILED: {:?}",error);
                });
                backends.insert(Backend { addr, weight:1 });
            }
        }

        Ok((backends, HashMap::new()))
    }
}
