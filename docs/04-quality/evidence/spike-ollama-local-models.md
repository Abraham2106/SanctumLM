# Spike — latencia y RAM de modelos locales vía Ollama (TASK-0011)

- **Tarea:** TASK-0011
- **ADR:** ADR-0008 (proposed)
- **Fecha:** 2026-10-16 (UTC-6)
- **Entorno declarado:** Windows 10.0.26200; worktree `SanctumLM-wt-TASK-0011`, rama `agent/TASK-0011-adr-router`
- **Riesgo:** RSK-12 — calidad/latencia en español sin benchmark no permite cerrar KPI-3/KPI-4

## Objetivo

Medir, con Ollama en loopback, al menos:

1. **1–2** completaciones de chat en **español** (latencia extremo a extremo).
2. **1** lote de **embeddings** en español (latencia y observación de RAM del proceso Ollama si es posible).

Reglas del spike: comprobar tamaño del modelo antes de `ollama pull`; **no** descargar si &gt; 4 GB o tamaño desconocido grande; directorio de trabajo temporal `%TEMP%\sanctumlm-spike-ollama` (no usado — spike no ejecutado).

## Resultado global

| Estado | Motivo |
| --- | --- |
| **not-run** | Ollama no está instalado o no está en el `PATH` del shell de medición |

No se reportan latencias ni RAM inventadas (cumplimiento RSK-12).

## Comandos de comprobación (evidencia)

Ejecutados en PowerShell:

```powershell
Get-Command ollama -ErrorAction SilentlyContinue | Select-Object -ExpandProperty Source
ollama --version
ollama list
```

**Salida observada (2026-10-16):**

```text
ollama : The term 'ollama' is not recognized as the name of a cmdlet, function, script file, or operable program.
```

`Get-Command ollama` no devolvió ruta.

## Tabla de medición

| # | Operación | Modelo | Prompt / entrada (ES) | Latencia (ms) | RAM Ollama (MB) | Notas |
| --- | --- | --- | --- | --- | --- | --- |
| 1 | chat | — | — | **not-run** | **not-run** | Ollama ausente |
| 2 | chat | — | — | **not-run** | **not-run** | Ollama ausente |
| 3 | embed | — | — | **not-run** | **not-run** | Ollama ausente |

### Prompts previstos (para repetición cuando Ollama exista)

Chat (español, ~2 turnos):

1. Usuario: «En una frase, ¿qué es la fotosíntesis?»
2. Usuario: «Resume en tres viñetas por qué importa en un ecosistema forestal.»

Embeddings (español):

- `["El agua evapora y forma nubes.", "La lluvia recarga los acuíferos subterráneos."]`

## Procedimiento reproducible (cuando Ollama esté disponible)

1. Confirmar servicio en loopback: `curl.exe -s -o NUL -w "%{http_code}" http://127.0.0.1:11434/` (esperado 200).
2. `ollama show <modelo> --modelfile` o documentación del registro para estimar tamaño; abortar pull si &gt; 4 GB.
3. Medir chat con `Measure-Command` alrededor de `ollama run <modelo> --keepalive 0` o `POST /api/chat` con JSON (§11.1).
4. Medir embed con `POST /api/embed` (§11.2).
5. RAM: `Get-Process ollama | Select-Object WorkingSet64` antes/después (orientativo, no sustituto de benchmark formal KPI-4).

Artefactos opcionales en `%TEMP%\sanctumlm-spike-ollama\` (logs JSON, sin datos personales).

## Implicación para OPEN-008

Sin filas de medición, **no** se puede ratificar modelo de referencia oficial. ADR-0008 propone candidatos (chat instruct Q4 ~3–8B; embedding compacto) sujetos a este spike en hardware 16 GB+ (investigación §14.1). **OPEN-008 permanece abierta.**

## Handoff — TASK-0011

Plantilla: `docs/templates/AGENT_REPORT_TEMPLATE.md`

- **Agente/rol:** architect (subagente TASK-0011)
- **Worktree y rama:** `C:\Users\solan\Documents\Personal\Startup\SanctumLM-wt-TASK-0011`, `agent/TASK-0011-adr-router`
- **PR y commits:** pendiente de push/PR (fuera de alcance TASK-0011)
- **Estado:** ready_for_review
- **Fecha UTC:** 2026-10-17T05:30:00Z (aprox.)

### Alcance ejecutado

- `docs/03-architecture/adr/ADR-0008-model-router.md` — contrato `ModelAdapter`, router, Ollama loopback, `cloud_fallback=false` en núcleo, chat general sin RAG, UI sin fetch a proveedores, remoto opt-in contra fake.
- `docs/04-quality/evidence/spike-ollama-local-models.md` — spike **not-run** (Ollama no en PATH).

Desviaciones: ninguna sobre superficie git permitida; no se editó `adr/README.md` ni `.agents/*` (índice ADR fallará en validador — esperado).

### Verificación

| Comando o TC | Entorno | Resultado | Evidencia |
| --- | --- | --- | --- |
| `python scripts/validate_docs.py` | worktree TASK-0011 | fail esperado (ADR-0008 sin índice) | salida en commit |
| `git diff --check` | worktree TASK-0011 | pass | sin conflictos de espacios |
| Spike Ollama chat/embed ES | local | not-run | comandos `ollama` arriba |

### Trazabilidad

- Requisitos: R-PROD-007, R-PROD-008, R-PROD-101, R-PROD-102, R-PROD-109
- Riesgo: RSK-12
- Cuestión: OPEN-008 (recomendación en ADR-0008, sin cierre)
- WBS: 3.2, 3.3, 4.5 (diseño router/egress)
- ADR: ADR-0008 proposed

### Decisiones propuestas y pendientes

- **Propuesta:** ADR-0008 (router + Ollama primero + contrato adapter).
- **Pendiente:** aprobación humana G1; indexar ADR-0008; implementación `crates/ai-router`; repetir spike con Ollama instalado.

### Riesgos/bloqueos/seguimiento

| Punto | Responsable | Siguiente acción |
| --- | --- | --- |
| RSK-12 sin datos | Architect / QA | Instalar Ollama en máquina de benchmark y re-ejecutar spike |
| Índice ADR | Planner | Añadir fila ADR-0008 en `adr/README.md` y `.agents/DECISIONS.md` en tarea coordinada |
| OPEN-008 | Humano + architect | Decidir tras spike con mediciones reales |

### Checklist de cierre

- [x] Sin secretos
- [ ] Revisión independiente (pendiente PR)
- [x] Documentos entregados
- [ ] Merge confirmado
- [ ] Worktree retirado
