# Casos de uso y escenarios

**Estado:** escenarios de ingeniería aprobables; escenarios de producto pendientes de G0.

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

## UC-PROD — pendiente

En G0 redactar como mínimo: flujo principal, flujo de error, permisos/abuso, experiencia inicial, recuperación y salida de datos, según el producto real. Ninguna de esas capacidades se afirma como existente.
