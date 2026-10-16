use serde::{Deserialize, Serialize};

/// Clasificación MVP (`OPEN-006` hipótesis: public | private).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DataClassification {
    Public,
    Private,
}
