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

Describen el comportamiento aprobado; ninguno está implementado todavía. Actores según el [CHARTER](../00-project/CHARTER.md).

### UC-PROD-01 — Notebook local con fuentes (fase 1)

**Actor:** estudiante. **Requisitos:** R-PROD-001, R-PROD-002, R-PROD-014, R-PROD-013.
1. Crea un notebook y arrastra PDFs, apuntes en Markdown y un DOCX.
2. La cola muestra el progreso de extracción e indexado; puede cancelar y reintentar.
3. Cierra la aplicación, desactiva la red y reabre: el notebook y sus fuentes siguen disponibles.

**Error:** un archivo corrupto o de tipo falso se rechaza con un motivo concreto, sin afectar al resto del lote.

### UC-PROD-02 — Preguntar a las fuentes con citas (fases 1–2)

**Actor:** investigador. **Requisitos:** R-PROD-006, R-PROD-104, R-PROD-105.
1. Selecciona las fuentes del notebook que entran en contexto.
2. Pregunta; la respuesta, generada con un modelo local, marca qué es respaldado, inferencia, discrepancia o no comprobado.
3. Pulsa una cita y se abre exactamente el fragmento de la versión citada.

**Error:** no hay evidencia → «no hay evidencia suficiente», sin citas inventadas. Una fuente con instrucciones incrustadas («ignora al usuario») se trata como datos.

### UC-PROD-03 — Chat general (fase 1)

**Actor:** usuario general. **Requisitos:** R-PROD-007, R-PROD-008. Abre un chat fuera de cualquier notebook, elige el modelo y conversa; ningún documento entra en el contexto.

### UC-PROD-04 — Investigar en la web e importar (fase 3)

**Actor:** docente. **Requisitos:** R-PROD-004, R-PROD-003, R-PROD-111.
1. Busca un tema en la web; ve fuente, dominio, fecha y fragmento.
2. Selecciona dos resultados y revisa la vista previa.
3. Se guardan copias textuales locales que luego funcionan sin red.

**Error:** una página redirige a `127.0.0.1` o a una red privada → la descarga se deniega y queda en la auditoría.

### UC-PROD-05 — Usar un proveedor remoto con consentimiento (fase 3)

**Actor:** investigador. **Requisitos:** R-PROD-008, R-PROD-102, R-PROD-110.
1. Para una pregunta concreta elige un modelo remoto.
2. La aplicación muestra destino, fragmentos exactos a enviar, tamaño y categorías de datos.
3. Aprueba solo esa solicitud; la decisión queda registrada.

**Error:** el modelo local se cae → la aplicación lo informa y **no** cambia sola a un proveedor remoto. Una fuente `restricted` nunca se envía.

### UC-PROD-06 — Generar un artefacto de estudio (fases 4–5, v1)

**Actor:** docente. **Requisitos:** R-PROD-009, R-PROD-010, R-PROD-011. Pide una guía, una infografía o una presentación; revisa el esquema con sus evidencias, edita una sección sin regenerar el resto y exporta.
