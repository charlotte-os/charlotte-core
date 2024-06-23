import os

from const import DISABLE_LINT
import errors
import rust_ast


def inject(file_path: str, line: int) -> None:
    complete_file_path = f'charlotte_core/{file_path}'
    lines = _load_source_file(complete_file_path)
    ast = rust_ast.fetch(complete_file_path)
    print(f'{complete_file_path}: {line}')
    statement_line = rust_ast.find_statement_from_line(ast, line)

    if line > len(lines):
        os._exit(errors.ErrorCodes.InvalidLine)
    _insert(lines, statement_line)
    _save_source_file(complete_file_path, lines)


def _load_source_file(file_path: str) -> list:
    with open(file_path, 'r', encoding='utf-8') as f:
        lines = f.readlines()
        f.close()
    return lines


def _get_indent(line: str) -> str:
    return line[:len(line) - len(line.lstrip())]


def _insert(lines: list, line: int) -> None:
    print(line)
    if line == -1:
        return
    lines.insert(line - 1, f'{_get_indent(lines[line - 1])}{DISABLE_LINT}')

def _save_source_file(file_path: str, lines: list) -> None:
    with open(file_path, 'w', encoding='utf-8') as f:
        f.write(''.join(lines))
        f.close()
