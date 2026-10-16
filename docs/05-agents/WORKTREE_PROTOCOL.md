# Protocolo obligatorio de Git worktrees

## Preparación del planner

Toda tarea tiene `TASK-####` único, rama base (`main` u otra aprobada), dueño y lista de archivos reservados. Solo tareas `ready` se asignan. Revisar `git worktree list` para evitar reusar rutas.

## Inicio del ejecutor

Desde el checkout principal del repositorio:

```bash
git fetch origin
git status --short
git worktree list
git worktree add ../SanctumLM-wt-TASK-0001 -b agent/TASK-0001-descripcion origin/main
git -C ../SanctumLM-wt-TASK-0001 status --short
```

Cambiar `0001` y `descripcion` por la tarea real; no copiar el ejemplo como un ID disponible. Si la rama ya existe, **no** sobrescribirla: consultar al planner. Los nombres de carpeta se adaptan a la ubicación real.

El ejecutor hace todos los cambios **dentro** de ese directorio y solo a archivos autorizados. No usa `git checkout` en el worktree de otro agente. No hace `git stash` sobre un checkout ajeno.

## Antes del PR

```bash
git -C ../SanctumLM-wt-TASK-0001 diff --check
git -C ../SanctumLM-wt-TASK-0001 status --short
git -C ../SanctumLM-wt-TASK-0001 add -A
git -C ../SanctumLM-wt-TASK-0001 commit -m "feat: TASK-0001 resultado verificable"
git -C ../SanctumLM-wt-TASK-0001 push -u origin agent/TASK-0001-descripcion
```

El commit de ejemplo puede ser `docs`, `fix`, `test` o `feat` según el cambio. Ejecutar pruebas y actualizar evidencia **antes** del commit/PR. Abrir PR hacia la rama base sin merge automático no aprobado.

## Conflictos y sincronización

Un worktree no comparte sus archivos cambiados con otros worktrees, aunque sí comparte objetos y referencias Git. No usar `.agents/STATUS.md` como bus de mensajes en tiempo real. Comunicación en issue/PR; snapshots solo los consolida el planner. Antes de rebase sobre una base que avanzó, verificar cambios, pruebas y autorización. No usar `push --force` en ramas compartidas; cualquier rebase publicado requiere política acordada.

Si dos tareas necesitan modificar el mismo archivo/contrato, secuenciarlas o separarlas en cambios compatibles antes de ejecutarlas; resolver conflictos en el worktree del dueño de integración, nunca en el checkout de `main`.

## Finalización y limpieza

Solo después de confirmar PR fusionado, estado registrado y ausencia de cambios no versionados:

```bash
git worktree list
git worktree remove ../SanctumLM-wt-TASK-0001
git branch -d agent/TASK-0001-descripcion
```

La supresión de rama remota es una acción distinta y requiere la política de retención. No usar `git worktree remove --force` o borrados recursivos para «limpiar» problemas. Las tareas fallidas se registran y se retienen hasta inspección.

## Ejecución remota o en CI

En GitHub Actions no existe un worktree compartido persistente entre jobs; cada job usa su propio entorno. No prometer visibilidad mutua. Para orquestación distribuida, elegir explícitamente un canal aprobado y autenticarlo (ADR-0004).
