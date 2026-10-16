use std::time::Duration;

use core_domain::AdapterError;
use reqwest::blocking::Client;
use reqwest::StatusCode;
use serde::Deserialize;

use crate::adapter::ModelAdapter;
use crate::config::InferenceConfig;
use crate::loopback::parse_loopback_http_base;
use crate::types::{
    ChatRequest, ChatResponse, ChatRole, EmbedRequest, EmbedResponse,
    ModelCapabilities, ModelDescriptor,
};

const ADAPTER_ID: &str = "ollama-loopback";

/// Adaptador HTTP a Ollama en loopback (`/api/tags`, `/api/chat`, `/api/embed`).
pub struct OllamaLoopbackAdapter {
    base_url: String,
    client: Client,
}

impl OllamaLoopbackAdapter {
    pub fn from_config(config: &InferenceConfig) -> Result<Self, AdapterError> {
        if config.cloud_fallback {
            return Err(AdapterError::PolicyDenied);
        }
        if config.local_host != "127.0.0.1" {
            return Err(AdapterError::PolicyDenied);
        }
        Self::with_base_url(&config.loopback_base_url())
    }

    pub fn with_base_url(base_url: &str) -> Result<Self, AdapterError> {
        let url = parse_loopback_http_base(base_url)?;
        let base_url = url.to_string();
        let client = Client::builder()
            .redirect(reqwest::redirect::Policy::none())
            .build()
            .map_err(|_| AdapterError::LocalUnavailable)?;
        Ok(Self { base_url, client })
    }

    fn endpoint(&self, path: &str) -> String {
        format!(
            "{}/{}",
            self.base_url.trim_end_matches('/'),
            path.trim_start_matches('/')
        )
    }

    fn request_timeout(timeout_ms: Option<u64>) -> Duration {
        Duration::from_millis(timeout_ms.unwrap_or(120_000).max(1))
    }

    fn map_transport(err: reqwest::Error) -> AdapterError {
        if err.is_timeout() {
            AdapterError::Timeout
        } else if err.is_connect() {
            AdapterError::LocalUnavailable
        } else {
            AdapterError::InvalidResponse
        }
    }

    fn infer_capabilities(model_id: &str) -> ModelCapabilities {
        let lower = model_id.to_ascii_lowercase();
        let embedding = lower.contains("embed");
        ModelCapabilities {
            chat: !embedding,
            embedding,
            json_schema: false,
            vision: false,
            image_generation: false,
            context_window: Some(8192),
            offline_available: true,
        }
    }
}

impl ModelAdapter for OllamaLoopbackAdapter {
    fn adapter_id(&self) -> &str {
        ADAPTER_ID
    }

    fn list_models(&self) -> Result<Vec<ModelDescriptor>, AdapterError> {
        #[derive(Deserialize)]
        struct TagsResponse {
            models: Vec<TagModel>,
        }
        #[derive(Deserialize)]
        struct TagModel {
            name: String,
        }

        let resp = self
            .client
            .get(self.endpoint("/api/tags"))
            .timeout(Self::request_timeout(None))
            .send()
            .map_err(Self::map_transport)?;

        if resp.status() == StatusCode::NOT_FOUND {
            return Err(AdapterError::InvalidResponse);
        }
        if !resp.status().is_success() {
            return Err(if resp.status().is_server_error() {
                AdapterError::LocalUnavailable
            } else {
                AdapterError::InvalidResponse
            });
        }

        let body: TagsResponse = resp.json().map_err(|_| AdapterError::InvalidResponse)?;
        Ok(body
            .models
            .into_iter()
            .map(|m| {
                let caps = Self::infer_capabilities(&m.name);
                ModelDescriptor {
                    model_id: m.name.clone(),
                    adapter_id: ADAPTER_ID.into(),
                    capabilities: caps,
                }
            })
            .collect())
    }

