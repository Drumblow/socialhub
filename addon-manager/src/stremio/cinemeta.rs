use crate::error::AddonError;
use serde::{Deserialize, Serialize};
use reqwest;

const CINEMETA_URL: &str = "https://v3-cinemeta.strem.io";

#[derive(Debug, Serialize, Deserialize)]
pub struct CinemetaMetadata {
    pub id: String,
    pub type_name: String,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub year: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub poster: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub runtime: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub genres: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
struct CinemetaResponse {
    meta: CinemetaMetadata,
}

pub struct CinemetaClient {
    client: reqwest::Client,
}

impl CinemetaClient {
    #[cfg(not(test))]
    const BASE_URL: &'static str = "https://v3-cinemeta.strem.io";
    
    #[cfg(test)]
    fn base_url() -> String {
        mockito::server_url()
    }

    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
        }
    }

    pub async fn get_metadata(&self, type_name: &str, imdb_id: &str) -> Result<CinemetaMetadata, AddonError> {
        #[cfg(not(test))]
        let url = format!("{}/meta/{}/{}.json", Self::BASE_URL, type_name, imdb_id);
        
        #[cfg(test)]
        let url = format!("{}/meta/{}/{}.json", Self::base_url(), type_name, imdb_id);
        
        println!("Requesting metadata from: {}", url);

        let response = self.client
            .get(&url)
            .send()
            .await?
            .json::<CinemetaResponse>()
            .await
            .map_err(|e| AddonError::ManifestFetch(e))?;

        Ok(response.meta)
    }

    pub async fn get_episode_metadata(
        &self,
        imdb_id: &str,
        season: u32,
        episode: u32
    ) -> Result<CinemetaMetadata, AddonError> {
        let video_id = format!("{}:{}:{}", imdb_id, season, episode);
        self.get_metadata("series", &video_id).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockito::{mock, server_url};
    use tokio;

    #[tokio::test]
    async fn test_get_movie_metadata() {
        let mock_response = r#"{
            "meta": {
                "id": "tt1254207",
                "type_name": "movie",
                "name": "Big Buck Bunny",
                "year": 2008,
                "genres": ["Animation"],
                "description": "Big Buck Bunny tells the story of a giant rabbit"
            }
        }"#;

        let _m = mock("GET", "/meta/movie/tt1254207.json")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(mock_response)
            .create();

        let client = CinemetaClient::new();
        let result = client.get_metadata("movie", "tt1254207").await;
        
        println!("Movie metadata result: {:?}", result);
        assert!(result.is_ok());
        let metadata = result.unwrap();
        assert_eq!(metadata.name, "Big Buck Bunny");
        assert_eq!(metadata.type_name, "movie");
    }

    #[tokio::test]
    async fn test_get_series_metadata() {
        let mock_response = r#"{
            "meta": {
                "id": "tt0108778",
                "type_name": "series",
                "name": "Friends",
                "genres": ["Comedy"],
                "description": "Six friends living in New York city"
            }
        }"#;

        let _m = mock("GET", "/meta/series/tt0108778.json")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(mock_response)
            .create();

        let client = CinemetaClient::new();
        let result = client.get_metadata("series", "tt0108778").await;
        
        println!("Series metadata result: {:?}", result);
        assert!(result.is_ok());
        let metadata = result.unwrap();
        assert_eq!(metadata.type_name, "series");
    }

    #[tokio::test]
    async fn test_get_episode_metadata() {
        let mock_response = r#"{
            "meta": {
                "id": "tt0108778:1:1",
                "type_name": "series",
                "name": "Friends - S01E01",
                "genres": ["Comedy"],
                "description": "The pilot episode"
            }
        }"#;

        let _m = mock("GET", "/meta/series/tt0108778:1:1.json")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(mock_response)
            .create();

        let client = CinemetaClient::new();
        let result = client.get_episode_metadata("tt0108778", 1, 1).await;
        
        println!("Episode metadata result: {:?}", result);
        assert!(result.is_ok());
        let metadata = result.unwrap();
        assert!(metadata.id.contains(":1:1"));
    }
}
