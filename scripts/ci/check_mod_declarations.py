#!/usr/bin/env python3
# SPDX-FileCopyrightText: 2026 Kevin Monaghan
# SPDX-License-Identifier: MIT-0

"""Enforce STYLE-MOD-DECLS: declarations-only module roots.

Every handwritten ``src/**/mod.rs`` may contain only the copyright header,
module documentation, attributes, out-of-line module declarations, and
re-exports. Type definitions, functions, ``impl`` blocks, constants, statics,
macro definitions, and test bodies belong in focused sibling files.

Files under a ``generated/`` path segment are exempt when they carry the same
generator markers that ``scripts/ci/check_file_sizes.sh`` requires.
"""

from __future__ import annotations

import os
import re
import sys
from pathlib import Path

GENERATED_MARKERS = ("@generated", "Generator:", "Source:")
GENERATED_HEADER_LIMIT = 20

VISIBILITY = r"pub(?:\((?:crate|super|in\s+[a-z_][a-z0-9_:]*)\))?"
DECLARATION_ITEM = re.compile(rf"^(?P<vis>{VISIBILITY}\s+)?(?P<kind>mod|use)\b")
VIOLATION_REASON = (
    "STYLE-MOD-DECLS: mod.rs may contain only the copyright header, module "
    "documentation, attributes, out-of-line module declarations, and "
    "re-exports; this belongs in a focused sibling file"
)
INLINE_MODULE_REASON = (
    "STYLE-MOD-DECLS: module declarations in mod.rs must be out-of-line "
    "(end with ';')"
)
PRIVATE_IMPORT_REASON = (
    "STYLE-MOD-DECLS: mod.rs may carry only re-export `use` items with public "
    "or restricted visibility, not private implementation imports"
)


def is_generated(path: Path) -> bool:
    return "generated" in path.parts


def has_generated_markers(path: Path) -> bool:
    header = "".join(line for _, line in zip(range(GENERATED_HEADER_LIMIT), path.open()))
    return all(marker in header for marker in GENERATED_MARKERS)


def bracket_delta(line: str) -> int:
    return line.count("{") - line.count("}") + line.count("[") - line.count("]")


def violations(path: Path) -> list[tuple[int, str, str]]:
    found: list[tuple[int, str, str]] = []
    in_block_comment = False
    pending_depth = 0
    attribute_depth = 0

    for number, line in enumerate(path.read_text().splitlines(), start=1):
        stripped = line.strip()

        if in_block_comment:
            in_block_comment = "*/" not in stripped
            continue
        if not stripped or stripped.startswith("//"):
            continue
        if "/*" in stripped:
            in_block_comment = "*/" not in stripped.split("/*", 1)[1]
            continue
        if pending_depth > 0:
            pending_depth += bracket_delta(stripped)
            continue
        if attribute_depth > 0 or stripped.startswith("#[") or stripped.startswith("#!["):
            attribute_depth += bracket_delta(stripped)
            continue

        match = DECLARATION_ITEM.match(stripped)
        if match is None:
            found.append((number, stripped, VIOLATION_REASON))
            pending_depth = max(bracket_delta(stripped), 0)
            continue
        if match.group("kind") == "mod":
            if "{" in stripped:
                found.append((number, stripped, INLINE_MODULE_REASON))
                pending_depth = max(bracket_delta(stripped), 0)
            continue
        if match.group("vis") is None:
            found.append((number, stripped, PRIVATE_IMPORT_REASON))
        pending_depth = max(bracket_delta(stripped), 0)

    return found


def main() -> int:
    src = Path(__file__).resolve().parent.parent.parent / "src"
    if not src.is_dir():
        print(f"error: no src directory found at {src}", file=sys.stderr)
        return 1

    status = 0
    checked = 0
    exempted = 0
    github = "GITHUB_ACTIONS" in os.environ

    for path in sorted(src.rglob("mod.rs")):
        checked += 1
        if is_generated(path):
            if has_generated_markers(path):
                exempted += 1
                continue
            print(f"error: {path}: generated-path exemption requires {' and '.join(GENERATED_MARKERS)} in the first {GENERATED_HEADER_LIMIT} lines")
            status = 1
            continue
        for number, text, reason in violations(path):
            status = 1
            if github:
                print(f"::error file={path},line={number}::{reason}: {text}")
            else:
                print(f"error: {path}:{number}: {reason}: {text}", file=sys.stderr)

    print(f"checked {checked} module roots ({exempted} documented generated exemptions)")
    if status == 0:
        print("all module roots are declarations-only")
    return status


if __name__ == "__main__":
    sys.exit(main())
