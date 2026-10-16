# ADR-0004 — Separar estado durable y efímero

- **Estado:** proposed
- **Fecha:** 2026-10-15
- **Alcance:** coordinación técnica del desarrollo
- **Vínculos:** R-ENG-006, R-ENG-012; WBS-2.4
- **Aprobación:** pendiente

## Contexto

Los worktrees tienen contenidos aislados y no reflejan ediciones sin commit de otros agentes. Un «blackboard» en `.agents/` dentro del checkout no sirve, por sí solo, de mensajería instantánea.

## Decisión propuesta

Usar Git/PR/issue y los archivos `.agents/` como historial **durable** que consolida el planner. Para ejecución simultánea en una misma máquina, el orquestador **podría** usar una carpeta compartida externa (por ejemplo `../.sanctumlm-runtime/`) con escritura atómica y control de propietarios. Si los agentes ejecutan en máquinas diferentes, preferir un servicio externo explícitamente aprobado o comunicación por GitHub.

## Alternativas

Mensajes únicamente en archivos Git (latencia/concurrencia); base de datos o cola desde el día uno (infraestructura prematura).

## Consecuencias

Se evita prometer visibilidad inexistente entre worktrees. Falta decidir transporte real, bloqueos, reintentos, recuperación y autenticación antes de automatizar el runtime. La solución propuesta NO autoriza escribir bajo `.git` ni provisionar servicios.
