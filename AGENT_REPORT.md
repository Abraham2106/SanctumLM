# Handoff — TASK-0019

- **Agente/rol:** implementer (subagente)
- **Worktree y rama:** `C:\Users\solan\Documents\Personal\Startup\SanctumLM-wt-TASK-0019` · `agent/TASK-0019-b1-desktop`
- **Base:** `agent/TASK-0018-b1-crates` @ `dee152d`
- **PR y commits:** pendiente (sin push)
- **Estado:** ready_for_review
- **Fecha UTC:** 2026-10-16

## Alcance ejecutado

- `apps/desktop/`: Vite + React 19 + TypeScript; UI en español con pestañas **Fuentes | Chat | Estudio** (placeholders, sin datos reales).
- `apps/desktop/src-tauri/`: Tauri 2, comando IPC stub `ping`, sin plugins HTTP/red; capability `default` con solo `core:default`; CSP sin `https://` remoto.
- Manifiestos Node **solo** en `apps/desktop/` (`package.json` + `package-lock.json`); raíz sin `package.json`.
- `Cargo.toml` (raíz): único cambio en crates TASK-0018 — miembro `apps/desktop/src-tauri`.
- `Cargo.lock`: resolución Tauri 2.x al compilar el crate desktop.

**Desviaciones:** no se modificó `scripts/verify.ps1` (alcance recortado por planner). Iconos generados con `npx tauri icon` local (no `create-tauri-app`); subcarpetas `android/`/`ios/` eliminadas del árbol.

## Cómo correr

```powershell
cd C:\Users\solan\Documents\Personal\Startup\SanctumLM-wt-TASK-0019\apps\desktop
npm ci
npm test          # tsc --noEmit
npm run dev       # UI en navegador (5173)
npx tauri dev     # app Tauri + Vite (Rust + WebView2)
```

Desde la raíz del worktree: `cargo test --workspace`.

## Verificación

| Comando | Entorno | Resultado |
| --- | --- | --- |
| `npm test` (apps/desktop) | Node local | pass |
| `cargo test --workspace` | Windows, Rust stable | pass |
| `git diff --check` | worktree | pendiente en commit |

## Dependencias (§9.4 — revisión de licencias)

| Paquete | Versión fijada |
| --- | --- |
| `@tauri-apps/api` | 2.12.4 |
| `@tauri-apps/cli` | 2.12.3 (dev) |
| `react` / `react-dom` | 19.1.0 |
| `vite` | 6.3.5 |
| `typescript` | 5.8.3 |
| `@vitejs/plugin-react` | 4.5.2 |
| `tauri` / `tauri-build` (crates.io) | 2.x (lock: 2.12.4 / 2.7.3) |

## Trazabilidad

- ADR-0006 (Tauri 2 + React/TS), R-PROD-108 (esqueleto UI tres paneles).
- Invariantes: UI sin `fetch()`; egress futuro vía IPC Rust; capabilities explícitas deny-by-default salvo `core:default`.

## Riesgos / seguimiento

- **WebView2** obligatorio en Windows para `tauri dev`/build.
- **Primer build Rust** descarga muchas crates Tauri (tiempo/red en desarrollo, no en runtime de producto).
- **npm audit** reportó 1 vulnerabilidad alta en árbol dev; no corregida en este stub (fuera de alcance).
- Integración con crates de dominio (storage, ai-router) pendiente de tareas posteriores.

## Archivos principales

- `Cargo.toml`, `Cargo.lock`
- `apps/desktop/package.json`, `package-lock.json`, `vite.config.ts`, `src/App.tsx`, `src-tauri/**`
