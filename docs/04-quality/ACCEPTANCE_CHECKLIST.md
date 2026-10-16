# Checklist de aceptación por gate

| Gate | Responsable de aprobación | Evidencia necesaria | Estado actual |
| --- | --- | --- | --- |
| G0 producto | Humano/PO | CHARTER, usuarios, MVP, límites, R-PROD iniciales | Aprobado 2026-10-16 por @Abraham2106, por instrucción en chat; contenido redactado por IA ([CHARTER](../00-project/CHARTER.md), [SRS §5](../01-requirements/SRS.md)) |
| G1 arquitectura | Humano responsable y architect | ADRs, amenazas, interfaces, arquitectura verificada | Pendiente (desbloqueado) |
| G2 vertical slice | PO y QA | TC end-to-end reproducibles, demo, PR revisado | Bloqueado por G1 |
| G3 calidad | QA, security, PO | Matriz completa, tests críticos, riesgos y excepciones | Bloqueado por G2 |
| G4 release | Humano propietario | Artefactos, operación, rollback, autorización de publicación | Bloqueado por G3 |

Este checklist no se autocompleta. Cambiar un estado exige fecha, persona/rol competente, enlace a evidencia y acuerdo rastreable; las aprobaciones no se infieren de la ausencia de objeciones.
