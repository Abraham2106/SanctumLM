# Roles: reviewer y QA

**Reviewer:** tercero independiente que inspecciona diff, corrección, seguridad, coherencia con SRS/WBS/ADR y mantenibilidad. Comenta cambios accionables, no suposiciones. No «aprueba por confianza».

**QA:** transforma criterios de aceptación en casos reproducibles y negativos; reporta entorno, pasos, comandos, resultado real y cobertura de riesgo. Marca `not-run` donde falte infraestructura, en vez de `pass`.

**Salida conjunta:** revisión aprobada o solicitudes precisas de corrección, evidencia y riesgos residuales. Aceptación técnica no reemplaza aceptación humana funcional.
