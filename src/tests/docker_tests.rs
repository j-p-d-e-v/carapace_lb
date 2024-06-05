#[tokio::test]
async fn load_containers(){
    use std::collections::HashMap;
    use crate::docker::DockerService;
    let client: DockerService  = DockerService::new().await;
    for container_label_value in vec![
        "groupA".to_string(),
        //"groupB".to_string()
    ] {
        let container_filters: HashMap<String,Vec<String>> = HashMap::from([
            ("label".to_string(),Vec::from([
                format!("{}={}","cbl.dev.nginx",container_label_value),
            ]))
        ]);
        let containers = client.containers(container_filters).await;
        assert_eq!(containers.len(),2);
        for container in containers {
            let ip_address: String = client.container_ip_address(&container).await;
            assert!(ip_address.len()>0);
            if let Some(labels) = container.labels {
                for label in labels {
                    if label.0 == "cbl.dev.nginx" {
                        assert_eq!(label.1,container_label_value);
                    }
                }
            }
        }
    }
}