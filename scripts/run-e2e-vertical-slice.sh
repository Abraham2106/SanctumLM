#!/usr/bin/env bash
# TASK-0037 — ejecuta evidencia E2E núcleo (import → FTS → chat notebook).
# No requiere Tauri UI ni Ollama; usa adaptador fake en tests de integración.
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

ARTIFACT_DIR="${ARTIFACT_DIR:-$ROOT/tests/e2e/artifacts}"
mkdir -p "$ARTIFACT_DIR"
STAMP="$(date -u +%Y%m%dT%H%M%SZ)"
LOG="$ARTIFACT_DIR/e2e-vertical-slice-${STAMP}.log"
META="$ARTIFACT_DIR/e2e-vertical-slice-${STAMP}.meta"

{
  echo "task=TASK-0037"
  echo "timestamp_utc=$STAMP"
  echo "git_commit=$(git rev-parse HEAD)"
  echo "git_branch=$(git rev-parse --abbrev-ref HEAD)"
  echo "rustc=$(rustc --version 2>/dev/null || echo n/a)"
} >"$META"

echo "== SanctumLM E2E vertical slice (TASK-0037) ==" | tee "$LOG"
echo "commit: $(git rev-parse --short HEAD)" | tee -a "$LOG"
echo "log: $LOG" | tee -a "$LOG"
echo | tee -a "$LOG"

set -o pipefail
cargo test -p sanctumlm-desktop e2e_task0037 -- --nocapture 2>&1 | tee -a "$LOG"
STATUS=${PIPESTATUS[0]}

echo | tee -a "$LOG"
echo "exit_code=$STATUS" | tee -a "$LOG"
echo "artifact_log=$LOG" | tee -a "$LOG"
echo "artifact_meta=$META" | tee -a "$LOG"

exit "$STATUS"
