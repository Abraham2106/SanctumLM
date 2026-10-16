# Roadmap y dependencias por puertas

No hay fechas ni capacidad de equipo acordadas. Por eso esta hoja de ruta indica precedencia y aceptación, no un cronograma ficticio.

| Etapa | Paquetes | Entrada | Criterio de salida / gate |
| --- | --- | --- | --- |
| Descubrimiento | WBS-1.* | Encargo y repositorio | G0: visión, MVP, usuarios, límites y métricas aprobadas por humano |
| Preparación agéntica | WBS-2.* | Modelo de colaboración | Reglas/worktrees/ADRs/tareas/CI documentados y verificables |
| Arquitectura | WBS-3.* | G0 aprobado | G1: arquitectura y seguridad revisadas; decisiones ADR aceptadas |
| MVP vertical | WBS-4.1..4.3 | G1 aprobado | G2: caso de uso funcional probado de extremo a extremo |
| Endurecimiento | WBS-4.4..4.5 | G2 aprobado | G3: riesgos tratados, criterios satisfechos, revisión independiente |
| Release | WBS-5.* | G3 aprobado | G4: paquete reproducible, operación/rollback y aceptación humana |

**Camino crítico provisional:** G0 → SRS del producto → ADR de stack/interfases → G1 → vertical slice → G2 → evaluación → G3 → release/G4.

**Paralelismo permitido:** normas, plantillas, seguridad de proceso y validador documental pueden avanzar antes de G0. Implementación de producto, integración externa e infraestructura quedan bloqueadas hasta su puerta correspondiente.

**Actualización de estimaciones:** tras G0, el planner agrega esfuerzo en días-persona o puntos *según metodología aprobada*, intervalos de incertidumbre, capacidad real y riesgos. Registrar fecha de baseline y variaciones en PR/RFC, nunca reemplazar silenciosamente el plan.
