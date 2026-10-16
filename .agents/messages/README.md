# Archivo de mensajes — política

No utilizar archivos bajo `.agents/messages/` como cola en vivo entre worktrees: sus sistemas de archivos de trabajo están aislados. Los issues y PRs son el canal durable predeterminado; el planner puede archivar resúmenes aquí **después** de reconciliar eventos.

Formato sugerido de archivo histórico: `TASK-####.md` con fecha, evento, actor, enlace GitHub, resumen y acción siguiente. Evitar PII, secretos y textos de herramientas no confiables. Para comunicación inmediata considerar ADR-0004 y el protocolo [COMMUNICATION](../../docs/05-agents/COMMUNICATION.md).
