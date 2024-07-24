import subprocess

from const import AST_COMMAND


def fetch(path: str) -> str:
    return _run_rustc(path)[6:].decode()


def _run_rustc(path: str) -> bytes:
    return subprocess.check_output(
        f"{AST_COMMAND} {path}".split(),
    )


def find_statement_from_line(ast, source_line) -> int:
    lines = ast.strip().split('\n')
    statement_found = None
    statement_start = -1

    for num, line in enumerate(lines):
        if 'stmt' in line.lower() and 'stmts' not in line.lower():
            statement_found = True
            continue

        if 'span' in line and statement_found is not None:
            spans = line.split('span: ')[-1].strip().split(':')
            #print(spans)
            if len(spans) != 5:
                continue
            if statement_found:
                statement_start = int(spans[1])
                statement_found = False
            if int(spans[1]) <= source_line <= int(spans[3]):
                print(line)
                print(num)
                print(f'{spans[1]} <= {source_line} <= {spans[3]}')
                break

    return statement_start
