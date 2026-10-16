/// Configuración de inferencia en núcleo (ADR-0008 §2).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InferenceConfig {
    pub local_host: String,
    pub local_port: u16,
    pub cloud_fallback: bool,
}

impl Default for InferenceConfig {
    fn default() -> Self {
        Self {
            local_host: "127.0.0.1".into(),
            local_port: 11434,
            cloud_fallback: false,
        }
    }
}

impl InferenceConfig {
    pub fn loopback_base_url(&self) -> String {
        format!("http://{}:{}", self.local_host, self.local_port)
    }
}
