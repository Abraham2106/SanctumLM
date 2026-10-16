# Operaciones, observabilidad y recuperación

## Operación del flujo de desarrollo

El planner trabaja por ciclos: reconciliar GitHub → publicar plan → asignar tareas → comprobar checkpoints/PR/CI → revisión → consolidación → reporte. Los archivos `.agents/` son snapshots y no heartbeats. Para procesos de larga duración se deben fijar explícitamente límites de tiempo/costo, política de cancelación y canales de alertas antes de su uso.

## Evidencia mínima de ejecución

Para cada tarea registrar ID, estado, rol, rama y commit, timestamp UTC, comandos de validación, resultados, eventos de bloqueo, handoff, PR y aprobación. Los logs solo contienen datos mínimos y sanitizados. La observabilidad del producto depende de G1 (no se asume stack de telemetría).

## Escenarios de recuperación

**Agente desconectado:** consultar último commit/checkpoint, verificar tareas sin duplicar, reasignar con nuevo worktree si el anterior no es fiable.

**Fallo de CI:** no fusionar; asociar errores a task, corregir y volver a ejecutar pruebas.

**Conflicto Git:** planner determina secuencia/propiedad; resolver en rama autorizada, no en main.

**Decisión contradictoria:** detener integración afectada, RFC/ADR de reconciliación, notificar a agentes.

**Secreto expuesto:** seguir [SECURITY](SECURITY.md); limpiar archivo no equivale a revocar secreto ni borrar historial.

## Entrega operativa del producto

Cuando exista producto, documentar instalación, configuración, métricas/SLO, backups, incidentes, on-call, migración y rollback concretos. Hasta entonces es un criterio de puerta y no una capacidad implementada.
