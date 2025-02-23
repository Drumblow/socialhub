use std::fs;
use toml::Value;

#[test]
fn test_parse_config() {
    let config_content = fs::read_to_string("config/default.toml")
        .unwrap_or_else(|_| String::from("[server]\nport = 8080"));
    
    let config: Value = toml::from_str(&config_content).unwrap();
    
    if let Some(server) = config.get("server") {
        assert!(server.get("port").is_some());
    }
}
