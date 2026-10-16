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
| RSK-11 | G0 basado en investigación documental, sin validación con estudiantes, docentes o investigadores reales | Medio | Alto | Pruebas con usuarios del recorrido de la fase 1 antes de G3; revisar prioridades con sus datos | PO / primeras sesiones con usuarios |
| RSK-12 | Los modelos locales en 16 GB no alcanzan calidad o latencia útiles en español (KPI-3, KPI-4) | Medio | Alto | Benchmark temprano en G1; ADR con modelos alternativos y degradación controlada | Architect / benchmark de G1 |
| RSK-13 | Alcance excesivo: perseguir paridad con Gemini Notebook impide entregar una versión estable (investigación §14.3) | Alto | Alto | Tramos MVP/v1 del CHARTER; nada de fases 4–5 antes de cerrar las fases 1–3 | Planner / tareas de v1 antes de G2 |
| RSK-14 | El parser pierde tablas, encabezados o ubicación y produce citas engañosas aunque el modelo responda bien | Medio | Alto | Corpus difícil en el conjunto de evaluación; ubicación verificada por cita; parser aislado y reemplazable | QA / caída de `citation_precision` |

## Tratamiento

En cada checkpoint el planner revisa riesgos activados, responsable, contención, residual y decisión humana cuando aplica. Los riesgos críticos sin mitigación bloquean la puerta correspondiente. Cada nueva decisión o cambio de alcance revisa este registro en el mismo PR.
