use serde::{Deserialize, Serialize};

/// Errores expuestos al renderer vía IPC (`DATA_AND_INTERFACES` § ProductError).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ProductErrorCode {
    ValidationFailed,
    UnsupportedApiVersion,
    PolicyDenied,
    NotFound,
    Conflict,
    JobCancelled,
    LocalUnavailable,
    SsrBlocked,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProductError {
    pub code: ProductErrorCode,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<serde_json::Value>,
}

impl ProductError {
    pub fn new(code: ProductErrorCode, message: impl Into<String>) -> Self {
        Self {
            code,
            message: message.into(),
            details: None,
        }
    }

    pub fn with_details(mut self, details: serde_json::Value) -> Self {
        self.details = Some(details);
        self
    }

    pub fn not_found(resource: &str) -> Self {
        Self::new(
            ProductErrorCode::NotFound,
            format!("{resource} no encontrado"),
        )
    }
}

/// Errores del contrato `ModelAdapter` (ADR-0008).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum AdapterError {
    LocalUnavailable,
    ModelNotFound,
    CapabilityUnsupported,
    Timeout,
    Cancelled,
    PolicyDenied,
    RemoteUnauthorized,
    InvalidResponse,
    RateLimited,
}

/// Envelope IPC: `{ ok: true, data }` | `{ ok: false, error }`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IpcResult<T> {
    pub ok: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<ProductError>,
}

impl<T> IpcResult<T> {
    pub fn ok(data: T) -> Self {
        Self {
            ok: true,
            data: Some(data),
            error: None,
        }
    }

    pub fn err(error: ProductError) -> Self {
        Self {
            ok: false,
            data: None,
            error: Some(error),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ipc_ok_serializes_shape() {
        let raw = serde_json::to_value(IpcResult::ok("pong")).unwrap();
        assert_eq!(raw["ok"], true);
        assert_eq!(raw["data"], "pong");
        assert!(raw.get("error").is_none());
    }
}
