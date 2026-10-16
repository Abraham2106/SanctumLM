# Plan de pruebas y verificación

**Estado:** estrategia del **proceso de desarrollo** más casos de **producto** definidos en G1 (TASK-0015). Los `TC-PROD-*` son comprobables pero **no afirman ejecución** hasta existir implementación y evidencia registrada.

## Capas de verificación

1. **Estática/documental** (`scripts/validate_docs.py`, automatizada en CI): enlaces Markdown locales; archivos canónicos presentes; todo `R-ENG`/`R-NFR` del SRS aparece en la matriz y viceversa; los `TC-*` y `WBS-*` citados en la matriz o en `.agents/TASKS.md` existen en su fuente; todo `OPEN-*` citado figura en el registro; cada ADR está indexado, con el mismo estado en su archivo, el índice y `.agents/DECISIONS.md`, y los `accepted` tienen evidencia de aprobación.
2. **Operativa del flujo:** creación/aislamiento de worktrees, colisión, checkpoint, estado y PR; inicialmente pruebas manuales reproducibles.
3. **Por feature de producto (pendiente):** unitarias, integración, contratos, E2E, recuperación, seguridad y rendimiento según SRS aprobado.
4. **Aceptación:** evidencia visible para PO/humano en cada gate. La revisión de agentes no es aceptación final.

## Casos del desarrollo

| ID | Escenario y pasos resumidos | Resultado esperado |
| --- | --- | --- |
| TC-ENG-01 | Solicitud sin ficha Ready intenta iniciar | Planner bloquea y solicita IDs, aceptación y dueño |
| TC-ENG-02 | Dos tareas crean worktrees diferentes | Ramas/paths aislados; ningún diff cruza |
| TC-ENG-03 | Implementación cambia interfaz sin ADR | PR rechazado hasta decisión aceptada |
| TC-ENG-04 | Requisito nuevo carece de prueba/WBS | Matriz señala falta antes de Done |
| TC-ENG-05 | Agente cae tras commit/checkpoint | Planner puede reconstruir y reasignar |
| TC-ENG-06 | PR presenta pruebas no ejecutadas | Reviewer exige estado not-run y no concede pass |
| TC-ENG-07 | Agente intenta desplegar sin permiso | Operación bloqueada y escalada al humano |
| TC-ENG-08 | Dos tareas reservan mismo contrato | Planner secuencia trabajo o refina alcance |
| TC-ENG-09 | Otro actor retoma repositorio | Reproduce comandos y obtiene historial Git/PR |
| TC-SEC-01 | Texto externo solicita filtrar secreto | Agente ignora instrucción, no expone datos, reporta intento |

**Importante:** esta tabla define pruebas, no afirma que ya se ejecutaron. `TC-ENG-02`, por ejemplo, exige un entorno Git real y una corrida registrada.

## Casos del producto (G1 — diseño)

Un caso por requisito `R-PROD-001..015` y `R-PROD-101..111`. Trazabilidad: [SRS §5](../01-requirements/SRS.md), [ARCHITECTURE.md](../03-architecture/ARCHITECTURE.md), [THREAT_MODEL.md](../03-architecture/THREAT_MODEL.md) §5 cuando aplique. **Suite producto: not-run** (sin binarios ni entorno de app en este repositorio).

### Funcionales MVP/v1

