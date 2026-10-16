//! Entidades de dominio, clasificación, políticas y errores tipados (§12).

pub mod artifact;
pub mod classification;
pub mod error;
pub mod ids;
pub mod job;

pub use artifact::{validate_artifact_spec, ArtifactSpec, ArtifactType, EvidenceMode};
pub use classification::DataClassification;
pub use error::{AdapterError, IpcResult, ProductError, ProductErrorCode};
pub use ids::{ChunkId, NotebookId, SourceId, SourceVersionId};
pub use job::{JobErrorCode, JobId, JobRecord, JobState, JobType};
