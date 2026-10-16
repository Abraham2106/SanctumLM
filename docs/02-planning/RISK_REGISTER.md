# Registro de riesgos — RSK

**Escala:** probabilidad e impacto: bajo / medio / alto; no se inventan probabilidades numéricas.

| ID | Riesgo / origen | Prob. | Impacto | Mitigación | Dueño / disparador |
| --- | --- | --- | --- | --- | --- |
| RSK-01 | Producto mal definido; los agentes suponen funcionalidades | Alto | Alto | G0; SRS sin hipótesis implícitas; bloquear implementación dependiente | Planner / tarea sin `R-PROD` |
| RSK-02 | Colisión de cambios entre worktrees | Medio | Alto | Rama/worktree por tarea; mapa de archivos y revisión de integración | Planner / archivos superpuestos |
| RSK-03 | ADR y código divergen | Medio | Alto | ADR obligatorio para decisiones estructurales; chequeo en PR | Architect / cambios de interfaz |
| RSK-04 | Autoevaluación indulgente o «pruebas» inventadas | Medio | Alto | Reviewer independiente; comandos/evidencias reales; CI | QA / handoff sin evidencia |
| RSK-05 | Fuga de tokens, prompts o datos | Medio | Alto | Secret scanning, mínimos permisos, logs sanitizados | Security / diffs sospechosos |
| RSK-06 | Aprobaciones humanas omitidas | Medio | Alto | Gates explícitos y bloqueo de acciones irreversibles | Planner / G0-G4 o despliegue |
| RSK-07 | Estado de agente desactualizado y perdido | Alto | Medio | Git/PR como durables; runtime externo solo efímero; checkpoints | Planner / reinicio |
| RSK-08 | Dependencias/servicios externos cambian | Medio | Medio | ADR de integración, versiones fijadas, contratos/pruebas | Architect / G1 |
| RSK-09 | Tiempo/costo de agentes sin control | Medio | Medio | Presupuesto por tarea, hitos, cancelación y límites | Planner / ejecución sobre límite |
| RSK-10 | Requisito sin prueba o criterio ambiguo | Medio | Alto | Trazabilidad y criterios de aceptación previos | QA / revisión de backlog |

## Tratamiento

En cada checkpoint el planner revisa riesgos activados, responsable, contención, residual y decisión humana cuando aplica. Los riesgos críticos sin mitigación bloquean la puerta correspondiente. Cada nueva decisión o cambio de alcance revisa este registro en el mismo PR.
