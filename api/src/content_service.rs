use crate::error::{ApiError, ApiResult};
use crate::document_registry::{DocumentRegistry, DocumentType};
use crate::{AgentInfo, DocumentMetadata};
use serde_json::Value;
use std::fs;
use std::path::PathBuf;

pub struct ContentService {
    base_path: PathBuf,
    registry: DocumentRegistry,
}

impl ContentService {
    pub fn new(base_path: PathBuf) -> ApiResult<Self> {
        let registry = DocumentRegistry::new(&base_path)?;
        Ok(ContentService { base_path, registry })
    }

    pub async fn get_agent_list(&self) -> ApiResult<Vec<AgentInfo>> {
        let agents_dir = self.base_path.join("AGENTS");
        if !agents_dir.exists() {
            return Ok(Vec::new());
        }

        let mut agents = Vec::new();
        let entries = fs::read_dir(&agents_dir)?;

        for entry in entries {
            let entry = entry?;
            let path = entry.path();
            
            if !path.is_dir() || path.file_name().unwrap().to_string_lossy().starts_with('.') {
                continue;
            }

            let agent_name = path.file_name().unwrap().to_string_lossy().to_string();
            let readme_path = path.join("README.md");
            let metadata_path = path.join("metadata.json");

            let mut agent_info = AgentInfo {
                name: agent_name,
                path: path.to_string_lossy().to_string(),
                status: if readme_path.exists() { "active".to_string() } else { "inactive".to_string() },
                has_readme: readme_path.exists(),
                has_metadata: metadata_path.exists(),
                description: None,
            };

            // Load metadata if available
            if metadata_path.exists() {
                if let Ok(metadata_content) = fs::read_to_string(&metadata_path) {
                    if let Ok(metadata_json) = serde_json::from_str::<Value>(&metadata_content) {
                        if let Some(desc) = metadata_json.get("description").and_then(|d| d.as_str()) {
                            agent_info.description = Some(desc.to_string());
                        }
                    }
                }
            }

            // Extract description from README if available and no metadata description
            if agent_info.description.is_none() && readme_path.exists() {
                if let Ok(readme_content) = fs::read_to_string(&readme_path) {
                    let lines: Vec<&str> = readme_content
                        .lines()
                        .map(|line| line.trim())
                        .filter(|line| !line.is_empty())
                        .collect();

                    for line in lines {
                        if !line.starts_with('#') && line.len() > 10 {
                            let description = if line.len() > 100 {
                                format!("{}...", &line[..100])
                            } else {
                                line.to_string()
                            };
                            agent_info.description = Some(description);
                            break;
                        }
                    }
                }
            }

            agents.push(agent_info);
        }

        // Sort by name
        agents.sort_by(|a, b| a.name.cmp(&b.name));
        Ok(agents)
    }

    pub async fn get_agent_readme(&self, agent_name: &str) -> ApiResult<String> {
        let readme_path = self.base_path.join("AGENTS").join(agent_name).join("README.md");
        if !readme_path.exists() {
            return Err(ApiError::NotFound(format!("README for agent '{}' not found", agent_name)));
        }
        Ok(fs::read_to_string(readme_path)?)
    }

    pub async fn get_document_content(&self, doc_type: DocumentType) -> ApiResult<(String, DocumentMetadata)> {
        let metadata = self.registry
            .get_document_metadata(doc_type)
            .ok_or_else(|| ApiError::NotFound("Document not found".to_string()))?;

        let content = fs::read_to_string(&metadata.path)?;
        Ok((content, metadata.clone()))
    }

    pub async fn get_document_registry(&self) -> ApiResult<&DocumentRegistry> {
        Ok(&self.registry)
    }
}