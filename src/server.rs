use pingora::lb::health_check::TcpHealthCheck;
use pingora::prelude::{ Server, background_service, http_proxy_service};
use pingora::server::configuration::{ Opt, ServerConf };
use crate::service_discovery::SD;
use crate::load_balancer::LB;
use crate::config::Config;
use std::time::Duration;
use pingora::lb::{
    Backends,
    LoadBalancer,
    selection::{
        algorithms::RoundRobin,
        weighted::Weighted
    },
};
use pingora::proxy::HttpProxy;
use pingora::services::listening::Service;

pub fn launch_server(){
    let config: Config = Config::new("config.toml".to_string());
    let ca_crt_file: &String = &config.load_balancer.ca_crt_path;
    let ca_pem_file: &String = &config.load_balancer.ca_pem_path;
    let enable_tls_ssl: &bool = &config.load_balancer.enable_tls_ssl;

    let mut server = Server::new_with_opt_and_conf(
        Opt::default(),
        ServerConf::default()
    );
    server.bootstrap();        
    let service_dicovery = SD { config: config.clone() };
    let backends = Backends::new(Box::new(service_dicovery));
    let mut upstreams:LoadBalancer<Weighted<RoundRobin>> = LoadBalancer::from_backends(backends);
    let health_check = TcpHealthCheck::new();
    let health_check_frequency: u64 = config.health_check.health_check_frequency;
    let update_frequency: u64 = config.health_check.update_frequency;
    let parallel_health_check: bool = config.health_check.parallel_health_check;
    upstreams.set_health_check(health_check);
    upstreams.health_check_frequency = Some(Duration::from_millis(health_check_frequency));
    upstreams.update_frequency = Some(Duration::from_millis(update_frequency));
    upstreams.parallel_health_check = parallel_health_check;
    let background = background_service("health_check", upstreams);
    let upstreams = background.task();
    let mut lb: Service<HttpProxy<LB>> = http_proxy_service(&server.configuration, LB {
        load_balancer: upstreams,
        config: config.clone()
    });
    let lb_host = config.load_balancer.host;
    let lb_port = config.load_balancer.port;
    let lb_addr: String = format!("{}:{}",lb_host,lb_port);
    if enable_tls_ssl == &true {
        println!("running on {}:{} in TLS/SSL",lb_host,lb_port);
        lb.add_tls(&lb_addr, ca_crt_file, ca_pem_file).unwrap();
    }
    else{
        println!("running on {}:{}",lb_host,lb_port);
        lb.add_tcp(format!("{}:{}",lb_host,lb_port).as_str());
    }
    server.add_service(background);
    server.add_service(lb);
    server.run_forever();
}