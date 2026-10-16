# Registro durable de coordinación agéntica

Este directorio pertenece al **repositorio** y guarda decisiones/estado consolidado, no un bus de eventos vivo. Cada worktree posee su copia independiente; el planner consolida actualizaciones por PR/merge.

| Archivo | Uso |
| --- | --- |
| [GOAL.md](GOAL.md) | Objetivo vigente y restricciones |
| [TASKS.md](TASKS.md) | Snapshot del backlog y owners |
| [STATUS.md](STATUS.md) | Último checkpoint |
| [DECISIONS.md](DECISIONS.md) | Índice resumido y pendientes |
| [messages/README.md](messages/README.md) | Política de eventos y archivo |

**Reglas:** un solo escritor lógico planner; IDs estables; timestamps UTC; enlace a fuente/PR; sin datos secretos; estados verificables. Si se requiere comunicación concurrente, usar issues/PR y, solo si se aprueba, canal externo que refleje ADR-0004. El sistema de mensajería externa no está implementado.

Los demás agentes no deben modificar estos archivos desde worktrees paralelos salvo tarea documental expresamente delegada y reconciliada por el planner.
