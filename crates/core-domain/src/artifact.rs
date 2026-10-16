use serde::{Deserialize, Serialize};

use crate::error::{ProductError, ProductErrorCode};

/// Claves prohibidas en `ArtifactSpec` (ADR-0010, INV-11).
const FORBIDDEN_SPEC_KEYS: &[&str] = &[
    "skip_security_checks",
    "bypass_policy",
    "raw_credentials",
    "credentials",
    "oauth_token",
    "api_token",
    "password",
];

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ArtifactType {
    Infographic,
    Slides,
    Report,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EvidenceMode {
    SourceBacked,
    Exploratory,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ArtifactSection {
    pub id: String,
    pub heading: String,
    #[serde(default)]
    pub body: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ArtifactSpec {
    pub schema_version: u32,
    #[serde(rename = "type")]
    pub artifact_type: ArtifactType,
    pub title: String,
    pub language: String,
    pub evidence_mode: EvidenceMode,
    #[serde(default)]
    pub source_scope: Vec<String>,
    pub sections: Vec<ArtifactSection>,
}

/// Valida el spec tipado y rechaza claves prohibidas si el JSON de entrada las incluye.
pub fn validate_artifact_spec(raw: &serde_json::Value) -> Result<ArtifactSpec, ProductError> {
    if let Some(obj) = raw.as_object() {
        for key in obj.keys() {
            if FORBIDDEN_SPEC_KEYS
                .iter()
                .any(|f| f.eq_ignore_ascii_case(key))
            {
                return Err(ProductError::new(
                    ProductErrorCode::PolicyDenied,
                    "ArtifactSpec contiene campos prohibidos",
                )
                .with_details(serde_json::json!({ "field": key })));
            }
        }
    }

    serde_json::from_value(raw.clone()).map_err(|e| {
        ProductError::new(
            ProductErrorCode::ValidationFailed,
            format!("ArtifactSpec inválido: {e}"),
        )
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_skip_security_checks() {
        let raw = serde_json::json!({
            "schema_version": 1,
            "type": "infographic",
            "title": "T",
            "language": "es",
            "evidence_mode": "source_backed",
            "sections": [{ "id": "s1", "heading": "H" }],
            "skip_security_checks": true
        });
        let err = validate_artifact_spec(&raw).unwrap_err();
        assert_eq!(err.code, ProductErrorCode::PolicyDenied);
    }

    #[test]
    fn accepts_minimal_infographic() {
        let raw = serde_json::json!({
            "schema_version": 1,
            "type": "infographic",
            "title": "Tema",
            "language": "es",
            "evidence_mode": "source_backed",
            "source_scope": ["src-1"],
            "sections": [{ "id": "s1", "heading": "Paso 1", "body": "…" }]
        });
        let spec = validate_artifact_spec(&raw).unwrap();
        assert_eq!(spec.artifact_type, ArtifactType::Infographic);
    }
}
