use super::*;
use mockito::{mock, server_url};  // Importação explícita de server_url
use tokio;

#[tokio::test]
async fn test_manifest_parsing() {
    let mock_manifest = r#"{
        "id": "org.myexampleaddon",
        "name": "Example Addon",
        "version": "1.0.0",
        "description": "Sample addon for testing",
        "resources": ["stream", "catalog"],
        "types": ["movie", "series"],
        "catalogs": []
    }"#;

    let _m = mock("GET", "/manifest.json")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(mock_manifest)
        .create();

    let adapter = StremioAddonAdapter::new(&format!("{}/manifest.json", server_url()))
        .await
        .expect("Failed to create adapter");

    assert_eq!(adapter.manifest.name, "Example Addon");
    assert_eq!(adapter.manifest.version, "1.0.0");
}

#[tokio::test]
async fn test_invalid_manifest() {
    let _m = mock("GET", "/manifest.json")
        .with_status(404)
        .create();

    let result = StremioAddonAdapter::new(&format!("{}/manifest.json", server_url())).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_get_catalog() {
    let mock_catalog = r#"{
        "metas": [{
            "id": "tt1254207",
            "type_name": "movie",
            "name": "Big Buck Bunny"
        }]
    }"#;

    // Corrigindo o path do mock
    let _m = mock("GET", "/catalog/movie/test.json")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(mock_catalog)
        .create();

    let adapter = create_test_adapter().await;
    let result = adapter.get_catalog("movie", "test").await;
    
    println!("Catalog result: {:?}", result);
    assert!(result.is_ok());
    let catalog = result.unwrap();
    assert_eq!(catalog.metas[0].id, "tt1254207");
}

#[tokio::test]
async fn test_get_streams() {
    let mock_streams = r#"{
        "streams": [{
            "url": "http://example.com/video.mp4"
        }]
    }"#;  // Simplificando o JSON para corresponder à struct

    // Mantendo o path correto
    let _m = mock("GET", "/stream/movie/tt1254207.json")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(mock_streams)
        .create();

    let adapter = create_test_adapter().await;
    let result = adapter.get_streams("movie", "tt1254207").await;
    
    println!("Streams result: {:?}", result);
    assert!(result.is_ok());
    let streams = result.unwrap();
    assert_eq!(streams[0].url, Some("http://example.com/video.mp4".to_string()));
}

#[tokio::test]
async fn test_search_catalog() {
    let mock_search_results = r#"{
        "metas": [{
            "id": "tt1254207",
            "type_name": "movie",
            "name": "Big Buck Bunny"
        }]
    }"#;

    let _m = mock("GET", "/catalog/movie/search=big%20buck%20bunny.json")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(mock_search_results)
        .create();

    let adapter = create_test_adapter().await;
    let result = adapter.search_catalog("movie", "big buck bunny").await;
    
    println!("Search result: {:?}", result);
    assert!(result.is_ok());
    let catalog = result.unwrap();
    assert_eq!(catalog.metas[0].id, "tt1254207");
    assert_eq!(catalog.metas[0].name, "Big Buck Bunny");
}

#[tokio::test]
async fn test_filter_catalog() {
    let mock_filter_results = r#"{
        "metas": [{
            "id": "tt1254207",
            "type_name": "movie",
            "name": "Big Buck Bunny"
        }]
    }"#;

    let _m = mock("GET", "/catalog/movie/catalog.json?genre=Animation")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(mock_filter_results)
        .create();

    let adapter = create_test_adapter().await;
    let mut filters = HashMap::new();
    filters.insert("genre".to_string(), "Animation".to_string());
    
    let result = adapter.filter_catalog("movie", &filters).await;
    
    println!("Filter result: {:?}", result);
    assert!(result.is_ok());
    let catalog = result.unwrap();
    assert_eq!(catalog.metas[0].id, "tt1254207");
    assert_eq!(catalog.metas[0].name, "Big Buck Bunny");
}

