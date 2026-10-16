use core_domain::AdapterError;

use crate::types::{
    ChatRequest, ChatResponse, EmbedRequest, EmbedResponse, ModelCapabilities, ModelDescriptor,
};

/// Contrato lógico `ModelAdapter` (ADR-0008).
pub trait ModelAdapter: Send + Sync {
    fn adapter_id(&self) -> &str;

    fn list_models(&self) -> Result<Vec<ModelDescriptor>, AdapterError>;

    fn describe_capabilities(&self, model_id: &str) -> Result<ModelCapabilities, AdapterError>;

    fn chat(&self, request: ChatRequest) -> Result<ChatResponse, AdapterError>;

    fn embed(&self, request: EmbedRequest) -> Result<EmbedResponse, AdapterError>;
}
