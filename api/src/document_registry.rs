use crate::error::{ApiError, ApiResult};
use crate::DocumentMetadata;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentRegistry {
    pub agents: AgentDocuments,
    pub protocols: ProtocolDocuments,
    pub architecture: ArchitectureDocuments,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentDocuments {
    pub overview: Option<DocumentMetadata>,
    pub index: Option<DocumentMetadata>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProtocolDocuments {
    pub overview: Option<DocumentMetadata>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArchitectureDocuments {
    pub current: Option<DocumentMetadata>,
}

#[derive(Debug, Clone)]
pub enum DocumentType {
    AgentsOverview,
    Protocols,
    Architecture,
}

impl DocumentRegistry {
    pub fn new(base_path: &Path) -> ApiResult<Self> {
        let agents = AgentDocuments {
            overview: Self::find_document(base_path, &[
                "docs/agent-system/README.md",
                "AGENTS/README.md",
                "AgentGuide.md",
            ])?,
            index: Self::find_document(base_path, &[
                "docs/agent-system/agent-index.md",
                "AGENT_INDEX.md",
            ])?,
        };

        let protocols = ProtocolDocuments {
            overview: Self::find_document(base_path, &[
                "docs/protocols/README.md",
                "Protocols/README.md",
            ])?,
        };

        let architecture = ArchitectureDocuments {
            current: Self::find_document(base_path, &[
                "docs/architecture/current/README.md",
                "Architectures/CURRENT/CURRENT_ARCHITECTURE_ANALYSIS.md",
            ])?,
        };

        Ok(DocumentRegistry {
            agents,
            protocols,
            architecture,
        })
    }

    fn find_document(base_path: &Path, possible_paths: &[&str]) -> ApiResult<Option<DocumentMetadata>> {
        for path_str in possible_paths {
            let full_path = base_path.join(path_str);
            if full_path.exists() {
                let metadata = full_path.metadata()?;
                let last_modified = metadata
                    .modified()?
                    .duration_since(std::time::UNIX_EPOCH)
                    .map_err(|_| ApiError::InternalServerError("Invalid timestamp".to_string()))?;
                
                return Ok(Some(DocumentMetadata {
                    path: full_path.to_string_lossy().to_string(),
                    relative_path: path_str.to_string(),
                    last_modified: DateTime::from_timestamp(last_modified.as_secs() as i64, 0)
                        .unwrap_or_else(Utc::now),
                    size: metadata.len(),
                }));
            }
        }
        Ok(None)
    }

    pub fn get_document_metadata(&self, doc_type: DocumentType) -> Option<&DocumentMetadata> {
        match doc_type {
            DocumentType::AgentsOverview => self.agents.overview.as_ref(),
            DocumentType::Protocols => self.protocols.overview.as_ref(),
            DocumentType::Architecture => self.architecture.current.as_ref(),
        }
    }
}