#[tokio::test]
async fn test_empty_search_results() {
    let mock_empty_results = r#"{"metas": []}"#;

    let _m = mock("GET", "/catalog/movie/search=nonexistent.json")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(mock_empty_results)
        .create();

    let adapter = create_test_adapter().await;
    let result = adapter.search_catalog("movie", "nonexistent").await;
    
    assert!(result.is_ok());
    let catalog = result.unwrap();
    assert!(catalog.metas.is_empty());
}

#[tokio::test]
async fn test_invalid_filter_response() {
    let _m = mock("GET", "/catalog/movie/catalog.json?genre=Invalid")
        .with_status(500)
        .create();

    let adapter = create_test_adapter().await;
    let mut filters = HashMap::new();
    filters.insert("genre".to_string(), "Invalid".to_string());
    
    let result = adapter.filter_catalog("movie", &filters).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_catalog_pagination() {
    let mock_catalog = r#"{
        "metas": [
            {
                "id": "tt1254207",
                "type_name": "movie",
                "name": "Big Buck Bunny"
            },
            {
                "id": "tt1254208",
                "type_name": "movie",
                "name": "Second Movie"
            }
        ]
    }"#;

    let _m = mock("GET", "/catalog/movie/test.json?skip=1&limit=1")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(mock_catalog)
        .create();

    let adapter = create_test_adapter().await;
    let result = adapter.get_catalog_with_pagination("movie", "test", Some(1), Some(1)).await;
    
    println!("Paginated catalog result: {:?}", result);
    assert!(result.is_ok());
    let catalog = result.unwrap();
    assert_eq!(catalog.metas.len(), 2);
    assert_eq!(catalog.metas[1].name, "Second Movie");
}

#[tokio::test]
async fn test_multiple_catalogs() {
    let mock_manifest = r#"{
        "id": "test.addon",
        "name": "Test Addon",
        "version": "1.0.0",
        "description": "Test addon",
        "resources": ["stream", "catalog"],
        "types": ["movie", "series"],
        "catalogs": [
            {
                "type_name": "movie",
                "id": "movieCatalog",
                "name": "Movies Catalog",
                "extra": [],
                "genres": ["Action", "Drama"]
            },
            {
                "type_name": "series",
                "id": "seriesCatalog",
                "name": "Series Catalog",
                "extra": [],
                "genres": ["Comedy", "Drama"]
            }
        ]
    }"#;

    let _m = mock("GET", "/manifest.json")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(mock_manifest)
        .create();

    let adapter = StremioAddonAdapter::new(&format!("{}/manifest.json", server_url()))
        .await
        .expect("Failed to create adapter");

    // Teste para catálogos de filmes
    let movie_catalogs = adapter.get_catalog_by_type("movie").await.unwrap();
    assert_eq!(movie_catalogs.len(), 1);
    assert_eq!(movie_catalogs[0].name, "Movies Catalog");
    assert_eq!(movie_catalogs[0].genres.as_ref().unwrap(), &vec!["Action".to_string(), "Drama".to_string()]);

    // Teste para catálogos de séries
    let series_catalogs = adapter.get_catalog_by_type("series").await.unwrap();
    assert_eq!(series_catalogs.len(), 1);
    assert_eq!(series_catalogs[0].name, "Series Catalog");

    // Teste para tipos suportados
    let supported_types = adapter.get_supported_types().await;
    assert_eq!(supported_types, vec!["movie".to_string(), "series".to_string()]);
}

// Helper function para criar um adapter para testes
async fn create_test_adapter() -> StremioAddonAdapter {
    let mock_manifest = r#"{
        "id": "test.addon",
        "name": "Test Addon",
        "version": "1.0.0",
        "description": "Test addon",
        "resources": ["stream", "catalog"],
        "types": ["movie"],
        "catalogs": []
    }"#;

    let _m = mock("GET", "/manifest.json")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(mock_manifest)
        .create();

    StremioAddonAdapter::new(&format!("{}/manifest.json", server_url()))
        .await
        .expect("Failed to create test adapter")
}
