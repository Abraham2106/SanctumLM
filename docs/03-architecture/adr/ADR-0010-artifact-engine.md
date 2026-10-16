# ADR-0010 — Motor de artefactos: ArtifactSpec, SVG y PPTX aislado

- **Estado:** proposed
- **Fecha:** 2026-10-16
- **Alcance:** producto — generación y exportación de artefactos de estudio (infografías, presentaciones, informes)
- **Tarea:** TASK-0013
- **Vínculos:** R-PROD-009, R-PROD-010, R-PROD-011, R-PROD-015; WBS-4.1, WBS-4.2; investigación §7.1–7.3, §8.1, §8.2, §11.5, §12; OPEN-012
- **Aprobación:** pendiente

## Contexto

SanctumLM debe generar **artefactos de estudio versionados** (infografías, presentaciones e informes) a partir de las mismas fuentes y `chunks` que alimentan el chat, con citas verificables y edición por secciones sin regenerar el documento completo (R-PROD-009, R-PROD-010, R-PROD-011). La investigación (§7–§8) rechaza el patrón «una sola imagen generada» como fuente de verdad: los textos, cifras y referencias deben permanecer **estructurados y editables**.

Fuerzas en tensión:

- **Evidencia vs. presentación:** el LLM propone estructura y redacción; el núcleo (Rust) valida citas, políticas y clasificación antes de persistir o renderizar.
- **Editabilidad vs. formatos de consumo:** PPTX y PDF son formatos de intercambio; la verdad estructural es JSON (`ArtifactSpec`) más revisiones en almacenamiento local (tabla `artifacts`, §12).
- **JavaScript vs. núcleo seguro:** PptxGenJS encaja en el ecosistema de plantillas de diapositivas (§8.1, §11.5) pero no debe ejecutarse en el renderer web ni recibir entradas no verificadas ni acceso a red.
- **Infografías:** §7.3 prioriza SVG/HTML/CSS con plantillas; las ilustraciones generativas son opcionales y dependen de OPEN-012 (aún **abierta**).

Este ADR **no** implementa código en `packages/artifacts`, **no** diseña §8.3 (tarjetas, cuestionarios, mapas) y **no** cierra OPEN-012.

## Decisión

### 1. Fuente de verdad: `ArtifactSpec` versionado

Todo artefacto persistido en el notebook guarda un **`ArtifactSpec` JSON tipado** en `spec_json`, con `type`, `revision` y `status` gestionados por el núcleo. Los formatos exportados (SVG, PNG, PPTX, PDF, DOCX, Markdown) son **proyecciones derivadas**, no sustitutos del spec.

Tipos v1 cubiertos por este ADR:

| `type` | Uso | Proyecciones previstas |
| --- | --- | --- |
| `infographic` | Explicación progresiva por secciones (§7) | SVG (canónico visual), PNG opcional, PDF opcional |
| `slides` | Presentación con guion y notas (§8.1) | PPTX (texto editable), PDF opcional |
| `report` | Informes, guías, fichas (§8.2) | Markdown, DOCX, PDF |

El spec **nunca** es una imagen raster única ni un blob opaco generado por difusión como almacenamiento principal.

### 2. Esquema conceptual de `ArtifactSpec`

Campos comunes (todos los tipos):

- `schema_version` (entero, p. ej. `1`)
- `type`: `infographic` \| `slides` \| `report`
- `title`, `language`, `audience` (opcional)
- `notebook_id`, `artifact_id` (asignados por el núcleo al persistir; no confiar en valores del LLM)
- `source_scope`: lista de `source_id` / `source_version_id` autorizados para este artefacto
- `evidence_mode`: `source_backed` \| `exploratory` (en `source_backed`, afirmaciones sustantivas exigen citas válidas)
- `sections[]`: unidad de edición incremental; cada sección tiene `id`, `heading`, cuerpo textual estructurado y metadatos de revisión
- `citations[]` o citas embebidas por afirmación: referencias a `chunk_id` + `quote` verificado + `source_version_id`
- `review_status`: `draft` \| `pending_validation` \| `validated` \| `export_ready`
- `limitations_and_doubts` (obligatorio en `report` cuando hay incertidumbre — R-PROD-009)

**Prohibiciones explícitas en el spec (rechazo en validación del núcleo):**

- Cualquier clave tipo `skip_security_checks`, `bypass_policy`, `raw_credentials`, tokens OAuth/API, contraseñas o material de keychain.
- Adjuntar secretos «para exportación» o URLs de red privada/localhost como destino de fetch desde el motor de artefactos.
- HTML/JS arbitrario de fuentes importadas incrustado en el spec (sanitización en proyección, no confianza en markup del LLM).

Ejemplo mínimo alineado con §7.2 (infografía):

