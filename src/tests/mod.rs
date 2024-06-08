//pub mod docker_tests;
//pub mod routes_tests;
//pub mod config_tests;
//pub mod server_tests;

#[test]
fn test_allowed_file_types(){
    use regex::Regex;
    let regex_pattern: &str = r"[0-9|_|-|A-Z|a-z]*\.(js|css)";
    let test_data: Vec<(&str,bool)> = Vec::from([
        ("/test.js",true),
        ("/app/static/hello.css",true),
        ("/app/static/hello.ca.ss",false),
        ("/app/static/hello",false),
        ("/app/static-files",false)
    ]);
    for item in test_data {
        let _regex = Regex::new(regex_pattern).unwrap();
        if let Some(matched) = _regex.find(item.0) {
            println!("Find={:?}",matched.as_str());
        }
        assert_eq!(_regex.is_match(item.0),item.1);
    }
}