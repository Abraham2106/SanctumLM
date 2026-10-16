/** Detecta el webview embebido de Tauri (no aplica en Vite/Vitest en navegador). */
export function isTauriRuntime(): boolean {
  if (typeof window === "undefined") {
    return false;
  }
  const w = window as Window & {
    __TAURI__?: unknown;
    __TAURI_INTERNALS__?: unknown;
  };
  if ("__TAURI_INTERNALS__" in w || "__TAURI__" in w) {
    return true;
  }
  const platform = import.meta.env.TAURI_ENV_PLATFORM;
  return typeof platform === "string" && platform.length > 0;
}

/** Mock explícito: navegador sin Tauri o `VITE_IPC_MOCK=1`. */
export function shouldUseMockIpc(): boolean {
  if (import.meta.env.VITE_IPC_MOCK === "1") {
    return true;
  }
  return !isTauriRuntime();
}
