use crate::error::{PjmError, Result};
use crate::{ProjectName, ProjectPath, SerializedRegistry};
use log::info;
use serde::{Deserialize, Serialize};

include!(concat!(env!("OUT_DIR"), "/generated.rs"));

/// The project registry
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct ProjectsRegistry {
    /// The project version
    pub version: String,
    /// The currently active project, if any
    pub current_project: ProjectName,
    /// The known projects
    pub project: Vec<ChangeToProject>,
    /// Stack of previous projects for push/pop navigation
    #[serde(default)]
    pub stack: Vec<ProjectName>,
}

/// A project
#[derive(Debug, Deserialize, Serialize)]
pub struct ChangeToProject {
    /// The project name
    pub name: ProjectName, // must precede action or toml serialization fails
    /// The associated project action
    pub action: Action,
}

/// An action associated with a project
#[derive(Debug, Deserialize, Serialize)]
pub struct Action {
    /// A file to source or a directory to switch to
    pub file_or_dir: ProjectPath,
}

impl ProjectsRegistry {
    /// Create a new Registry with zero projects
    pub fn new() -> Self {
        ProjectsRegistry {
            version: generated_version().to_string(),
            ..Default::default()
        }
    }

    /// Serialize the registry
    pub fn ser(&self) -> Result<SerializedRegistry> {
        info!("serialize");
        toml::to_string(self).map_err(|e| PjmError::ConfigSerialize(e.to_string()))
    }

    /// Load a serialized registry
    pub fn deser(s: SerializedRegistry) -> Result<Self> {
        info!("deserialize");
        toml::from_str(&s).map_err(|e| PjmError::ConfigParse(e.to_string()))
    }
}
