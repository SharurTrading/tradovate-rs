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
import sys
from pathlib import Path

from rust_tokens import CLOSING, RustTokenError, Token, group_end, tokenize

GENERATED_MARKERS = ("@generated", "Generator:", "Source:")
GENERATED_HEADER_LIMIT = 20

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
    with path.open(encoding="utf-8") as source:
        header = "".join(line for _, line in zip(range(GENERATED_HEADER_LIMIT), source))
    return all(marker in header for marker in GENERATED_MARKERS)


def item_end(tokens: list[Token], start: int, *, body_ends_item: bool) -> tuple[int, bool]:
    """Skip one item, retaining its outer brace/semicolon boundary."""
    position = start
    generic_depth = 0
    while position < len(tokens):
        text = tokens[position].text
        if text == ';':
            return position + 1, False
        if text in CLOSING:
            position = group_end(tokens, position)
            if text == '{' and body_ends_item and generic_depth == 0:
                if position < len(tokens) and tokens[position].text == ';':
                    position += 1
                return position, True
        else:
            # Const-generic defaults can contain braces inside item headers.
            if body_ends_item and text == '<':
                generic_depth += 1
            elif body_ends_item and text == '>' and generic_depth:
                generic_depth -= 1
            position += 1
    raise RustTokenError(tokens[start].line, 'unterminated item')


def item_violations(tokens: list[Token], lines: list[str]) -> list[tuple[int, str, str]]:
    found: list[tuple[int, str, str]] = []
    position = 0
    while position < len(tokens):
        # Attributes are token groups, including nested cfg_attr and opaque docs.
        if tokens[position].text == '#':
            attribute = position + 1
            if attribute < len(tokens) and tokens[attribute].text == '!':
                attribute += 1
            if attribute < len(tokens) and tokens[attribute].text == '[':
                position = group_end(tokens, attribute)
                continue

        start = position
        visible = tokens[position].text == 'pub'
        if visible:
            position += 1
            if position < len(tokens) and tokens[position].text == '(':
                position = group_end(tokens, position)
        if position == len(tokens):
            raise RustTokenError(tokens[start].line, 'missing item after visibility')

        kind = tokens[position].text
        # Const/static/type expressions can have outer brace groups before ';'.
        # A const function instead ends at its body, like an ordinary function.
        const_function = (kind == 'const' and position + 1 < len(tokens)
                          and tokens[position + 1].text in {'fn', 'unsafe', 'extern', 'async'})
        semicolon_item = kind in {'use', 'const', 'static', 'type'} and not const_function
        position, has_body = item_end(tokens, position, body_ends_item=not semicolon_item)

        reason = None
        if kind == 'mod':
            if has_body:
                reason = INLINE_MODULE_REASON
        elif kind == 'use':
            if not visible:
                reason = PRIVATE_IMPORT_REASON
        else:
            reason = VIOLATION_REASON
        if reason is not None:
            line = tokens[start].line
            found.append((line, lines[line - 1].strip(), reason))
    return found


def violations(path: Path) -> list[tuple[int, str, str]]:
    source = path.read_text(encoding="utf-8")
    lines = source.split('\n')
    try:
        return item_violations(tokenize(source), lines)
    except RustTokenError as error:
        return [(error.line, lines[error.line - 1].strip(),
                 f'STYLE-MOD-DECLS: cannot classify Rust items: {error}')]


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
        if is_generated(path.relative_to(src)):
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
