
#[tokio::test]
async fn load_config(){
    use crate::config::Config;
    let config: Config = Config::new("config.toml".to_string());
    println!("{:#?}",config)
}