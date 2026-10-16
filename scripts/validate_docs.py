#!/usr/bin/env python3
"""Comprobaciones conservadoras y sin dependencias de la línea base docs-as-code."""

from __future__ import annotations

import re
import sys
from pathlib import Path
from urllib.parse import unquote

ROOT = Path(__file__).resolve().parents[1]
REQUIRED = [
    "README.md", "AGENTS.md", "CONTRIBUTING.md",
    "docs/README.md", "docs/00-project/CHARTER.md",
    "docs/01-requirements/SRS.md", "docs/01-requirements/TRACEABILITY.md",
    "docs/02-planning/WBS.md", "docs/02-planning/ROADMAP.md",
    "docs/03-architecture/ARCHITECTURE.md",
    "docs/03-architecture/adr/README.md",
    "docs/04-quality/TEST_PLAN.md",
    "docs/05-agents/OPERATING_MODEL.md",
    "docs/05-agents/WORKTREE_PROTOCOL.md",
    ".agents/GOAL.md", ".agents/TASKS.md", ".agents/STATUS.md",
]
LINK_RE = re.compile(r"(?<!!)\[[^\]]+\]\(([^)]+)\)")
REQ_RE = re.compile(r"\bR-(?:ENG|NFR)-\d{3}\b")
ADR_RE = re.compile(r"\bADR-\d{4}\b")
issues = []

for relative in REQUIRED:
    if not (ROOT / relative).is_file():
        issues.append(f"Falta archivo requerido: {relative}")

for file in ROOT.rglob("*.md"):
    if any(part in {".git", "node_modules", ".venv"} for part in file.parts):
        continue
    content = file.read_text(encoding="utf-8")
    for raw in LINK_RE.findall(content):
        destination = raw.strip().split(" ", 1)[0].strip("<>")
        if not destination or destination.startswith(("https://", "http://", "mailto:", "#")):
            continue
        path_part = unquote(destination.split("#", 1)[0].split("?", 1)[0])
        if not path_part:
            continue
        if not (file.parent / path_part).exists():
            issues.append(f"{file.relative_to(ROOT)}: enlace roto a {destination}")

srs_path = ROOT / "docs/01-requirements/SRS.md"
matrix_path = ROOT / "docs/01-requirements/TRACEABILITY.md"
if srs_path.is_file() and matrix_path.is_file():
    srs_ids = set(REQ_RE.findall(srs_path.read_text(encoding="utf-8")))
    matrix_ids = set(REQ_RE.findall(matrix_path.read_text(encoding="utf-8")))
    for req_id in sorted(srs_ids - matrix_ids):
        issues.append(f"Requisito sin trazabilidad: {req_id}")
    for req_id in sorted(matrix_ids - srs_ids):
        issues.append(f"Requisito de matriz no definido en SRS: {req_id}")

adr_dir = ROOT / "docs/03-architecture/adr"
index = adr_dir / "README.md"
if index.is_file():
    indexed = set(ADR_RE.findall(index.read_text(encoding="utf-8")))
    defined = {m.group() for p in adr_dir.glob("ADR-*.md") if (m := ADR_RE.search(p.name))}
    for adr_id in sorted(defined - indexed):
        issues.append(f"ADR sin índice: {adr_id}")

if issues:
    print("Falló validación documental:")
    for issue in issues:
        print(f" - {issue}")
    sys.exit(1)
print(f"OK: {len(list(ROOT.rglob('*.md')))} archivos Markdown revisados; "
      "enlaces locales, requisitos iniciales e índice ADR verificados.")
