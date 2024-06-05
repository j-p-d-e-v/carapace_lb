# Container Load Balancer (CBL)

A load balancer for containers. Uses [Pingora Framework by Cloudflare](https://github.com/cloudflare/pingora).

# Todo
- [x] Develop a struct for retrieving containers based on labels.
- [x] Develop confinguration using TOML for defining the load balancer, proxy services, and healthchecks.
- [x] Develop Service Discovery and uses the discovered containers IP:PORT as upstreams.
- [ ] Develop Load Balancer and Specific Routing / Targeting specific upstream
- [ ] Add TLS/SSL Feature
- [ ] Test using Websocket
OTHER TASKS TBD...

# Configuration

The configuration uses TOML config format. See https://toml.io/en/v1.0.0 for more details.

## ```[load_balancer]```
Configure the load balancer.
```
host="0.0.0.0"
port=6170
```


## ```[[proxy_services]]```
Discover the containers based on the configured container labels
```
Format: [<label_key>,<label_value>]
```
```
container_labels =[
    [ "cbl.dev.nginx","groupA"]
]
```


## ```[health_checks]```
Configure healthchecks and auto discovery.
Reference: https://docs.rs/pingora/0.2.0/pingora/lb/struct.LoadBalancer.html#structfield.health_check_frequency
```
health_check_frequency=15
update_frequency=15
parallel_health_check=false
```