    fn describe_capabilities(&self, model_id: &str) -> Result<ModelCapabilities, AdapterError> {
        let models = self.list_models()?;
        models
            .into_iter()
            .find(|m| m.model_id == model_id)
            .map(|m| m.capabilities)
            .ok_or(AdapterError::ModelNotFound)
    }

    fn chat(&self, request: ChatRequest) -> Result<ChatResponse, AdapterError> {
        #[derive(serde::Serialize)]
        struct ChatBody<'a> {
            model: &'a str,
            messages: &'a [OllamaMessage<'a>],
            stream: bool,
        }
        #[derive(serde::Serialize)]
        struct OllamaMessage<'a> {
            role: &'a str,
            content: &'a str,
        }
        #[derive(Deserialize)]
        struct ChatResponseBody {
            message: Option<OllamaReply>,
        }
        #[derive(Deserialize)]
        struct OllamaReply {
            content: String,
        }

        let caps = self.describe_capabilities(&request.model_id)?;
        if !caps.chat {
            return Err(AdapterError::CapabilityUnsupported);
        }

        let messages: Vec<OllamaMessage<'_>> = request
            .messages
            .iter()
            .map(|m| OllamaMessage {
                role: match m.role {
                    ChatRole::System => "system",
                    ChatRole::User => "user",
                    ChatRole::Assistant => "assistant",
                },
                content: &m.content,
            })
            .collect();

        let body = ChatBody {
            model: &request.model_id,
            messages: &messages,
            stream: false,
        };

        let resp = self
            .client
            .post(self.endpoint("/api/chat"))
            .timeout(Self::request_timeout(request.timeout_ms))
            .json(&body)
            .send()
            .map_err(Self::map_transport)?;

        if resp.status() == StatusCode::NOT_FOUND {
            return Err(AdapterError::ModelNotFound);
        }
        if !resp.status().is_success() {
            return Err(if resp.status().is_server_error() || resp.status().is_client_error() {
                AdapterError::LocalUnavailable
            } else {
                AdapterError::InvalidResponse
            });
        }

        let parsed: ChatResponseBody = resp.json().map_err(|_| AdapterError::InvalidResponse)?;
        let content = parsed
            .message
            .map(|m| m.content)
            .ok_or(AdapterError::InvalidResponse)?;

        Ok(ChatResponse {
            content,
            model_id: request.model_id,
            adapter_id: ADAPTER_ID.into(),
        })
    }

    fn embed(&self, request: EmbedRequest) -> Result<EmbedResponse, AdapterError> {
        #[derive(serde::Serialize)]
        struct EmbedBody<'a> {
            model: &'a str,
            input: &'a [String],
        }
        #[derive(Deserialize)]
        struct EmbedResponseBody {
            embeddings: Vec<Vec<f32>>,
        }

        let caps = self.describe_capabilities(&request.model_id)?;
        if !caps.embedding {
            return Err(AdapterError::CapabilityUnsupported);
        }
        if request.texts.is_empty() {
            return Err(AdapterError::InvalidResponse);
        }

        let body = EmbedBody {
            model: &request.model_id,
            input: &request.texts,
        };

        let resp = self
            .client
            .post(self.endpoint("/api/embed"))
            .timeout(Self::request_timeout(request.timeout_ms))
            .json(&body)
            .send()
            .map_err(Self::map_transport)?;

        if resp.status() == StatusCode::NOT_FOUND {
            return Err(AdapterError::ModelNotFound);
        }
        if !resp.status().is_success() {
            return Err(AdapterError::LocalUnavailable);
        }

        let parsed: EmbedResponseBody = resp.json().map_err(|_| AdapterError::InvalidResponse)?;
        if parsed.embeddings.len() != request.texts.len() {
            return Err(AdapterError::InvalidResponse);
        }
        let dim = parsed.embeddings.first().map(|v| v.len()).unwrap_or(0);
        if dim == 0 || parsed.embeddings.iter().any(|v| v.len() != dim) {
            return Err(AdapterError::InvalidResponse);
        }

        Ok(EmbedResponse {
            vectors: parsed.embeddings,
            model_id: request.model_id,
            adapter_id: ADAPTER_ID.into(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::OnceLock;
    use std::thread;

    use crate::types::{ChatMessage, ChatRole};

    fn spawn_fake_ollama() -> (String, thread::JoinHandle<()>) {
        static NEXT: AtomicUsize = AtomicUsize::new(19_000);
        let port = NEXT.fetch_add(1, Ordering::Relaxed);
        let addr = format!("127.0.0.1:{port}");
        let server = tiny_http::Server::http(&addr).expect("bind loopback");

        let handle = thread::spawn(move || {
            for mut request in server.incoming_requests().take(32) {
                let mut body = String::new();
                let _ = request.as_reader().read_to_string(&mut body);
                let path = request.url().to_string();
                let (status, json) = if path.contains("/api/tags") {
                    (
                        200,
                        r#"{"models":[{"name":"llama3.2:latest"},{"name":"nomic-embed-text:latest"}]}"#,
                    )
                } else if path.contains("/api/chat") {
                    (
                        200,
                        r#"{"message":{"role":"assistant","content":"hola desde fake ollama"}}"#,
                    )
                } else if path.contains("/api/embed") {
                    (
                        200,
                        r#"{"embeddings":[[0.1,0.2,0.3],[0.4,0.5,0.6]]}"#,
                    )
                } else {
                    (404, r#"{"error":"not found"}"#)
                };
                let response = tiny_http::Response::from_string(json)
                    .with_status_code(status)
                    .with_header(
                        tiny_http::Header::from_bytes(&b"Content-Type"[..], &b"application/json"[..])
                            .unwrap(),
                    );
                let _ = request.respond(response);
            }
        });

        (format!("http://{addr}"), handle)
    }

    #[test]
    fn fake_http_list_chat_embed() {
        let (base, _handle) = spawn_fake_ollama();
        let adapter = OllamaLoopbackAdapter::with_base_url(&base).unwrap();

        let models = adapter.list_models().unwrap();
        assert_eq!(models.len(), 2);

        let chat = adapter
            .chat(ChatRequest {
                model_id: "llama3.2:latest".into(),
                messages: vec![ChatMessage {
                    role: ChatRole::User,
                    content: "hola".into(),
                }],
                timeout_ms: Some(5_000),
            })
            .unwrap();
        assert_eq!(chat.content, "hola desde fake ollama");

        let embed = adapter
            .embed(EmbedRequest {
                model_id: "nomic-embed-text:latest".into(),
                texts: vec!["a".into(), "b".into()],
                timeout_ms: Some(5_000),
            })
            .unwrap();
        assert_eq!(embed.vectors.len(), 2);
        assert_eq!(embed.vectors[0].len(), 3);
    }

    #[test]
    fn closed_port_is_local_unavailable() {
        let adapter =
            OllamaLoopbackAdapter::with_base_url("http://127.0.0.1:1").unwrap();
        let err = adapter.list_models().unwrap_err();
        assert_eq!(err, AdapterError::LocalUnavailable);
    }

    fn ollama_daemon_up() -> bool {
        static CACHE: OnceLock<bool> = OnceLock::new();
        *CACHE.get_or_init(|| {
            Client::builder()
                .timeout(Duration::from_millis(400))
                .build()
                .ok()
                .and_then(|c| c.get("http://127.0.0.1:11434/").send().ok())
                .map(|r| r.status().is_success())
                .unwrap_or(false)
        })
    }

    #[test]
    fn live_ollama_tags_not_run_without_daemon() {
        if !ollama_daemon_up() {
            eprintln!("not-run: Ollama daemon no disponible en 127.0.0.1:11434 (RSK-12)");
            return;
        }
        let adapter = OllamaLoopbackAdapter::from_config(&InferenceConfig::default()).unwrap();
        let _models = adapter.list_models().expect("daemon presente");
    }
}
