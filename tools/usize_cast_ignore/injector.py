import os

from const import DISABLE_LINT
import errors
import parser


def inject(file_path: str, line: int) -> None:
    complete_file_path = f'charlotte_core/{file_path}'
    lines = _load_source_file(complete_file_path)
    ranges = parser.analyze_rust_file(lines, cache_id=complete_file_path)

    if line > len(lines):
        os._exit(errors.ErrorCodes.InvalidLine)
    _insert(lines, ranges, line)
    _save_source_file(complete_file_path, lines)


def _load_source_file(file_path: str) -> list:
    with open(file_path, 'r', encoding='utf-8') as f:
        lines = f.readlines()
        f.close()
    return lines


def _get_indent(line: str) -> str:
    return line[:len(line) - len(line.lstrip())]


def _insert(lines: list, ranges: dict, line: int) -> bool:
    for type_ in ranges:
        for range_ in ranges[type_]:
            if range_[0] <= line <= range_[1]:
                lines.insert(range_[0], f'{_get_indent(lines[range_[0]])}{DISABLE_LINT}')
                return True

    return False


def _save_source_file(file_path: str, lines: list) -> None:
    with open(file_path, 'w', encoding='utf-8') as f:
        f.write(''.join(lines))
        f.close()
