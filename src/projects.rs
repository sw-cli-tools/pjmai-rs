use crate::{ProjectName, ProjectPath, SerializedConfig};
use log::info;
use serde_derive::{Deserialize, Serialize};
include!(concat!(env!("OUT_DIR"), "/generated.rs"));

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct ProjectsRegistry {
    pub version: String,
    pub current_project: ProjectName,
    pub project: Vec<ChangeToProject>,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct ChangeToProject {
    pub name: ProjectName, // must preceed action or toml serialization fails
    pub action: Action,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct Action {
    pub file_or_dir: ProjectPath,
}
impl ProjectsRegistry {
    pub fn new() -> Self {
        ProjectsRegistry {
            version: generated_version().to_string(),
            ..Default::default()
        }
    }
    pub fn ser(&self) -> SerializedConfig {
        info!("serialize");
        match toml::to_string(self) {
            Ok(s) => s,
            Err(e) => panic!("ser e={}", e),
        }
    }
    pub fn deser(s: SerializedConfig) -> Self {
        info!("deserialize");
        toml::from_str(&s).unwrap()
    }
}
