use pingora::prelude::*;
use async_trait::async_trait;
use pingora::lb::LoadBalancer;
use pingora::protocols::Digest;
use crate::config::Config;
use std::sync::{Arc, Mutex};
use http::uri::Uri;
use crate::routes::{BackendMapping, Routes};


/// The load balancer struct that holdsthe Load Balancer instance and the configuration values. It inherits the ProxyHttp trait.
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


    async fn connected_to_upstream(&self, 
        _session: &mut Session,
        _reused: bool,
        _peer: &HttpPeer,
        _fd: std::os::unix::io::RawFd,
        _digest: Option<&Digest>,
        _ctx: &mut Self::CTX ) -> Result<()> {
            let inet = _peer._address.as_inet().unwrap().to_string();
            match &self.config.load_balancer.debug {
                status if status == &String::from("INFO") => {
                    println!("(INFO) [{:?}] - {}",inet,_session.req_header().uri.to_string());
                }
                status if status == &String::from("DEBUG") => {
                    println!("(DEBUG) [{:?}] - {:?}",inet,_session.req_header());
                }
                _ => {}
            };
            Ok(())
        }

    async fn upstream_peer(
        &self, 
        _session: &mut Session, 
        _ctx: &mut Self::CTX,
    ) -> Result<Box<HttpPeer>> {
        let config = self.config.clone();
        let matched_path: Arc<Mutex<String>> =  Arc::new(Mutex::new(String::new()));
        let url_path = _session.req_header().uri.path().to_string();
        let lb_routes_path = config.load_balancer.routes_path.clone();
        let backend_routes: Routes = Routes::new(lb_routes_path);
        let backend_mapping: Vec<BackendMapping> = backend_routes.read();    
        let enable_tls_ssl: bool = config.load_balancer.enable_tls_ssl.clone();
        let is_file: bool = if let Some(_) = _session.req_header().uri_file_extension() { true } else { false };

        match self.load_balancer.select_with(b"", 256, |backend,accept| {
            let addr: String = backend.addr.to_string();
            let config: Config = config.clone();
            let mut url_path = url_path.clone();
            
            if is_file {
                if let Some(matched) = self.get_file_in_path(config.load_balancer.allow_file_types_pattern,url_path.clone()) {
                    let file_name = matched.as_str();
                    url_path = url_path.replace(file_name, "").to_string(); 
                }
            }
            
            if let Some(_) = backend_mapping.clone().into_iter().find(|bm| {
                let is_addr_matched_accepted: bool =  bm.addr == addr && accept;
                if bm.path == "/" || bm.path.is_empty() {
                    if is_addr_matched_accepted && url_path.replace("/","") == bm.path.replace("/","") {
                        return true;
                    }
                }
                else {
                    if is_addr_matched_accepted && url_path.starts_with(bm.path.as_str())  {
                        let mut path: std::sync::MutexGuard<String> = matched_path.lock().unwrap();
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
                let path: String = matched_path.lock().unwrap().clone();
                if !path.is_empty() {
                    _session.req_header_mut().set_uri(Uri::try_from(
                        url_path.replace(path.as_str(), "/")
                    ).unwrap());
                }
                let http = HttpPeer::new(upstream,enable_tls_ssl,"".to_string());
                let peer: Box<HttpPeer> = Box::new(http);
                Ok(peer)
            }
            None => {
                Err(pingora::Error::new_str("NO_HEALTH_UPSTREAM_PEER"))
            }
        }
    }
}

impl LB {
    /// Check if the url path is targeting a file.
    pub fn get_file_in_path(&self,pattern: String, path: String) -> Option<String> {
        use regex::Regex;
        let regex_pattern: &str = pattern.as_str();        
        let _regex = Regex::new(regex_pattern).unwrap();
        if let Some(matched) = _regex.find(path.as_str()) {
            return Some(matched.as_str().to_string());
        }
        None
    }
    
}