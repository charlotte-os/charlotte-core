import os

from const import DISABLE_LINT
import errors


def inject(file_path: str, line: int) -> None:
    complete_file_path = f'charlotte_core/{file_path}'
    lines = _load_source_file(complete_file_path)
    if line > len(lines):
        os._exit(errors.ErrorCodes.InvalidLine)
    _find_place_to_insert(lines, line)
    _save_source_file(complete_file_path, lines)


def _load_source_file(file_path: str) -> list:
    with open(file_path, 'r', encoding='utf-8') as f:
        lines = f.readlines()
        f.close()
    return lines


def _get_indent(line: str) -> str:
    return line[:len(line) - len(line.lstrip())]


def _find_place_to_insert(lines: list, line: int) -> None:
    line_num = line
    while line_num > 0:
        for scope in ('fn', 'trait'):
            if scope not in lines[line_num]:
                continue
            print(lines[line_num])
            if DISABLE_LINT in lines[line_num - 1]:
                continue
            lines.insert(line_num, f'{_get_indent(lines[line_num])}{DISABLE_LINT}')
            break

        line_num -= 1


def _save_source_file(file_path: str, lines: list) -> None:
    with open(file_path, 'w', encoding='utf-8') as f:
        f.write(''.join(lines))
        f.close()
