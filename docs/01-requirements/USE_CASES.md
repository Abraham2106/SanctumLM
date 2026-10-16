# Casos de uso y escenarios

**Estado:** escenarios de ingeniería aprobables; escenarios de producto aprobados en G0 (2026-10-16).

## UC-ENG-01 — De necesidad a tarea ejecutable

**Actor:** planner. **Disparador:** solicitud nueva o corrección. **Precondición:** propósito confirmado, o descubrimiento abierto.

1. Registrar necesidad y origen, separar hecho de hipótesis.
2. Crear/actualizar `R-*` y criterio de aceptación.
3. Evaluar dependencias, seguridad, ADR y trabajo en WBS.
4. Definir ficha de tarea con dueño, archivos, pruebas y puerta de aprobación.
5. Publicar asignación; ningún agente implementa fuera de su ficha.

**Alternativa:** si falta información crítica, marcar `blocked` y pedir resolución humana concreta, sin inventar.

**Evidencia:** ficha enlazada a SRS/WBS y estado canónico.

## UC-ENG-02 — Implementación aislada

**Actor:** implementer. **Precondición:** tarea asignada y rama base conocida.

1. Crear rama/worktree exclusivo; verificar limpieza.
2. Leer contrato de trabajo y decisiones aceptadas.
3. Implementar cambios mínimos y pruebas; proponer ADR antes de decisiones estructurales.
4. Ejecutar validaciones reproducibles y actualizar trazabilidad.
5. Publicar commit, PR y handoff.

**Alternativa:** fallo, conflicto o dependencia → checkpoint, estado `blocked` y mensaje al planner; no tocar otro worktree.

## UC-ENG-03 — Revisión e integración

**Actor:** reviewer y planner. Revisor independiente contrasta aceptación, tests, seguridad y ADR; solicita correcciones o aprueba. Planner confirma puertas y condiciones de fusión. Un fallo devuelve la tarea a ejecución sin saltarse los controles.

## UC-ENG-04 — Cambio de requisito o decisión

**Actor:** planner/architect/PO. Registrar RFC, motivación, alternativas, impacto (SRS/WBS/tests/seguridad), ADR si corresponde, aprobación competente, versión y comunicación a agentes afectados.

## Casos de uso del producto (línea base G0)

Describen el comportamiento aprobado; ninguno está implementado todavía.

### UC-PROD-01 — Primera puesta en marcha

**Actor:** usuario primario. **Requisitos:** R-PROD-009, R-PROD-001, R-PROD-013.
1. Abre la aplicación; esta detecta el entorno del modelo local.
2. Si falta, la aplicación guía la instalación o la selección del modelo, sin conexión una vez descargado.
3. Crea su primera colección.

**Error:** el equipo no cumple el mínimo de memoria → la aplicación lo indica y sugiere un modelo más pequeño.

### UC-PROD-02 — Importar e indexar

**Actor:** usuario primario. **Requisitos:** R-PROD-002, R-PROD-003.
1. Arrastra una carpeta o elige archivos.
2. La aplicación importa los formatos admitidos y muestra el progreso del indexado.
3. Al terminar, informa los archivos rechazados con su motivo.

**Error:** se cierra la aplicación a mitad → al reabrir se reanuda sin reprocesar.

### UC-PROD-03 — Preguntar con citas

**Actor:** usuario primario. **Requisitos:** R-PROD-004, R-PROD-005, R-PROD-008, R-PROD-011.
1. Escribe una pregunta sobre la colección.
2. Recibe una respuesta generada localmente, con citas a documento y página.
3. Pulsa una cita y ve el fragmento de origen resaltado.

**Error:** no hay evidencia → la respuesta lo dice explícitamente y sugiere reformular.

### UC-PROD-04 — Buscar y resumir

**Actor:** usuario primario. **Requisitos:** R-PROD-006, R-PROD-007. Busca un término o concepto, abre un resultado y pide el resumen del documento con citas.

### UC-PROD-05 — Exportar y borrar

**Actor:** usuario primario. **Requisitos:** R-PROD-010. Exporta una conversación a Markdown; elimina una colección y todos sus datos locales desaparecen.
