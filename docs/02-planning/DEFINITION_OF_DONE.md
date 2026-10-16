# Definition of Ready y Definition of Done

## Ready: antes de ejecutar

Una tarea está **Ready** si tiene ID, prioridad, responsable, dependencia resuelta, requisito(s) `R-*`, WBS, criterios observables, alcance y archivos permitidos, ADR requerido identificado, plan de pruebas y presupuesto/límite de ejecución cuando se aplique. Debe estar indicado si necesita aprobación humana.

Si falta una de estas piezas, el planner la mantiene en `blocked` o `draft`; los ejecutores no adivinan.

## Done: antes de integrar

1. Cambio limitado a la tarea y criterios de aceptación verificados.
2. Pruebas correspondientes ejecutadas y reportadas con comandos, entorno y resultados; no ocultar fallos.
3. SRS/WBS/matriz/ADR/documentación de usuario o API actualizados según impacto.
4. Revisión independiente positiva, riesgos residuales comunicados y controles de seguridad pertinentes.
5. PR con evidencia de trazabilidad, sin secretos, CI satisfactoria y aprobaciones exigidas.
6. Merge autorizado, registro canónico actualizado, handoff archivado y worktree retirado *solo tras comprobar integración*.

La aceptación del producto por el humano/PO es distinta de la revisión técnica. Una tarea puede estar técnicamente revisada y seguir pendiente de aceptación funcional.

## Condiciones de rechazo

Cambios fuera de alcance; tests no ejecutados sin explicitarlo; decisiones arquitectónicas sin ADR; datos sensibles; merge sin revisión; requisitos inventados; conflicto de archivo no resuelto.
