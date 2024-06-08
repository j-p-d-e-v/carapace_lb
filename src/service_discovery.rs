use async_trait::async_trait;
use pingora::lb::discovery::ServiceDiscovery;
use std::collections::BTreeSet;
use std::collections::HashMap;
use pingora::protocols::l4::socket::SocketAddr;
use pingora::lb::Backend;
use pingora::prelude::Result;
use crate::docker::DockerService;
use crate::config::{Config, ProxyService};
use crate::routes::{BackendMapping, Routes};

pub struct SD {
    pub config: Config
}
#[async_trait]
impl ServiceDiscovery for SD {
    async fn discover(&self) -> Result<(BTreeSet<Backend>, HashMap<u64,bool>)> {
        let docker_service: DockerService = DockerService::new().await;
        let proxy_services: Vec<ProxyService> = self.config.proxy_services.clone();
        let mut backends: BTreeSet<Backend> = BTreeSet::new();
        let backend_routes: Routes = Routes::new(self.config.load_balancer.routes_path.clone());
        let mut backend_mapping: Vec<BackendMapping> = Vec::new();
        for ps in proxy_services {  
            let container_port: u16 = ps.container_port; 
            let container_path: String = ps.container_path;       
            let containers = docker_service.containers(HashMap::from([(
                "label".to_string(),
                Vec::from([format!("{}={}",ps.container_label_key,ps.container_label_value)])
            )])).await;            
            for container in containers {
                let container_ip_address: String = if ps.container_private {
                    docker_service.container_ip_address(&container).await
                } else {
                    ps.container_public_ip.clone()
                };
                if container_ip_address.is_empty() {
                    panic!("CONTAINER_NO_IP_ADDRESS: {}",container.id.unwrap());
                }
                let addr_string: String = format!("{}:{}",container_ip_address,container_port);
                let addr: SocketAddr = addr_string.parse::<SocketAddr>().unwrap_or_else(|error| {
                    panic!("PROXY_SERVICE_ADDR_FAILED: {:?}",error);
                });
                let backend = Backend { addr, weight: 1};
                backend_mapping.push(BackendMapping{
                    addr: addr_string,
                    path: container_path.clone()
                });
                backends.insert(backend);
            }
        }
        backend_routes.write(backend_mapping);
        Ok((backends, HashMap::new()))
    }
}