```json
{
  "schema_version": 1,
  "type": "infographic",
  "title": "El ciclo del agua",
  "language": "es",
  "audience": "secundaria",
  "evidence_mode": "source_backed",
  "layout": "vertical-steps",
  "sections": [
    {
      "id": "paso-1",
      "heading": "Evaporación",
      "body": "El agua líquida pasa a vapor por aporte de energía.",
      "claims": [
        {
          "text": "El agua líquida pasa a vapor por aporte de energía.",
          "citation": { "chunk_id": "chk-174", "quote": "<texto verificado por el núcleo>" }
        }
      ],
      "visual": { "kind": "process-icon", "template_ref": "sun-water" }
    }
  ],
  "review_status": "pending_validation"
}
```

Para `slides`, cada sección mapea a una diapositiva (objetivo, título, bullets, `speaker_notes` opcional, `visual` opcional). Para `report`, las secciones siguen la plantilla elegida (informe técnico, guía, etc.) y pueden anidar subsecciones lógicas en el spec sin duplicar el outline como archivo separado canonico.

### 3. Pipeline de generación (núcleo Rust + cola)

Flujo acordado con §7.1, §8.1 y §8.2:

1. **Brief validado:** comando IPC `generate_artifact` (§12) con `notebook_id`, `artifact_type`, alcance de fuentes, idioma y preferencia de modelo — **sin** banderas que eludan política.
2. **Outline / guion:** el LLM propone estructura; el núcleo persiste borrador `ArtifactSpec` con `review_status: draft`.
3. **Evidencia por sección:** para cada afirmación en modo `source_backed`, el núcleo resuelve y valida `chunk_id` contra SQL de autorización (§11.3); sustituye citas inventadas o rechaza la sección.
4. **Edición humana por sección:** actualizaciones parciales al spec (UC-PROD-06); no se regenera el artefacto entero salvo acción explícita del usuario.
5. **Pase de validadores** (§4): solo entonces `export_ready`.
6. **Composición:** SVG vía plantillas; PPTX vía sidecar; MD/DOCX/PDF vía proyección desde el mismo spec validado.

Los trabajos largos usan cola asíncrona con progreso y cancelación (R-ENG-012, §3.3).

### 4. Validadores (diseño; pruebas alineadas con §15 i, j)

Antes de composición o exportación, el núcleo ejecuta validación **determinista** (no delegada al LLM):

| Validador | Aplica a | Criterio |
| --- | --- | --- |
| Texto y citas | Todos | Toda afirmación sustantiva en `source_backed` tiene cita resoluble; `quote` coincide con el fragmento autorizado |
| Coherencia outline | `report`, `slides` | Orden pedagógico, secciones referenciadas existen, sin huecos obligatorios |
| Solapamiento / layout | `infographic`, `slides` | Bounding boxes de plantilla sin solapamiento ilegible; rejilla mínima |
| Legibilidad | Todos | Longitud de línea, tamaño mínimo de fuente en plantilla, densidad en modo «documento para lectura» vs «presentación visual» (§8.1) |
| Desbordes PPTX | `slides` | Medición previa al write: título/cuerpo exceden caja → fallo exportación o marca de revisión (escenario §15 **i**) |
| Contraste | `infographic`, `slides` | Pares texto/fondo de plantilla cumplen umbral WCAG AA en tokens de diseño aprobados |
| Cifras y fórmulas | `infographic`, `report` | Cambios numéricos editables en el spec sin romper enlaces de cita (escenario §15 **j** a nivel de diseño) |

Los validadores producen informes estructurados (código, sección, severidad) para la UI; no mutan el spec sin acción del usuario salvo normalizaciones seguras (p. ej. recorte de espacios).

### 5. Composición SVG (infografías)

- **Entrada:** `ArtifactSpec` con `type: infographic` en estado `export_ready`.
- **Mecanismo:** plantillas SVG parametrizadas + diagramación algorítmica (layout declarado en spec: p. ej. `vertical-steps`). Texto y citas se inyectan como elementos editables del DOM/SVG, no como imagen plana.
- **Ubicación de implementación futura:** paquete `packages/artifacts` (§12), invocado desde el núcleo; **v1 recomendado:** composición invocable offline sin GPU.
- **Ilustraciones generativas:** solo si OPEN-012 se resuelve a favor y con política de egress explícita; hasta entonces, **recomendación de producto v1: SVG estructurado + iconografía de plantilla**, sin cerrar la cuestión OPEN-012. Las ilustraciones generativas, si se habilitan después, serían capa opcional referenciada por `visual.kind: generated-illustration` con asset externo versionado — nunca reemplazo del spec textual.

### 6. PptxGenJS en sidecar Node aislado

- **Proceso:** sidecar Node dedicado (mismo patrón de confianza que el conversor de documentos, §3.3 / §12), lanzado por el núcleo Tauri con binario empaquetado y versión fijada.
- **Contrato I/O:** una sola petición por exportación — JSON **`VerifiedSlideSpec`** (proyección ya validada del `ArtifactSpec` `slides`, sin campos libres de red) por stdin o archivo temporal en directorio confinado; salida PPTX en ruta controlada. Sin shell arbitrario.
- **Aislamiento:**
  - **Sin red** en el sidecar (firewall de proceso / variables de entorno; validación en CI con escenario offline).
  - **Sin fuentes crudas del usuario** ni descarga de fuentes: solo familias empaquetadas con la plantilla.
  - **Sin acceso a keychain, SQLite ni APIs de LLM**; el sidecar no parsea prompts ni lee fuentes del notebook.
  - El renderer de la UI **no** importa PptxGenJS.
