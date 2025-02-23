use crate::error::AddonError;
use reqwest;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub mod cinemeta;  // Adicione esta linha no início do arquivo

#[derive(Debug, Serialize, Deserialize)]
pub struct StremioManifest {
    pub id: String,
    pub name: String,
    pub version: String,
    pub description: String,
    pub resources: Vec<String>,
    pub types: Vec<String>,
    pub catalogs: Vec<StreamioCatalog>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_prefixes: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub behavior_hints: Option<BehaviorHints>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]  // Adicionando Clone
pub struct StreamioCatalog {
    pub type_name: String,
    pub id: String,
    pub name: String,
    pub extra: Vec<ExtraProperty>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub genres: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip_steps: Option<Vec<usize>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]  // Adicionando Clone
pub struct ExtraProperty {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<Vec<String>>,
    pub is_required: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BehaviorHints {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configurable: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_required: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Catalog {
    pub metas: Vec<MetaItem>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MetaItem {
    pub id: String,
    pub type_name: String,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Stream {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_url: Option<String>,
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

pub struct StremioAddonAdapter {
    manifest: StremioManifest,
    base_url: String,
    client: reqwest::Client,
}

impl StremioAddonAdapter {
    pub async fn new(manifest_url: &str) -> Result<Self, AddonError> {
        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(5))
            .build()?;

        let manifest: StremioManifest = client
            .get(manifest_url)
            .send()
            .await?
            .json()
            .await
            .map_err(|e| AddonError::ManifestFetch(e))?;

        // Get base URL by removing everything after the last slash
        let base_url = manifest_url
            .rfind('/')
            .map(|i| &manifest_url[..i])
            .unwrap_or(manifest_url);

        Ok(Self {
            manifest,
            base_url: base_url.to_string(),
            client,
        })
    }

    pub async fn get_catalog(&self, type_name: &str, id: &str) -> Result<Catalog, AddonError> {
        let url = format!("{}/catalog/{}/{}.json", self.base_url, type_name, id);
        println!("Requesting catalog from: {}", url); // Debug log
        
        self.client
            .get(&url)
            .send()
            .await?
            .json()
            .await
            .map_err(|e| AddonError::CatalogFetch(e))
    }

    pub async fn get_streams(&self, type_name: &str, id: &str) -> Result<Vec<Stream>, AddonError> {
        let url = format!("{}/stream/{}/{}.json", self.base_url, type_name, id);
        println!("Requesting streams from: {}", url); // Debug log
        
        let response = self.client
            .get(&url)
            .send()
            .await?
            .json::<StreamResponse>()
            .await
            .map_err(|e| AddonError::StreamFetch(e))?;

        Ok(response.streams)
    }

    pub async fn search_catalog(
        &self, 
        catalog_id: &str, 
        query: &str
    ) -> Result<Catalog, AddonError> {
        let url = format!(
            "{}/catalog/{}/search={}.json", 
            self.base_url, 
            catalog_id, 
            urlencoding::encode(query)
        );
        
        self.client
            .get(&url)
            .send()
            .await?
            .json()
            .await
            .map_err(|e| AddonError::CatalogFetch(e))
    }

    pub async fn filter_catalog(
        &self,
        catalog_id: &str,
        filter: &HashMap<String, String>
    ) -> Result<Catalog, AddonError> {
        let query = filter
            .iter()
            .map(|(k, v)| format!("{}={}", k, v))
            .collect::<Vec<_>>()
            .join("&");

        let url = format!(
            "{}/catalog/{}/catalog.json?{}", 
            self.base_url, 
            catalog_id,
            query
        );

        self.client
            .get(&url)
            .send()
            .await?
            .json()
            .await
            .map_err(|e| AddonError::CatalogFetch(e))
    }

    pub async fn get_catalog_with_pagination(
        &self, 
        type_name: &str, 
        id: &str,
        skip: Option<usize>,
        limit: Option<usize>
    ) -> Result<Catalog, AddonError> {
        let mut url = format!("{}/catalog/{}/{}.json", self.base_url, type_name, id);
        
        if let Some(skip) = skip {
            url.push_str(&format!("?skip={}", skip));
        }
        
        if let Some(limit) = limit {
            url.push_str(&format!("{}", if skip.is_some() { "&" } else { "?" }));
            url.push_str(&format!("limit={}", limit));
        }

        println!("Requesting paginated catalog from: {}", url);
        
        self.client
            .get(&url)
            .send()
            .await?
            .json()
            .await
            .map_err(|e| AddonError::CatalogFetch(e))
    }

    pub async fn get_catalog_by_type(&self, catalog_type: &str) -> Result<Vec<StreamioCatalog>, AddonError> {
        let catalogs = self.manifest.catalogs.iter()
            .filter(|cat| cat.type_name == catalog_type)
            .cloned()
            .collect::<Vec<_>>();

        if catalogs.is_empty() {
            Err(AddonError::NotFound(format!("No catalogs found for type: {}", catalog_type)))
        } else {
            Ok(catalogs)
        }
    }

    pub async fn get_supported_types(&self) -> Vec<String> {
        self.manifest.types.clone()
    }

    pub async fn enrich_with_metadata(&self, meta_item: &mut MetaItem) -> Result<(), AddonError> {
        let cinemeta = cinemeta::CinemetaClient::new();
        if let Ok(metadata) = cinemeta.get_metadata(&meta_item.type_name, &meta_item.id).await {
            meta_item.name = metadata.name;
            // Adicione outros campos conforme necessário
        }
        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct StreamResponse {
    streams: Vec<Stream>
}

#[cfg(test)]
mod tests;
