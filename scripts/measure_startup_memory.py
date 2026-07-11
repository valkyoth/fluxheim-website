#!/usr/bin/env python3
"""Run a command and print its peak child-process RSS in KiB."""

from __future__ import annotations

import os
import resource
import subprocess
import sys


def rss_kib(value: int, platform: str) -> int:
    if platform == "darwin":
        return (value + 1023) // 1024
    return value


def self_test() -> None:
    assert rss_kib(224_000, "linux") == 224_000
    assert rss_kib(229_376_000, "darwin") == 224_000
    print("startup memory unit conversion ok")


def main(arguments: list[str]) -> int:
    if arguments == ["--self-test"]:
        self_test()
        return 0
    if not arguments:
        print("usage: measure_startup_memory.py COMMAND [ARG ...]", file=sys.stderr)
        return 2

    environment = os.environ.copy()
    environment["FLUXHEIM_OTLP"] = "disabled"
    completed = subprocess.run(
        arguments,
        check=False,
        env=environment,
        stdout=sys.stderr,
        stderr=sys.stderr,
    )
    if completed.returncode != 0:
        return completed.returncode

    usage = resource.getrusage(resource.RUSAGE_CHILDREN)
    print(rss_kib(int(usage.ru_maxrss), sys.platform))
    return 0


if __name__ == "__main__":
    raise SystemExit(main(sys.argv[1:]))
