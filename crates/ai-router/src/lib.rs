//! Adaptadores de inferencia y autorización de egress a modelos.
//!
//! ## INVARIANTES (ADR-0006; recordatorio para implementación)
//!
//! - Motores locales (Ollama, llama.cpp, LM Studio): HTTP solo hacia `127.0.0.1`, nunca `0.0.0.0` ni LAN.
//! - `cloud_fallback = false` por defecto y aplicado en el backend; un fallo local no dispara envío automático a API remota.
//! - La UI (renderer) no ejecuta `fetch()` ni equivalentes hacia red; toda salida pasa por IPC al núcleo Rust.

pub mod adapter;
pub mod fake;
pub mod types;

pub use adapter::ModelAdapter;
pub use fake::FakeModelAdapter;
pub use types::*;
