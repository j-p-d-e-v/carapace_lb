#[test]
fn test_routes(){
    use crate::routes::{BackendMapping, Routes};
    let r = Routes::new("tests/routes_test.json".to_string());
    let route_data: Vec<BackendMapping> = Vec::from([
        BackendMapping{ addr:"1.1.1.1:80".to_string(), path: "/test1".to_string() },
        BackendMapping{ addr:"1.1.1.2:82".to_string(), path: "/test2".to_string() },
        BackendMapping{ addr:"1.1.1.3:83".to_string(), path: "/test3".to_string() },
    ]);
    let total_routes = route_data.len();
    let write_status = r.write(route_data);
    let read_routes = r.read();
    assert!(write_status);
    assert_eq!(read_routes.len(),total_routes);
}