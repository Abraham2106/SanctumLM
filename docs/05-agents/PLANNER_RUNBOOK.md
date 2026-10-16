# Runbook del agente planner

## Arranque de sesión

1. Leer `AGENTS.md`, CHARTER, SRS, WBS, ROADMAP, ADR index, matriz y `.agents/STATUS.md`.
2. Verificar estado Git/PR/issues/CI vigente; los snapshots documentales pueden estar desfasados.
3. Conciliar requerimientos, bloqueos, riesgos, tareas en curso y aprobaciones faltantes.
4. Priorizar tareas por dependencia, valor confirmado, riesgo y capacidad; no estimar capacidad inexistente.
5. Publicar plan del ciclo y tareas `Ready` con dueño y reviewer independiente.

## Algoritmo de asignación

- Nunca asignar implementación del producto si no existe G0 o el `R-PROD` pertinente.
- Identificar entradas/salidas por tarea y reservar archivos/contratos que podrían colisionar.
- Seleccionar trabajo de mínima superficie y dependencias satisfechas.
- Una tarea por worktree; una rama por tarea. Evitar que el mismo agente sea implementer y único reviewer.
- Definir límite autorizado: hora/plazo, recursos, operaciones permitidas y condición de parada. No asumir «30 minutos» como mandato permanente; puede especificarse por ciclo.
- Si bloqueada, registrar `BLOCKED`, motivo y decisión requerida, sin inventar continuación.

## Gestión de ADR y RFC

Para todo trade-off de arquitectura, interfaces, dependencias, datos, privacidad o herramientas: crear ADR `proposed`, vincular requisitos y solicitar revisión. Cambios de alcance significativos: RFC + impacto. Solo el aprobador legítimo marca aceptación. Refrescar documentos derivados en el mismo PR.

## Revisión e integración

Al recibir `READY_FOR_REVIEW`:
1. Comprobar PR, tarea, cumplimiento de aceptación y pruebas.
2. Solicitar QA/reviewer distinto de ejecutor; revisar riesgos, permisos y decisiones.
3. Revalidar sobre la base actual y conciliar conflictos.
4. Confirmar aprobaciones de puertas; autorizar merge solo si condiciones explícitas están satisfechas.
5. Registrar `MERGED`, cerrar tarea, actualizar `.agents/TASKS.md` y `STATUS.md` y liberar reserva de archivos.
6. Retirar worktree al verificar limpieza; comunicar siguiente tarea desbloqueada.

## Checkpoint y salida

Cada checkpoint incluye fecha UTC, branch/commit/PR, task, dueño, estado, bloqueos, decisiones y próxima acción. Al cierre emitir reporte con cambios reales, pruebas, puertas pendientes, costos si se midieron, riesgo residual y backlog. No confundir «ejecutó comando» con «pasó prueba».

## Prompt de delegación mínimo

«Ejecutá únicamente TASK-#### en rama/worktree exclusivos. Leé AGENTS.md y la ficha. No elijas requisitos de producto ni ADRs por tu cuenta. Ejecutá tests y reportá evidencia. Abrí PR y handoff; escalá cualquier conflicto o acción externa sensible.»
