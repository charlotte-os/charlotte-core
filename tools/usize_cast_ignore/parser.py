from typing import Dict

import re


def analyze_rust_file(lines: list, cache_id: str) -> dict:
    results: Dict[str, list] = {
        'functions': [],
        'structs': [],
        'enums': [],
        'static_vars': []
    }

    patterns = {
        'function': re.compile(r'^\s*(pub\s+)?(async\s+)?fn\s+[\w_]+\s*[(<]'),
        'struct': re.compile(r'^\s*(pub\s+)?struct\s+[\w_]+\s*[{;]'),
        'enum': re.compile(r'^\s*(pub\s+)?enum\s+[\w_]+\s*[{;]'),
        'static_var': re.compile(r'^\s*(pub\s+)?static\s+[\w_]+\s*[:=]')
    }

    for index, line in enumerate(lines):
        for key, pattern in patterns.items():
            if pattern.match(line):
                start_line = index + 1
                if key in ('struct', 'enum', 'function',):
                    end_line = _find_end_line(lines, start_line, index)
                else:
                    # Static variables
                    end_line = index + 1
                results[f'{key}s'].append((start_line, end_line))

    return results


def _find_end_line(lines, start_line, start_index):
    """Find the ending line of a block starting from start_line"""
    open_braces = 0
    for i in range(start_line, len(lines)):
        open_braces += lines[i].count('{')
        open_braces -= lines[i].count('}')
        if open_braces == 0:
            return i + 1
    return len(lines)
