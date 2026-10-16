use core_domain::AdapterError;

use crate::adapter::ModelAdapter;
use crate::types::{
    ChatRequest, ChatResponse, EmbedRequest, EmbedResponse, ModelCapabilities, ModelDescriptor,
};

/// Adaptador in-memory para tests de contrato (`OPEN-005`).
pub struct FakeModelAdapter {
    adapter_id: String,
    models: Vec<ModelDescriptor>,
    embed_dim: usize,
}

impl FakeModelAdapter {
    pub fn ollama_loopback_stub() -> Self {
        let caps = ModelCapabilities {
            chat: true,
            embedding: true,
            json_schema: false,
            vision: false,
            image_generation: false,
            context_window: Some(8192),
            offline_available: true,
        };
        Self {
            adapter_id: "ollama-loopback".into(),
            models: vec![ModelDescriptor {
                model_id: "llama3.2:latest".into(),
                adapter_id: "ollama-loopback".into(),
                capabilities: caps,
            }],
            embed_dim: 4,
        }
    }
}

impl ModelAdapter for FakeModelAdapter {
    fn adapter_id(&self) -> &str {
        &self.adapter_id
    }

    fn list_models(&self) -> Result<Vec<ModelDescriptor>, AdapterError> {
        Ok(self.models.clone())
    }

    fn describe_capabilities(&self, model_id: &str) -> Result<ModelCapabilities, AdapterError> {
        self.models
            .iter()
            .find(|m| m.model_id == model_id)
            .map(|m| m.capabilities.clone())
            .ok_or(AdapterError::ModelNotFound)
    }

    fn chat(&self, request: ChatRequest) -> Result<ChatResponse, AdapterError> {
        if !self.models.iter().any(|m| m.model_id == request.model_id) {
            return Err(AdapterError::ModelNotFound);
        }
        let last_user = request
            .messages
            .iter()
            .rev()
            .find(|m| matches!(m.role, crate::types::ChatRole::User))
            .map(|m| m.content.as_str())
            .unwrap_or("");
        Ok(ChatResponse {
            content: format!("fake-echo:{last_user}"),
            model_id: request.model_id,
            adapter_id: self.adapter_id.clone(),
        })
    }

    fn embed(&self, request: EmbedRequest) -> Result<EmbedResponse, AdapterError> {
        if !self.models.iter().any(|m| m.model_id == request.model_id) {
            return Err(AdapterError::ModelNotFound);
        }
        let caps = self.describe_capabilities(&request.model_id)?;
        if !caps.embedding {
            return Err(AdapterError::CapabilityUnsupported);
        }
        let vectors: Vec<Vec<f32>> = request
            .texts
            .iter()
            .map(|t| {
                (0..self.embed_dim)
                    .map(|i| (t.len() as f32 * 0.1) + i as f32)
                    .collect()
            })
            .collect();
        Ok(EmbedResponse {
            vectors,
            model_id: request.model_id,
            adapter_id: self.adapter_id.clone(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::ChatMessage;
    use crate::types::ChatRole;

    #[test]
    fn fake_chat_echoes_user() {
        let adapter = FakeModelAdapter::ollama_loopback_stub();
        let resp = adapter
            .chat(ChatRequest {
                model_id: "llama3.2:latest".into(),
                messages: vec![ChatMessage {
                    role: ChatRole::User,
                    content: "hola".into(),
                }],
                timeout_ms: None,
            })
            .unwrap();
        assert_eq!(resp.content, "fake-echo:hola");
    }

    #[test]
    fn fake_embed_uniform_dimension() {
        let adapter = FakeModelAdapter::ollama_loopback_stub();
        let resp = adapter
            .embed(EmbedRequest {
                model_id: "llama3.2:latest".into(),
                texts: vec!["a".into(), "bb".into()],
                timeout_ms: None,
            })
            .unwrap();
        assert_eq!(resp.vectors.len(), 2);
        assert!(resp.vectors.iter().all(|v| v.len() == 4));
    }
}
