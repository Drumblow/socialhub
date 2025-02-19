use std::fs;
use toml::Value;

#[test]
fn test_config_file_validity() {
    let config_content = fs::read_to_string("config/default.toml").unwrap();
    let config: Value = toml::from_str(&config_content).unwrap();
    
    // Verificar seções obrigatórias
    assert!(config.get("server").is_some());
    assert!(config.get("database").is_some());
    assert!(config.get("cache").is_some());
    assert!(config.get("media").is_some());
    assert!(config.get("auth").is_some());

    // Verificar valores específicos
    let server = config.get("server").unwrap();
    assert_eq!(server.get("port").unwrap().as_integer().unwrap(), 8080);
    
    let media = config.get("media").unwrap();
    assert_eq!(
        media.get("max_file_size").unwrap().as_integer().unwrap(), 
        10485760
    );
}
