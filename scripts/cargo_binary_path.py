#!/usr/bin/env python3
"""Print a named binary path from Cargo JSON build messages."""

from __future__ import annotations

import json
import sys
from collections.abc import Iterable


def binary_path(messages: Iterable[str], binary_name: str) -> str:
    executable = ""
    build_succeeded = False
    for line in messages:
        try:
            message = json.loads(line)
        except json.JSONDecodeError:
            continue
        target = message.get("target", {})
        if (
            message.get("reason") == "compiler-artifact"
            and target.get("name") == binary_name
            and "bin" in target.get("kind", [])
            and message.get("executable")
        ):
            executable = message["executable"]
        if message.get("reason") == "build-finished":
            build_succeeded = message.get("success") is True
    if not build_succeeded:
        raise ValueError("Cargo build did not finish successfully")
    if not executable:
        raise ValueError(f"Cargo did not emit binary {binary_name}")
    return executable


def self_test() -> None:
    expected = "/tmp/custom-target/release/fluxheim-website"
    messages = [
        json.dumps({"reason": "build-script-executed"}),
        json.dumps(
            {
                "reason": "compiler-artifact",
                "target": {"name": "fluxheim-website", "kind": ["bin"]},
                "executable": expected,
            }
        ),
        json.dumps({"reason": "build-finished", "success": True}),
    ]
    assert binary_path(messages, "fluxheim-website") == expected
    try:
        binary_path(messages[:-1], "fluxheim-website")
    except ValueError:
        pass
    else:
        raise AssertionError("unfinished Cargo build was accepted")
    print("Cargo executable path parsing ok")


def main(arguments: list[str]) -> int:
    if arguments == ["--self-test"]:
        self_test()
        return 0
    if len(arguments) != 1:
        print("usage: cargo_binary_path.py BINARY_NAME", file=sys.stderr)
        return 2
    try:
        print(binary_path(sys.stdin, arguments[0]))
    except ValueError as error:
        print(error, file=sys.stderr)
        return 1
    return 0


if __name__ == "__main__":
    raise SystemExit(main(sys.argv[1:]))
