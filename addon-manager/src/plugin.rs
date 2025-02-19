use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Plugin {
    pub id: Uuid,
    pub name: String,
    pub version: String,
    pub enabled: bool,
    pub permissions: Vec<Permission>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Permission {
    ReadUser,
    WriteUser,
    ReadMedia,
    WriteMedia,
    ReadStream,
    WriteStream,
}

pub trait PluginInterface {
    fn initialize(&self) -> Result<(), Box<dyn std::error::Error>>;
    fn shutdown(&self) -> Result<(), Box<dyn std::error::Error>>;
    fn get_manifest(&self) -> Plugin;
}
