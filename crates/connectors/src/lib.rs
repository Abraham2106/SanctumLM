//! Conectores web, OAuth y proveedores de archivos.
//!
//! ## INVARIANTES (ADR-0006, R-PROD-111; recordatorio para implementación)
//!
//! - Egress y OAuth se autorizan en el núcleo Rust; el LLM no decide permisos de red.
//! - Tokens y credenciales solo en keychain del SO; nunca en SQLite ni logs.
//! - La UI no usa `fetch()` hacia proveedores externos; solo comandos IPC tipados.
//! - Adaptadores a motores locales: destino `127.0.0.1` únicamente; `cloud_fallback = false` en backend.

#[cfg(test)]
mod tests {
    #[test]
    fn workspace_smoke() {
        assert!(true);
    }
}
