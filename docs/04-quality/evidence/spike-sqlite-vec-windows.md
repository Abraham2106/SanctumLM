# Evidencia — spike sqlite-vec en Windows (TASK-0010)

- **Fecha:** 2026-10-16
- **Entorno:** Windows 10.0.26200, `rustc 1.99.0`, `cargo 1.99.0`, MSVC 14.51 (Visual Studio 2026 Community)
- **Ubicación spike (no versionado):** `%TEMP%\sanctumlm-spike-sqlite-vec`
- **Resultado global:** **pass** (carga, consulta vectorial y distancia L2)

## Objetivo

Comprobar que el crate `sqlite-vec` fijado compila con el toolchain Windows del proyecto, se registra como extensión SQLite y responde a `vec0` / `vec_distance_l2`.

## Versiones probadas

| Versión crate | Resultado compilación | Notas |
| --- | --- | --- |
| `0.1.10-alpha.4` | **fail** | `build-script` de `cc-rs` falla: `fatal error C1083: Cannot open include file: 'sqlite-vec-diskann.c'` (empaquetado incompleto en crates.io). |
| `=0.1.9` | **pass** | Compila con MSVC; extensión enlazada como `sqlite_vec0`. |

## Comandos ejecutados

Desde PowerShell, directorio `%TEMP%\sanctumlm-spike-sqlite-vec`:

```powershell
cargo init --name sanctumlm_spike_sqlite_vec
cargo add rusqlite --features load_extension
# Cargo.toml: sqlite-vec = "=0.1.9"
$env:CARGO_TARGET_DIR = "$env:TEMP\sanctumlm-spike-sqlite-vec\target"
cargo clean
cargo run
```

## Salida observada (extracto)

```
vec_version=v0.1.9
PASS: distance(row1)=2.236068 nearest_rowid=2
```

Interpretación: la extensión se auto-registra vía `sqlite3_auto_extension` + `sqlite3_vec_init`; la tabla virtual `vec0` acepta inserciones; la búsqueda por distancia L2 devuelve el vecino esperado (`[1,1,1]` más cercano a rowid 2).

## Riesgos detectados en el spike

- **Pre-v1:** la API Rust cambió entre `0.1.9` (`sqlite3_vec_init` público) y `0.1.10-alpha.4` (función `load` documentada pero crate roto en Windows).
- **Enlace SQLite:** si `LIB`/`LIBPATH` apuntan a un `libsqlite3` ajeno (p. ej. MSYS2), el enlazado puede fallar; el spike exitoso usó el SQLite embebido de `libsqlite3-sys` (bundled vía `rusqlite`).
- **Reproducibilidad CI:** fijar `=0.1.9` hasta que exista un release estable probado en las tres plataformas objetivo.

## Relación con ADR-0007

Esta evidencia respalda la propuesta de fijar **sqlite-vec `0.1.9`** detrás de un trait de índice vectorial intercambiable, con plan B FTS5-only documentado en [ADR-0007](../../03-architecture/adr/ADR-0007-storage.md).
