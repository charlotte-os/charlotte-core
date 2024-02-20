#!/usr/bin/env python3
# -*- coding: utf-8 -*-

import os
import sys
import subprocess

# When adding a new target, add the target to the TARGETS list
TARGETS = ["x86_64-unknown-none", "aarch64-unknown-none", "riscv64gc-unknown-none-elf"]
TARGET_RESULTS = {}


def run_grep(lookup, filename) -> str:
    """Run grep on the file and return the result"""
    try:
        return subprocess.run(
            ["grep", "--recursive", lookup, filename],
            stdout=subprocess.PIPE,
            check=True,
        ).stdout.decode("utf-8")
    except subprocess.CalledProcessError:
        return ""


def check_code():
    """Check the project"""
    grep_res = run_grep("allow(unused)", "./charlotte_core")

    for target in TARGETS:
        print(f"Checking target: {target}")
        os.environ["TARGET"] = target
        try:
            subprocess.run(
                [
                    "cargo",
                    "check",
                    "--target",
                    target,
                    "--manifest-path",
                    "charlotte_core/Cargo.toml",
                ],
                check=True,
                stderr=subprocess.PIPE,
                stdout=subprocess.PIPE,
            )
            subprocess.run(
                [
                    "cargo",
                    "doc",
                    "--target",
                    target,
                    "--manifest-path",
                    "charlotte_core/Cargo.toml",
                ],
                check=True,
                stderr=subprocess.PIPE,
                stdout=subprocess.PIPE,
            )
        except subprocess.CalledProcessError:
            target_result = "Failed"
        else:
            target_result = "Ok"
        TARGET_RESULTS[target] = target_result

    print("\n\nResults:")
    print("--------")
    if grep_res:
        print(
            "Unused code warning supression detected! Please check them and remove if not needed."
        )
        print("Affected files:")
        print(grep_res)
    print("Target results:")
    for target, result in TARGET_RESULTS.items():
        print(f"{target}: {result}")


if __name__ == "__main__":
    check_code()
