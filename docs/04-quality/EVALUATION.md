# Evaluación de agentes y del proceso

El objetivo es determinar si los agentes generan resultados **correctos, rastreables y seguros**, no si producen más texto o commits.

## Dataset mínimo de evaluación (propuesto)

Construir casos controlados con: requisito ambiguo sin G0, tarea con dependencia bloqueada, conflicto del mismo archivo, ADR incompatible, test fallido, mensaje con prompt injection, PR con secreto ficticio, intento de desplegar sin permiso, caída entre commits, revisión del propio cambio, tarea sin criterio de aceptación y recuperación de checkpoint.

### Corpus RAG producto (§5.3 investigación)

El conjunto **`benchmarks/es-mvp-rag/`** (15 fuentes en español, manifiesto con SHA-256, `evaluation/questions.json`) cubre tablas, HTML estático, inyección como datos, fuentes contradictorias, PDF escaneado simulado, archivos `corrupt-*` y pares aislados por notebook (§15 e). Uso: importación mixta (R-PROD-002), citas y abstención (R-PROD-006/104), aislamiento (R-PROD-105) y batería adversarial documental (R-PROD-111). Ver `benchmarks/README.md`.

## Métricas

- **Correctitud:** % tareas aceptadas sin defectos graves posteriores (denominador: tareas revisadas).
- **Trazabilidad:** % requisitos aprobados con WBS, test y PR asociados.
- **Aislamiento:** número de ediciones no autorizadas fuera de worktree/task (meta normativa: cero).
- **Aprobaciones:** número de acciones sensibles sin autorización (meta normativa: cero).
- **Seguridad:** exposición de secretos y fallos críticos detectados antes de merge.
- **Confiabilidad:** % de recuperaciones que mantienen evidencia; conflictos y retrabajo por tarea.
- **Eficiencia:** tiempo de ciclo, costo medido por tarea cuando existan datos y porcentaje de tareas bloqueadas.

## Procedimiento

Definir baseline y umbrales reales después de las primeras corridas; evaluar con mismos inputs, permisos y condiciones; exigir artefactos verificables (PR, diff, TC). Revisor independiente clasifica hallazgos y el planner actualiza backlog. No confundir puntuación de un juez LLM con verdad: contrastar con criterios determinísticos, pruebas y observación humana.

El registro de evaluaciones no debe contener credenciales, código confidencial de terceros ni datos personales.
