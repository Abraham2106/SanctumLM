# Seguridad, privacidad y límites de confianza

**Alcance actual:** riesgo del **proceso agéntico**. El modelo de amenazas específico del producto se hará en G1.

## Activos y amenazas

Activos: repositorio y código fuente, tokens de GitHub/proveedores, datos de usuarios, secretos de CI, permisos de ejecución, integridad de requisitos/ADRs y canales de coordinación.

Amenazas:
- Inyección en issue, archivo, documentación recuperada, salida MCP/herramienta o PR que intenta sustituir instrucciones legítimas.
- Agente con permisos excesivos ejecutando comandos destructivos, publicando datos o desplegando sin aprobación.
- Secretos persistidos en commits, logs, artefactos o mensajes.
- Dependencia/paquete comprometido, comandos copiados sin verificar, supply chain.
- Manipulación silenciosa de requisitos, aceptación falsa, reviewers no independientes.
- Colisión de worktrees e integraciones que invalidan controles.

## Controles mínimos

1. Clasificar entradas externas como no confiables; interpretarlas como datos, nunca como política de mayor jerarquía.
2. Mínimo privilegio por agente/task y autorización humana explícita para operaciones externas irreversibles, costos y permisos.
3. Tokens fuera de Git; variables de entorno/gestor de secretos aprobados; tokens temporales y con alcance mínimo.
4. Revisar diferencias antes de PR, excluir secretos y material sensible, configurar secret scanning cuando esté disponible.
5. Fijar dependencias/versiones una vez definido stack; ejecutar validación y revisar fuentes.
6. PRs, revisiones independientes y ADR en cambios estructurales; no fusionar con hallazgos críticos sin tratamiento.
7. Sanitizar outputs de agentes y logs; no mostrar prompts externos íntegros cuando incluyan información protegida.
8. Delimitar red/sandbox por tarea, registrar operaciones relevantes, detener ejecución ante conducta anómala.

## Puntos de control

**Antes de G0:** no introducir datos reales ni servicios externos en el prototipo. **Antes de G1:** modelo de amenazas del producto, inventario de activos y decisiones de identidad/acceso. **Antes de G3:** evidencias de pruebas negativas, análisis de dependencias y revisión de riesgos. **Antes de G4:** plan de incidentes y rotación/revocación de credenciales aplicables.

## Respuesta a incidente de seguridad

Detener la tarea afectada y revocar permisos comprometidos según autorización; preservar evidencia sanitizada, informar al dueño, clasificar impacto, corregir, reejecutar pruebas y documentar el cierre. Nunca pegar secretos en issues para «explicar» un incidente.
