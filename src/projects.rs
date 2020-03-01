use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct ProjectsRegistry {
    pub version: String,
    pub current_project: String,
    pub project: Vec<ChangeToProject>,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct ChangeToProject {
    pub name: String, // must preceed action or toml serialization fails
    pub action: Action,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct Action {
    pub file_or_dir: String,
}
impl ProjectsRegistry {
    pub fn new() -> Self {
        ProjectsRegistry {
            current_project: "".to_string(),
            project: vec![],
            version: "pjm1-0.1.0".to_string(),
        }
    }
    pub fn ser(&self) -> String {
        match toml::to_string(self) {
            Ok(s) => s,
            Err(e) => panic!("ser e={}", e),
        }
    }
    pub fn deser(s: String) -> Self {
        toml::from_str(&s).unwrap()
    }
}
