use crate::error::{PjmError, Result};
use crate::{ProjectName, ProjectPath, SerializedRegistry};
use chrono;
use log::info;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ChangeToProject {
    /// The project name
    pub name: ProjectName, // must precede action or toml serialization fails
    /// The associated project action
    pub action: Action,
    /// Optional project metadata
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ProjectMetadata>,
}

/// Optional metadata for a project
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ProjectMetadata {
    /// Human-readable description of the project
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Tags for categorization (e.g., "rust", "web", "work")
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<String>,
    /// Primary programming language
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    /// Project group (e.g., "work", "personal", "oss")
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
    /// Last time this project was accessed (ISO 8601)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_used: Option<String>,
    /// Free-form notes about the project
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub notes: Vec<String>,
    /// Environment configuration for project entry
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub environment: Option<EnvironmentConfig>,
}

/// Environment configuration for a project
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EnvironmentConfig {
    /// Environment variables to set
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vars: Option<HashMap<String, String>>,
    /// Commands to run on project entry
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub on_enter: Option<Vec<String>>,
}

/// An action associated with a project
#[derive(Debug, Clone, Deserialize, Serialize)]
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

    /// Find a project by name
    pub fn find_project(&self, name: &str) -> Option<&ChangeToProject> {
        self.project.iter().find(|p| p.name == name)
    }

    /// Find a project by name (mutable)
    pub fn find_project_mut(&mut self, name: &str) -> Option<&mut ChangeToProject> {
        self.project.iter_mut().find(|p| p.name == name)
    }

    /// Update last_used timestamp for a project
    pub fn touch_project(&mut self, name: &str) {
        if let Some(proj) = self.find_project_mut(name) {
            let now = chrono::Utc::now().to_rfc3339();
            match &mut proj.metadata {
                Some(meta) => meta.last_used = Some(now),
                None => {
                    proj.metadata = Some(ProjectMetadata {
                        last_used: Some(now),
                        ..Default::default()
                    });
                }
            }
        }
    }

    /// Get projects sorted by last_used (most recent first)
    pub fn projects_by_recency(&self) -> Vec<&ChangeToProject> {
        let mut projects: Vec<_> = self.project.iter().collect();
        projects.sort_by(|a, b| {
            let a_time = a
                .metadata
                .as_ref()
                .and_then(|m| m.last_used.as_ref())
                .map(|s| s.as_str())
                .unwrap_or("");
            let b_time = b
                .metadata
                .as_ref()
                .and_then(|m| m.last_used.as_ref())
                .map(|s| s.as_str())
                .unwrap_or("");
            b_time.cmp(a_time) // Reverse order (most recent first)
        });
        projects
    }

    /// Get projects filtered by tag
    pub fn projects_with_tag(&self, tag: &str) -> Vec<&ChangeToProject> {
        self.project
            .iter()
            .filter(|p| {
                p.metadata
                    .as_ref()
                    .map(|m| m.tags.iter().any(|t| t.eq_ignore_ascii_case(tag)))
                    .unwrap_or(false)
            })
            .collect()
    }

    /// Get projects filtered by group
    pub fn projects_in_group(&self, group: &str) -> Vec<&ChangeToProject> {
        self.project
            .iter()
            .filter(|p| {
                p.metadata
                    .as_ref()
                    .and_then(|m| m.group.as_ref())
                    .map(|g| g.eq_ignore_ascii_case(group))
                    .unwrap_or(false)
            })
            .collect()
    }

    /// Get all unique tags across all projects
    pub fn all_tags(&self) -> Vec<String> {
        let mut tags: Vec<String> = self
            .project
            .iter()
            .filter_map(|p| p.metadata.as_ref())
            .flat_map(|m| m.tags.iter().cloned())
            .collect();
        tags.sort();
        tags.dedup();
        tags
    }

    /// Get all unique groups across all projects
    pub fn all_groups(&self) -> Vec<String> {
        let mut groups: Vec<String> = self
            .project
            .iter()
            .filter_map(|p| p.metadata.as_ref())
            .filter_map(|m| m.group.clone())
            .collect();
        groups.sort();
        groups.dedup();
        groups
    }
}
