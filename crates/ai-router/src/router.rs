use core_domain::AdapterError;

use crate::adapter::ModelAdapter;
use crate::types::{
    ChatRequest, ChatResponse, EmbedRequest, EmbedResponse, ModelCapabilities, ModelDescriptor,
};

/// Enrutador de inferencia: local primero; `cloud_fallback` permanece desactivado (ADR-0008).
pub struct ModelRouter {
    cloud_fallback: bool,
    local: Box<dyn ModelAdapter>,
    remote: Option<Box<dyn ModelAdapter>>,
}

impl ModelRouter {
    /// Router MVP: solo adaptador local; sin fallback remoto.
    pub fn local_only(local: impl ModelAdapter + 'static) -> Self {
        Self {
            cloud_fallback: false,
            local: Box::new(local),
            remote: None,
        }
    }

    /// Constructor explícito para pruebas adversariales (remote registrado pero inalcanzable si fallback off).
    pub fn new(local: impl ModelAdapter + 'static, remote: Option<Box<dyn ModelAdapter>>) -> Self {
        Self {
            cloud_fallback: false,
            local: Box::new(local),
            remote,
        }
    }

    pub fn cloud_fallback(&self) -> bool {
        self.cloud_fallback
    }

    pub fn list_models(&self) -> Result<Vec<ModelDescriptor>, AdapterError> {
        self.local.list_models()
    }

    pub fn describe_capabilities(&self, model_id: &str) -> Result<ModelCapabilities, AdapterError> {
        match self.local.describe_capabilities(model_id) {
            Ok(caps) => Ok(caps),
            Err(_err) if self.cloud_fallback && self.remote.is_some() => {
                self.remote.as_ref().unwrap().describe_capabilities(model_id)
            }
            Err(err) => Err(err),
        }
    }

    pub fn chat(&self, request: ChatRequest) -> Result<ChatResponse, AdapterError> {
        match self.local.chat(request.clone()) {
            Ok(resp) => Ok(resp),
            Err(_err) if self.cloud_fallback && self.remote.is_some() => {
                self.remote.as_ref().unwrap().chat(request)
            }
            Err(err) => Err(err),
        }
    }

    pub fn embed(&self, request: EmbedRequest) -> Result<EmbedResponse, AdapterError> {
        match self.local.embed(request.clone()) {
            Ok(resp) => Ok(resp),
            Err(_err) if self.cloud_fallback && self.remote.is_some() => {
                self.remote.as_ref().unwrap().embed(request)
            }
            Err(err) => Err(err),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::Arc;

    use crate::fake::FakeModelAdapter;
    use crate::types::{ChatMessage, ChatRole};

    struct FailingLocalAdapter;

    impl ModelAdapter for FailingLocalAdapter {
        fn adapter_id(&self) -> &str {
            "failing-local"
        }

        fn list_models(&self) -> Result<Vec<ModelDescriptor>, AdapterError> {
            Err(AdapterError::LocalUnavailable)
        }

        fn describe_capabilities(&self, _: &str) -> Result<ModelCapabilities, AdapterError> {
            Err(AdapterError::LocalUnavailable)
        }

        fn chat(&self, _: ChatRequest) -> Result<ChatResponse, AdapterError> {
            Err(AdapterError::LocalUnavailable)
        }

        fn embed(&self, _: EmbedRequest) -> Result<EmbedResponse, AdapterError> {
            Err(AdapterError::LocalUnavailable)
        }
    }

    struct CountingRemoteAdapter {
        calls: Arc<AtomicUsize>,
    }

    impl ModelAdapter for CountingRemoteAdapter {
        fn adapter_id(&self) -> &str {
            "fake-remote"
        }

        fn list_models(&self) -> Result<Vec<ModelDescriptor>, AdapterError> {
            self.calls.fetch_add(1, Ordering::SeqCst);
            Ok(vec![])
        }

        fn describe_capabilities(&self, _: &str) -> Result<ModelCapabilities, AdapterError> {
            self.calls.fetch_add(1, Ordering::SeqCst);
            Err(AdapterError::ModelNotFound)
        }

        fn chat(&self, request: ChatRequest) -> Result<ChatResponse, AdapterError> {
            self.calls.fetch_add(1, Ordering::SeqCst);
            Ok(ChatResponse {
                content: "remote".into(),
                model_id: request.model_id,
                adapter_id: self.adapter_id().into(),
            })
        }

        fn embed(&self, _: EmbedRequest) -> Result<EmbedResponse, AdapterError> {
            self.calls.fetch_add(1, Ordering::SeqCst);
            Err(AdapterError::CapabilityUnsupported)
        }
    }

    #[test]
    fn cloud_fallback_disabled_by_default() {
        let router = ModelRouter::local_only(FakeModelAdapter::ollama_loopback_stub());
        assert!(!router.cloud_fallback());
    }

    #[test]
    fn local_failure_does_not_invoke_remote() {
        let calls = Arc::new(AtomicUsize::new(0));
        let remote = CountingRemoteAdapter {
            calls: Arc::clone(&calls),
        };
        let router = ModelRouter::new(FailingLocalAdapter, Some(Box::new(remote)));

        let err = router
            .chat(ChatRequest {
                model_id: "any".into(),
                messages: vec![ChatMessage {
                    role: ChatRole::User,
                    content: "x".into(),
                }],
                timeout_ms: None,
            })
            .unwrap_err();
        assert_eq!(err, AdapterError::LocalUnavailable);
        assert_eq!(calls.load(Ordering::SeqCst), 0);
    }

    #[test]
    fn routes_to_local_fake_when_healthy() {
        let router = ModelRouter::local_only(FakeModelAdapter::ollama_loopback_stub());
        let resp = router
            .chat(ChatRequest {
                model_id: "llama3.2:latest".into(),
                messages: vec![ChatMessage {
                    role: ChatRole::User,
                    content: "ping".into(),
                }],
                timeout_ms: None,
            })
            .unwrap();
        assert_eq!(resp.content, "fake-echo:ping");
    }
}
