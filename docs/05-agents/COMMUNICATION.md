# Protocolo de comunicación y bitácora

## Canales y autoridad

**Durable/canónico:** Git, PR, issues, ADR, SRS, WBS y snapshots versionados por planner en `.agents/`. **Transitorio:** transporte externo al repositorio, si se habilita mediante ADR-0004. Ningún transportador de mensajes sustituye una aceptación formal.

Los mensajes en distintos worktrees NO se sincronizan automáticamente. Por defecto, el ejecutor comunica a través del issue/PR de su task; el planner consolida un snapshot cuando corresponde. No hacer commits periódicos de todos los ejecutores a `.agents/STATUS.md`.

## Sobre mínimo de evento

```json
{
  "event_id": "evt-unique",
  "event_type": "BLOCKED",
  "task_id": "TASK-0001",
  "agent_id": "implementer-01",
  "timestamp_utc": "2026-10-15T00:00:00Z",
  "summary": "Falta una decisión de interfaz",
  "evidence_url": "https://github.com/Abraham2106/SanctumLM/issues/1",
  "action_required": "Evaluar ADR antes de continuar"
}
```

Este objeto es **ejemplo de esquema**, no un evento real ni una URL de issue existente. Eventos permitidos: `ASSIGNED`, `STARTED`, `BLOCKED`, `ADR_PROPOSED`, `READY_FOR_REVIEW`, `CHANGES_REQUESTED`, `APPROVED`, `MERGED`, `CANCELLED`. Payloads reales se validarán si se programa transporte.

## Concurrencia y conflictos

- Planner mantiene un escritor lógico para `.agents/TASKS.md`, `STATUS.md`, `DECISIONS.md`.
- Las propuestas de cambios de otros agentes viajan como issues, PRs y mensajes; no sobrescriben fuentes centrales sin revisión.
- En transporte local eventual, el orquestador debe implementar escrituras atómicas, deduplicación de `event_id`, TTL/heartbeat y recuperación; no se consideran implementados en esta línea base.
- Un mensaje no debe contener credenciales, PII, prompts completos externos ni ejecuciones con datos confidenciales.

## Handoff y escalación

Se requiere handoff al cambiar de owner, completar o bloquear tarea. Usar [plantilla](../templates/AGENT_REPORT_TEMPLATE.md). Escalar en este orden: problema técnico → planner; arquitectura → architect/ADR; producto, costo, despliegue o permisos → humano. Registrar decisión y enlace, no solo «resuelto».
