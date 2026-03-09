use crate::args::VERSION;
use crate::error::{PjmError, Result};
use crate::{ProjectName, ProjectPath, SerializedRegistry};
use chrono;
use log::info;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;

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
    /// Navigation history (most recent last)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub history: Vec<ProjectName>,
    /// Group aliases (group_name -> alias)
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub group_aliases: HashMap<String, String>,
}

/// An inferred group (computed at runtime, not stored)
#[derive(Debug, Clone)]
pub struct InferredGroup {
    /// Group name (parent directory name)
    pub name: String,
    /// Optional alias for the group
    pub alias: Option<String>,
    /// Path to the group directory
    pub path: String,
    /// Project names in this group
    pub projects: Vec<String>,
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
    /// Commands to run when leaving the project
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub on_exit: Option<Vec<String>>,
    /// Paths to prepend to PATH
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path_prepend: Option<Vec<String>>,
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
            version: format!("pjmai-{}", VERSION),
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

    /// Record a project navigation in history (capped at 50 entries)
    pub fn record_history(&mut self, name: &str) {
        const MAX_HISTORY: usize = 50;
        self.history.push(name.to_string());
        if self.history.len() > MAX_HISTORY {
            let drain = self.history.len() - MAX_HISTORY;
            self.history.drain(..drain);
        }
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

    /// Get projects filtered by language (case-insensitive, matches any component in "rust+python")
    pub fn projects_with_language(&self, lang: &str) -> Vec<&ChangeToProject> {
        let query = lang.to_lowercase();
        self.project
            .iter()
            .filter(|p| {
                p.metadata
                    .as_ref()
                    .and_then(|m| m.language.as_ref())
                    .map(|l| {
                        l.to_lowercase()
                            .split('+')
                            .any(|component| component == query)
                    })
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

    /// Get all unique groups across all projects (legacy - uses metadata.group)
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

    /// Infer group from a project path (parent directory name)
    pub fn infer_group_from_path(path: &str) -> Option<(String, String)> {
        let expanded = crate::util::expand_file_path(path);
        let path = Path::new(&expanded);
        let parent = path.parent()?;
        let group_name = parent.file_name()?.to_str()?;
        let group_path = parent.to_str()?;
        Some((group_name.to_string(), group_path.to_string()))
    }

    /// Get the group for a project (explicit metadata.group or inferred from path)
    pub fn get_project_group(&self, project: &ChangeToProject) -> Option<(String, String)> {
        // First check explicit group in metadata
        if let Some(ref meta) = project.metadata
            && let Some(ref group) = meta.group
        {
            // For explicit groups, we don't have a path
            return Some((group.clone(), String::new()));
        }
        // Otherwise infer from path
        Self::infer_group_from_path(&project.action.file_or_dir)
    }

    /// Get all inferred groups with their projects
    pub fn get_inferred_groups(&self) -> Vec<InferredGroup> {
        let mut groups_map: HashMap<String, InferredGroup> = HashMap::new();

        for project in &self.project {
            if let Some((group_name, group_path)) = self.get_project_group(project) {
                let entry = groups_map
                    .entry(group_name.clone())
                    .or_insert_with(|| InferredGroup {
                        name: group_name.clone(),
                        alias: self.group_aliases.get(&group_name).cloned(),
                        path: group_path,
                        projects: Vec::new(),
                    });
                entry.projects.push(project.name.clone());
            }
        }

        let mut groups: Vec<_> = groups_map.into_values().collect();
        groups.sort_by(|a, b| a.name.cmp(&b.name));
        groups
    }

    /// Get current group (based on current project)
    pub fn get_current_group(&self) -> Option<InferredGroup> {
        if self.current_project.is_empty() {
            return None;
        }
        let project = self.find_project(&self.current_project)?;
        let (group_name, group_path) = self.get_project_group(project)?;

        // Find all projects in this group
        let projects: Vec<String> = self
            .project
            .iter()
            .filter(|p| {
                self.get_project_group(p)
                    .map(|(name, _)| name == group_name)
                    .unwrap_or(false)
            })
            .map(|p| p.name.clone())
            .collect();

        Some(InferredGroup {
            name: group_name.clone(),
            alias: self.group_aliases.get(&group_name).cloned(),
            path: group_path,
            projects,
        })
    }

    /// Find a group by name or alias
    pub fn find_group(&self, name_or_alias: &str) -> Option<InferredGroup> {
        let groups = self.get_inferred_groups();

        // Try exact name match first
        if let Some(group) = groups.iter().find(|g| g.name == name_or_alias) {
            return Some(group.clone());
        }

        // Try alias match
        groups
            .into_iter()
            .find(|g| g.alias.as_deref() == Some(name_or_alias))
    }

    /// Resolve "." to current group name, or return the input
    pub fn resolve_group_name(&self, name_or_alias: &str) -> Option<String> {
        if name_or_alias == "." {
            self.get_current_group().map(|g| g.name)
        } else {
            self.find_group(name_or_alias).map(|g| g.name)
        }
    }

    /// Get projects in a group (by name or alias, "." for current group)
    pub fn projects_in_inferred_group(&self, name_or_alias: &str) -> Vec<&ChangeToProject> {
        // Handle "." as current group
        let group = if name_or_alias == "." {
            match self.get_current_group() {
                Some(g) => g,
                None => return Vec::new(),
            }
        } else {
            match self.find_group(name_or_alias) {
                Some(g) => g,
                None => return Vec::new(),
            }
        };

        self.project
            .iter()
            .filter(|p| {
                self.get_project_group(p)
                    .map(|(name, _)| name == group.name)
                    .unwrap_or(false)
            })
            .collect()
    }
}
