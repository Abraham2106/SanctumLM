# Contribuir a SanctumLM

Leer [AGENTS.md](AGENTS.md), [índice](docs/README.md), [SRS](docs/01-requirements/SRS.md) y [modelo operativo](docs/05-agents/OPERATING_MODEL.md). El [glosario](docs/00-project/GLOSSARY.md) define los términos propios del proyecto.

## Flujo

1. Toda contribución está ligada a una tarea `TASK-####` con requisito, WBS, aceptación y permisos. El planner asigna trabajo y evita conflictos.
2. Trabajar en un worktree y rama propios siguiendo [WORKTREE_PROTOCOL](docs/05-agents/WORKTREE_PROTOCOL.md).
3. Ejecutar el validador documental (ver abajo) y las pruebas que correspondan.
4. Abrir PR y completar la [plantilla de PR](.github/PULL_REQUEST_TEMPLATE.md). El revisor no puede ser el mismo implementer.

## Reglas de contenido

- Requisitos y decisiones se cambian en PR con trazabilidad. Para un cambio de alcance, abrir RFC; para una decisión estructural, crear ADR.
- No inventar funciones del producto: hasta G0 la definición está pendiente. Si falta información, registrar una cuestión en [OPEN_QUESTIONS](docs/00-project/OPEN_QUESTIONS.md).
- No subir tokens, archivos locales, secretos ni datos reales sensibles.

## Validación local

```bash
python3 scripts/validate_docs.py   # Linux/macOS
python scripts/validate_docs.py    # Windows (o: py -3 scripts/validate_docs.py)
```

El mismo script corre en CI (`.github/workflows/docs-governance.yml`). Para producto, ejecutar además el pipeline real que se adopte tras G1. Registrar `not-run` donde no se haya ejecutado una prueba. La aprobación de un reviewer no es permiso para desplegar.
