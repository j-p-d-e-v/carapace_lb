use pingora::prelude::*;
use async_trait::async_trait;
use pingora::lb::LoadBalancer;
use crate::config::Config;
use std:: sync::{Arc, Mutex};
use http::uri::Uri;
use crate::routes::{BackendMapping, Routes};

pub struct LB {
    pub load_balancer: Arc<LoadBalancer<RoundRobin>>,
    pub config: Config
}

#[async_trait]
impl ProxyHttp for LB {
    type CTX = ();
    fn new_ctx(&self) -> () {
        ()
    }

    async fn upstream_peer(
        &self, 
        _session: &mut Session, 
        _ctx: &mut Self::CTX,
    ) -> Result<Box<HttpPeer>> {
        let config = self.config.clone();
        let container_path: Arc<Mutex<String>> =  Arc::new(Mutex::new(String::new()));
        let url_path = _session.req_header().uri.path().to_string();
        let backend_routes: Routes = Routes::new(config.load_balancer.routes_path);
        let backend_mapping: Vec<BackendMapping> = backend_routes.read();

        match self.load_balancer.select_with(b"", 256, |backend,accept| {
            let addr: String = backend.addr.to_string();
            if let Some(_) = backend_mapping.clone().into_iter().find(|bm| {
                let is_addr_matched_accepted: bool =  bm.addr == addr && accept;
                if bm.path == "/" || bm.path.is_empty() {
                    if is_addr_matched_accepted && url_path.replace("/","") == bm.path.replace("/","") {
                        return true;
                    }
                }
                else {
                    if is_addr_matched_accepted && url_path.starts_with(bm.path.as_str())  {
                        let mut path: std::sync::MutexGuard<String> = container_path.lock().unwrap();
                        *path = bm.path.clone();
                        return true;
                    }
                }
                false
            }) {
                return accept;
            }
            false
        }) {
            Some(upstream) => {
                let container_path: String = container_path.lock().unwrap().clone();
                if !container_path.is_empty() {
                    _session.req_header_mut().set_uri(Uri::try_from(
                        url_path.replace(container_path.as_str(), "/")
                    ).unwrap());
                }
                let peer: Box<HttpPeer> = Box::new(
                    HttpPeer::new(upstream,false,"0.0.0.0".to_string())
                );
                Ok(peer)
            }
            None => {
                Err(pingora::Error::new_str("NO_HEALTH_UPSTREAM_PEER"))
            }
        }
    }
}