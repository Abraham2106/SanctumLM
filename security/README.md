# security — amenazas y pruebas futuras

Espacio reservado para threat model ejecutable y pruebas de regresión de seguridad (WBS-3.3, ADR-0011).

## Alcance previsto (no implementado en TASK-0018)

- Tests de **SSRF**: conectores y workers no deben seguir redirects a rangos privados ni metadatos cloud sin política.
- Tests de **egress**: con red deshabilitada o política deny, ningún tráfico saliente desde el núcleo sin decisión explícita.
- Escenarios alineados con KPI-1 (R-PROD-101) y escenario §15 (c) modelo local caído sin tráfico externo.

## Restricciones de este directorio

- Solo pruebas y fixtures **defensivos**; sin exploits, payloads ofensivos ni credenciales reales.
- Los secretos de prueba serán sintéticos y rotados en tareas dedicadas.
