# Contribuir a SanctumLM

Leer [AGENTS.md](AGENTS.md), [índice](docs/README.md), [SRS](docs/01-requirements/SRS.md) y [modelo operativo](docs/05-agents/OPERATING_MODEL.md).

Toda contribución está ligada a una tarea `TASK-####` con requisito, WBS, aceptación y permisos. El planner asigna trabajo y evita conflictos. Usá un worktree y rama propios siguiendo [WORKTREE_PROTOCOL](docs/05-agents/WORKTREE_PROTOCOL.md), abrí PR y completá la plantilla. El revisor no puede ser el mismo implementer.

Documentos de requisitos y decisiones se cambian en PR con trazabilidad. Para un cambio de alcance abrí RFC; para una decisión estructural creá ADR. No inventés funciones del producto: hasta G0 hay definición pendiente. No subás tokens, archivos locales, secretos ni datos reales sensibles.

Antes de proponer cambios documentales: `python3 scripts/validate_docs.py`. Para producto, ejecutar además el pipeline real que se adopte tras G1. Registrar `not-run` donde no haya test ejecutado. Aprobación de reviewer ≠ permiso de desplegar.