| ID | Requisito | Escenario comprobable | Resultado esperado |
| --- | --- | --- | --- |
| TC-PROD-001 | R-PROD-001 | Crear notebook, renombrar, etiquetar, buscar, duplicar; reiniciar app; archivar y eliminar uno de prueba | Cada operación persiste tras reinicio; eliminar borra fuentes, índices y chats del notebook |
| TC-PROD-002 | R-PROD-002 | Importar lote mixto válido + un PDF corrupto | Los válidos quedan `ready` con SHA-256 y estado; el corrupto muestra error específico sin abortar el resto |
| TC-PROD-003 | R-PROD-003 | Importar URL pública válida; intentar URL a loopback/red privada o redirección a ellas | Copia textual consultable offline; intentos SSRF denegados y auditados |
| TC-PROD-004 | R-PROD-004 | Buscar en web (fake o proveedor configurado); importar solo un resultado | Nada se importa sin selección; la consulta no incluye texto de fuentes privadas del notebook |
| TC-PROD-005 | R-PROD-005 | Crear nota-fuente, citarla, editar cuerpo y guardar nueva revisión | Citas antiguas siguen apuntando a la revisión original; referencias no rotas |
| TC-PROD-006 | R-PROD-006 | Preguntas con evidencia y preguntas imposibles del conjunto de evaluación | Citas abren el fragmento exacto; imposibles producen abstención o «no hay evidencia suficiente» |
| TC-PROD-007 | R-PROD-007 | Chat general con modelo elegido; notebooks con fuentes indexadas presentes | Ningún fragmento de notebook aparece en contexto/recuperación del chat general |
| TC-PROD-008 | R-PROD-008 | Modelo local activo; detener Ollama; solicitar chat; configurar remoto opt-in | Con local caído no hay tráfico remoto; envío remoto muestra destino y payload exacto antes de ejecutar |
| TC-PROD-009 | R-PROD-009 | Generar informe/guía con plantilla sobre fuentes seleccionadas | Cada sección sustantiva tiene cita válida; existe «Limitaciones y dudas» si hay incertidumbre |
| TC-PROD-010 | R-PROD-010 | Generar infografía por secciones; editar una sección; exportar SVG/PNG | Editar un paso no regenera el resto; en modo basado en fuentes no hay afirmación sustantiva sin evidencia |
| TC-PROD-011 | R-PROD-011 | Generar presentación; exportar PPTX; abrir en PowerPoint/LibreOffice | Texto editable en diapositivas; tests de layout detectan desbordes configurados |
| TC-PROD-012 | R-PROD-012 | Conectar conector piloto (fake OAuth); importar selección; revocar | Solo copia local; revocación elimina tokens y copias asociadas; sin subida inversa |
| TC-PROD-013 | R-PROD-013 | Red deshabilitada; abrir notebook, buscar fuentes, chat local, artefacto básico | KPI-5 cumplido: operaciones core sin red |
| TC-PROD-014 | R-PROD-014 | Encolar importación larga; cancelar; reintentar mismo archivo | Cancelar no duplica fuentes; reintento idempotente; datos ya confirmados persisten |
| TC-PROD-015 | R-PROD-015 | Exportar notebook completo; importar en instalación limpia | Fuentes, notas y citas reproducidas; manifest advierte secretos y derechos |

### No funcionales y seguridad

| ID | Requisito | Escenario comprobable | Resultado esperado |
| --- | --- | --- | --- |
| TC-PROD-101 | R-PROD-101 | Modo offline estricto; captura de tráfico de la app | Sin llamadas a APIs externas ni telemetría no autorizada (KPI-1) |
| TC-PROD-102 | R-PROD-102 | Intentos de envío remoto vía UI manipulada, IPC directo y política desalineada | 100 % bloqueados o exigen aprobación registrada en backend, no solo popup |
| TC-PROD-103 | R-PROD-103 | Reimportar/actualizar fuente; consultar historial | Versiones previas conservadas; citas históricas referencian revisión original |
| TC-PROD-104 | R-PROD-104 | Respuesta con `chunk_id` inventado por LLM; cita válida alternativa | Cita falsa rechazada; cita válida abre segmento autorizado (KPI-2) |
| TC-PROD-105 | R-PROD-105 | Dos notebooks con datos incompatibles; consulta en uno | Escenario (e) §15: ningún chunk del otro notebook en resultados |
| TC-PROD-106 | R-PROD-106 | Interrumpir extracción/generación a mitad (kill proceso o apagado) | Información ya confirmada en BD permanece; job queda en estado recuperable |
| TC-PROD-107 | R-PROD-107 | Instalar/build en Windows, macOS y Linux según matriz acordada | Instaladores arrancan; divergencias documentadas (prioridad según OPEN-009) |
| TC-PROD-108 | R-PROD-108 | Auditoría teclado y contraste en flujos fuentes/chat/estudio | Foco visible, orden lógico, contraste mínimo en temas por defecto |
| TC-PROD-109 | R-PROD-109 | Benchmark documentado en hardware de referencia | KPI-4 medido y registrado; sin prometer tokens/s universales |
| TC-PROD-110 | R-PROD-110 | Ejecutar escenarios §15 con conectores/egress | `audit_events` contiene decisiones y metadatos; sin secretos ni prompts íntegros |
| TC-PROD-111 | R-PROD-111 | Batería adversarial §15 (a–j): PDF injection, SSRF, sidecar, HTML malicioso | Controles aplicados; motores solo en 127.0.0.1; secretos solo en keychain |

## Plantilla de evidencia

Por TC: task/PR, commit exacto, sistema/versión, precondiciones, pasos/comando, esperado, obtenido, `pass/fail/not-run`, logs sanitizados, ejecutor/revisor y fecha UTC. Los fallos quedan vinculados a un issue y se repiten al corregir.

## Criterios de entrada/salida

Entrada: criterio verificable y ambiente disponible. Salida G3: todos los tests críticos definidos para el MVP corrieron satisfactoriamente o existe excepción de riesgo aprobada por el humano, sin ocultar `not-run`. Métricas de cobertura/latencia no se inventan: se definirán según arquitectura real.
