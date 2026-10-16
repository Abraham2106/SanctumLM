# Glosario

Términos con significado específico en este repositorio. Si un documento los usa con otro sentido, prevalece esta definición y el desajuste se corrige por PR.

| Término | Significado |
| --- | --- |
| Agregación | Contribución integrada a `main` a partir de una tarea. En la práctica equivale a **una tarea**: una ficha, una rama, un worktree y un PR. |
| ADR | Architecture Decision Record. Explica por qué se tomó una decisión estructural y qué alternativas se descartaron. Ver [índice ADR](../03-architecture/adr/README.md). |
| Checkpoint | Registro del estado de una tarea (rama, commit, bloqueos, siguiente acción) suficiente para que otro agente la retome. |
| Estado durable | Información versionada en Git, PRs o issues. Es la que cuenta como evidencia. |
| Estado efímero | Información de coordinación en tiempo real fuera del repositorio (ver ADR-0004). Nunca sustituye al estado durable. |
| Ficha de tarea | Descripción aprobada de una tarea según la [plantilla](../templates/TASK_BRIEF_TEMPLATE.md). Sin ficha no se implementa. |
| Gate (puerta) | Punto de control G0–G4 que solo supera una aprobación humana registrada. Ver [CHARTER](CHARTER.md#puertas). |
| Handoff | Informe de entrega al cambiar de dueño, terminar o bloquear una tarea. Ver [plantilla](../templates/AGENT_REPORT_TEMPLATE.md). |
| Hipótesis | Inferencia no confirmada. Debe rotularse como tal y no puede usarse como requisito. |
| `not-run` | Resultado de una prueba que no se ejecutó. Nunca se reporta como `pass`. |
| `OPEN-###` | Cuestión sin respuesta confirmada que bloquea trabajo dependiente. Ver [registro](OPEN_QUESTIONS.md). |
| Planner | Rol lógico que mantiene el plan y el estado canónico. Puede ejercerlo un humano o un agente, pero no aprueba gates. |
| Ready / Done | Condiciones para empezar y para integrar una tarea. Ver [Definition of Ready/Done](../02-planning/DEFINITION_OF_DONE.md). |
| RFC | Propuesta de cambio de alcance o de requisitos, con análisis de impacto. Ver [plantilla](../templates/RFC_TEMPLATE.md). |
| Worktree | Directorio de trabajo Git independiente (`git worktree`) asignado a una sola tarea. Ver [protocolo](../05-agents/WORKTREE_PROTOCOL.md). |
