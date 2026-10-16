# Release, versiones y aceptación

## Política

Las etiquetas/versiones del producto se definen tras G0 y tras elegir sistema de construcción; por ahora los documentos llevan versión 0.1 y no representan una entrega de software.

## Checklist G4

1. SRS de producto aprobado, funcionalidades priorizadas y casos de aceptación vinculados.
2. Arquitectura/ADRs aceptados y seguridad revisada.
3. CI, pruebas funcionales y negativas con evidencia; riesgos residuales explícitos.
4. Build/artefacto reproducible desde un commit, versiones y dependencias identificables.
5. Notas de release, guía de instalación/actualización, compatibilidad y restricciones.
6. Plan de backup/migración/rollback cuando corresponda, criterio y responsables.
7. Validación del humano/PO y permisos para publicar, desplegar o incurrir en costos.
8. Estado de tasks/PRs y documentación de operación actualizados.

## Rollback y fallos

Antes de publicar definir señal de aborto, procedimiento ensayado, afectación de datos y autoridad para activarlo. Nunca asumir que revertir un commit revierte migraciones o acciones externas.

La preparación de una versión y la **ejecución** de un despliegue son aprobaciones diferentes.
