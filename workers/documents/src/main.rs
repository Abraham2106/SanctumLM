//! Sidecar de ingesta: un job JSON por stdin, respuesta `IpcResult` en stdout.

use documents_worker::ipc::run_stdio_sidecar;

fn main() {
    if let Err(err) = run_stdio_sidecar() {
        eprintln!("documents-worker: error de E/S: {err}");
        std::process::exit(2);
    }
}
