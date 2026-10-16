/// <reference types="vite/client" />

interface ImportMetaEnv {
  readonly VITE_IPC_MOCK?: string;
  readonly TAURI_ENV_PLATFORM?: string;
  readonly TAURI_ENV_ARCH?: string;
  readonly TAURI_ENV_FAMILY?: string;
}

interface ImportMeta {
  readonly env: ImportMetaEnv;
}
