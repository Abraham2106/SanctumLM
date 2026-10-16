# SanctumLM — escritorio (Tauri 2 + React)

Manifiestos Node **solo en este directorio** (`apps/desktop/package.json` + `package-lock.json`); la raíz del monorepo no tiene `package.json` global.

## Comandos

```bash
npm ci
npm test          # tsc --noEmit
npm run dev       # Vite (UI en navegador)
npx tauri dev     # Tauri + Vite (requiere toolchain Rust)
```

El núcleo Rust vive en `src-tauri/` y es miembro del workspace Cargo en la raíz.