- **Alternativa descartada para v1:** ejecutar PptxGenJS en el hilo del frontend Electron/Tauri webview (superficie XSS, dependencias amplias, difícil offline estricto).

El ejemplo §11.5 ilustra la forma del input; la implementación real debe incluir medición de desbordes, notas de ponente y estilos de plantilla.

### 7. Exportación Markdown, DOCX y PDF como proyección

- **Markdown:** serialización directa del `ArtifactSpec` `report` (y opcionalmente resumen de infografía/slides) con citas en formato enlazable al visor de fuentes.
- **DOCX/PDF:** generados desde el spec validado + plantillas de maquetación; **no** se almacenan como fuente de verdad salvo copia explícita del usuario en filesystem exportado.
- **Reimportación (R-PROD-015):** el paquete versionado del notebook incluye `spec_json` y revisiones de artefactos, no sustituye PPTX/DOCX exportados como modelo interno. Importar un paquete restaura specs y metadatos de citas; advertencias de secretos y permisos aplican al paquete, no al spec (que no debe contenerlos).

### 8. Pipeline de informes (R-PROD-009) — fases futuras encadenadas

Para `type: report`, el orden normativo de generación asistida es:

1. **Outline** con referencias previstas por sección.
2. **Generación por secciones** con citas por afirmación.
3. **Pase de coherencia** (terminología, no contradicciones entre secciones, alineación con outline).
4. **Sección «Limitaciones y dudas»** cuando el núcleo o el validador detecten incertidumbre, evidencia débil o conflictos entre fuentes.

Este pipeline puede ejecutarse en múltiples jobs encadenados; cada paso actualiza el mismo `ArtifactSpec` con incremento de `revision`.

### 9. Límites explícitos

- No se especifican tarjetas, cuestionarios ni mapas conceptuales (§8.3 no es requisito aprobado en G0).
- No se elige en este ADR el motor concreto de PDF/DOCX (solo que consume spec validado).
- La aceptación de ilustraciones generativas queda supeditada a resolución humana de OPEN-012.

## Alternativas evaluadas

| Opción | Descripción | Motivo de descarte / aplazamiento |
| --- | --- | --- |
| **A — Propuesta** | `ArtifactSpec` + SVG plantillas + PptxGenJS en sidecar sin red | Equilibra editabilidad, offline y frontera de confianza |
| Imagen única (difusión) como artefacto | Un PNG generado es el entregable | Incumple R-PROD-010/011, impide citas editables y escenario §15 j |
| PptxGenJS en el frontend | Menos procesos | Superficie de ataque amplia; difícil garantizar spec ya verificada |
| Sidecar Python para PPTX | python-pptx u otro | Duplica runtime; investigación ya ancla PptxGenJS [S22] |
| Solo Markdown en v1 | Sin PPTX/SVG | Insuficiente para R-PROD-010 y R-PROD-011 aprobados en G0 |
| Cerrar OPEN-012 como «solo SVG» en este ADR | Decisión de patrocinador | OPEN-012 sigue abierta; solo **recomendación** v1 sin cierre |

## Consecuencias

**Beneficios:** una sola fuente estructural para infografías, slides e informes; alineación con paquete de notebook (R-PROD-015); validación reproducible; sidecar acotado para JavaScript.

**Costos:** mantener esquema JSON versionado, plantillas SVG/PPTX, sidecar empaquetado multiplataforma y batería de tests de desborde/contraste.

**Riesgos residuales:** compatibilidad PPTX con LibreOffice vs PowerPoint; OPEN-012 sin resolver puede generar expectativas de ilustración generativa; elección concreta de librería PDF/DOCX pendiente de tarea de implementación.

**Pruebas previstas:** escenarios §15 **i** (desbordes detectados) y **j** (corrección numérica preservando citas); export offline; rechazo de spec con `skip_security_checks`; sidecar sin tráfico de red en prueba automatizada.

**Reversibilidad:** sustituir PptxGenJS o el compositor SVG no altera el contrato de `ArtifactSpec` si se mantiene `schema_version` y migraciones de revisiones.

## Evidencia y aprobación

- Investigación: §7.1–7.3, §8.1, §8.2, §11.5, §12, §15 (i, j).
- SRS: R-PROD-009, R-PROD-010, R-PROD-011, R-PROD-015.
- Registro OPEN-012: abierta; este ADR documenta recomendación SVG-only v1 sin resolverla.

Estado **proposed**; aprobación humana en G1 antes de implementar `packages/artifacts` y sidecar.

## Sustitución

Ninguna. Complementa la estructura de repositorio sugerida en investigación §12 y futuros ADR de IPC/almacenamiento si los contratos cambian.
