//! Núcleo Tauri: comandos IPC tipados (stub). Sin plugins de red HTTP.

/// Respuesta mínima para comprobar el canal IPC desde la UI.
#[tauri::command]
fn ping() -> &'static str {
    "pong"
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![ping])
        .run(tauri::generate_context!())
        .expect("error al ejecutar la aplicación Tauri");
}

#[cfg(test)]
mod tests {
    #[test]
    fn ping_stub() {
        assert_eq!(super::ping(), "pong");
    }
}
