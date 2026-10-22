# SPDX-FileCopyrightText: 2026 Kevin Monaghan
# SPDX-License-Identifier: MIT-0

"""Rust lexical boundaries needed by the module-root policy checker.

Comments are discarded and literals become opaque tokens. This is deliberately
not a Rust syntax validator: rustc checks syntax, while the policy checker needs
only item keywords, visibility, and balanced token groups.
"""

import re
from typing import NamedTuple


class Token(NamedTuple):
    text: str
    line: int


class RustTokenError(ValueError):
    def __init__(self, line: int, reason: str):
        super().__init__(reason)
        self.line = line


RAW_STRING = re.compile(r'(?:br|cr|r)(?P<hashes>#*)"')
QUOTED_STRING = re.compile(r'[bc]?"')
CHARACTER = re.compile(r"b?'(?:\\(?:u\{[0-9a-fA-F_]+\}|x[0-9a-fA-F]{2}|[^\n])|[^'\\\n])'")
IDENTIFIER = re.compile(r'(?:r#)?[^\W\d]\w*')
CLOSING = {'(': ')', '[': ']', '{': '}'}


def block_comment_end(source: str, start: int, line: int) -> int:
    depth = 1
    position = start + 2
    while position < len(source):
        if source.startswith('/*', position):
            depth += 1
            position += 2
        elif source.startswith('*/', position):
            depth -= 1
            position += 2
            if depth == 0:
                return position
        else:
            position += 1
    raise RustTokenError(line, 'unterminated block comment')


def quoted_string_end(source: str, position: int, line: int) -> int:
    while position < len(source):
        if source[position] == '\\':
            position += 2
        elif source[position] == '"':
            return position + 1
        else:
            position += 1
    raise RustTokenError(line, 'unterminated string literal')


def tokenize(source: str) -> list[Token]:
    tokens: list[Token] = []
    position = 0
    line = 1
    while position < len(source):
        start = position
        if source[position].isspace():
            position += 1
        elif source.startswith('//', position):
            end = source.find('\n', position)
            position = len(source) if end == -1 else end
        elif source.startswith('/*', position):
            position = block_comment_end(source, position, line)
        elif match := RAW_STRING.match(source, position):
            terminator = '"' + match.group('hashes')
            end = source.find(terminator, match.end())
            if end == -1:
                raise RustTokenError(line, 'unterminated raw string literal')
            position = end + len(terminator)
            tokens.append(Token('<literal>', line))
        elif match := QUOTED_STRING.match(source, position):
            position = quoted_string_end(source, match.end(), line)
            tokens.append(Token('<literal>', line))
        elif match := CHARACTER.match(source, position):
            position = match.end()
            tokens.append(Token('<literal>', line))
        elif match := IDENTIFIER.match(source, position):
            position = match.end()
            tokens.append(Token(match.group(), line))
        else:
            position += 1
            tokens.append(Token(source[start], line))
        line += source.count('\n', start, position)
    return tokens


def group_end(tokens: list[Token], start: int) -> int:
    """Return the first token after a balanced (), [], or {} group."""
    closing: list[str] = []
    for position in range(start, len(tokens)):
        token = tokens[position]
        if token.text in CLOSING:
            closing.append(CLOSING[token.text])
        elif token.text in CLOSING.values():
            if not closing or closing.pop() != token.text:
                raise RustTokenError(token.line, 'mismatched token-group delimiter')
            if not closing:
                return position + 1
    raise RustTokenError(tokens[start].line, 'unterminated token group')
