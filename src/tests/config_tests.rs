
#[tokio::test]
async fn load_config(){
    use crate::config::Config;
    let _: Config = Config::new("config.toml".to_string());
    assert!(true)
}