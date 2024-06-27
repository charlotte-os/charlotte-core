from collections.abc import Generator

import json
import subprocess

from const import CLIPPY_COMMAND


def fetch() -> Generator[dict, None, None]:
    lints = _run_clippy()[:-1].split(b'\n')
    lints.reverse()
    for lint in lints:
        if lint is None:
            continue
        yield _bytes_to_dict(lint)


def _run_clippy() -> bytes:
    return subprocess.check_output(
        CLIPPY_COMMAND.split(),
    )


def _bytes_to_dict(json_: bytes) -> dict:
    return json.loads(json_)
