#!/usr/bin/env python3
"""Comprobaciones conservadoras y sin dependencias de la línea base docs-as-code."""

from __future__ import annotations

import re
import sys
from pathlib import Path
from urllib.parse import unquote

if hasattr(sys.stdout, "reconfigure"):
    sys.stdout.reconfigure(encoding="utf-8")

ROOT = Path(__file__).resolve().parents[1]
REQUIRED = [
    "README.md", "AGENTS.md", "CONTRIBUTING.md",
    "docs/README.md", "docs/00-project/CHARTER.md",
    "docs/00-project/OPEN_QUESTIONS.md", "docs/00-project/GLOSSARY.md",
    "docs/01-requirements/SRS.md", "docs/01-requirements/TRACEABILITY.md",
    "docs/02-planning/WBS.md", "docs/02-planning/ROADMAP.md",
    "docs/03-architecture/ARCHITECTURE.md",
    "docs/03-architecture/adr/README.md",
    "docs/04-quality/TEST_PLAN.md",
    "docs/05-agents/OPERATING_MODEL.md",
    "docs/05-agents/WORKTREE_PROTOCOL.md",
    ".agents/GOAL.md", ".agents/TASKS.md", ".agents/STATUS.md",
    ".agents/DECISIONS.md",
]
SKIP_DIRS = {".git", "node_modules", ".venv"}
LINK_RE = re.compile(r"(?<!!)\[[^\]]+\]\(([^)]+)\)")
REQ_RE = re.compile(r"\bR-(?:ENG|NFR)-\d{3}\b")
ADR_RE = re.compile(r"\bADR-\d{4}\b")
TC_RE = re.compile(r"\bTC-(?:ENG|SEC|PROD)-\d{2,3}\b")
WBS_RE = re.compile(r"\bWBS-\d+\.\d+\b")
OPEN_RE = re.compile(r"\bOPEN-\d{3}\b")
OPEN_ROW_RE = re.compile(r"^\|\s*(OPEN-\d{3})\s*\|", re.MULTILINE)
ADR_STATUS_RE = re.compile(r"\*\*Estado:\*\*\s*(\w+)")
ADR_APPROVAL_RE = re.compile(r"\*\*Aprobación:\*\*\s*(.+)")
INDEX_ROW_RE = re.compile(r"^\|\s*\[(\d{4})\]\([^)]*\)\s*\|[^|]*\|\s*(\w+)\s*\|", re.MULTILINE)
DECISIONS_ROW_RE = re.compile(r"^\|\s*(ADR-\d{4})\s*\|[^|]*\|\s*(\w+)\s*\|", re.MULTILINE)
issues = []


def read(relative: str) -> str:
    path = ROOT / relative
    return path.read_text(encoding="utf-8") if path.is_file() else ""


def check_defined(kind: str, used: set[str], defined: set[str], where: str) -> None:
    for item in sorted(used - defined):
        issues.append(f"{where}: {kind} {item} no existe en su documento fuente")


for relative in REQUIRED:
    if not (ROOT / relative).is_file():
        issues.append(f"Falta archivo requerido: {relative}")

markdown_files = [
    f for f in ROOT.rglob("*.md") if not any(part in SKIP_DIRS for part in f.parts)
]

for file in markdown_files:
    content = file.read_text(encoding="utf-8")
    for raw in LINK_RE.findall(content):
        destination = raw.strip().split(" ", 1)[0].strip("<>")
        if not destination or destination.startswith(("https://", "http://", "mailto:", "#")):
            continue
        path_part = unquote(destination.split("#", 1)[0].split("?", 1)[0])
        if not path_part:
            continue
        if not (file.parent / path_part).exists():
            issues.append(f"{file.relative_to(ROOT).as_posix()}: enlace roto a {destination}")

srs = read("docs/01-requirements/SRS.md")
matrix = read("docs/01-requirements/TRACEABILITY.md")
if srs and matrix:
    srs_ids = set(REQ_RE.findall(srs))
    matrix_ids = set(REQ_RE.findall(matrix))
    for req_id in sorted(srs_ids - matrix_ids):
        issues.append(f"Requisito sin trazabilidad: {req_id}")
    for req_id in sorted(matrix_ids - srs_ids):
        issues.append(f"Requisito de matriz no definido en SRS: {req_id}")

test_plan = read("docs/04-quality/TEST_PLAN.md")
if matrix and test_plan:
    check_defined("caso", set(TC_RE.findall(matrix)), set(TC_RE.findall(test_plan)),
                  "docs/01-requirements/TRACEABILITY.md")

wbs_ids = set(WBS_RE.findall(read("docs/02-planning/WBS.md")))
if wbs_ids:
    for relative in ["docs/01-requirements/TRACEABILITY.md", ".agents/TASKS.md"]:
        check_defined("paquete", set(WBS_RE.findall(read(relative))), wbs_ids, relative)

open_register = ROOT / "docs/00-project/OPEN_QUESTIONS.md"
if open_register.is_file():
    open_ids = set(OPEN_ROW_RE.findall(open_register.read_text(encoding="utf-8")))
    for file in markdown_files:
        if file == open_register:
            continue
        used = set(OPEN_RE.findall(file.read_text(encoding="utf-8")))
        check_defined("cuestión", used, open_ids, file.relative_to(ROOT).as_posix())

adr_dir = ROOT / "docs/03-architecture/adr"
index = adr_dir / "README.md"
if index.is_file():
    index_text = index.read_text(encoding="utf-8")
    indexed = set(ADR_RE.findall(index_text))
    index_status = {f"ADR-{num}": status for num, status in INDEX_ROW_RE.findall(index_text)}
    decisions_status = dict(DECISIONS_ROW_RE.findall(read(".agents/DECISIONS.md")))
    for adr_file in sorted(adr_dir.glob("ADR-*.md")):
        match = ADR_RE.search(adr_file.name)
        if not match:
            continue
        adr_id = match.group()
        if adr_id not in indexed:
            issues.append(f"ADR sin índice: {adr_id}")
        text = adr_file.read_text(encoding="utf-8")
        status_match = ADR_STATUS_RE.search(text)
        status = status_match.group(1) if status_match else None
        if status is None:
            issues.append(f"{adr_id}: falta campo Estado")
            continue
        for source, table in (("índice ADR", index_status), ("DECISIONS.md", decisions_status)):
            if table.get(adr_id) != status:
                issues.append(f"{adr_id}: estado '{status}' no coincide con {source} "
                              f"('{table.get(adr_id)}')")
        approval = ADR_APPROVAL_RE.search(text)
        if status == "accepted" and (not approval or approval.group(1).startswith("pendiente")):
            issues.append(f"{adr_id}: accepted sin evidencia de aprobación")

if issues:
    print("Falló validación documental:")
    for issue in issues:
        print(f" - {issue}")
    sys.exit(1)
print(f"OK: {len(markdown_files)} archivos Markdown revisados; enlaces, archivos requeridos, "
      "IDs R/TC/WBS/OPEN y estado/aprobación de ADR verificados.")
