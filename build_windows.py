#!/usr/bin/env python3
"""
Build x86_64-pc-windows-gnu using Cross.

This is for cross-compiling from Linux to Windows.

Prerequisites:

    sudo apt install zip  # and docker or podman
    cargo install cross --git https://github.com/cross-rs/cross
    cargo update

"""

import os.path as op
import os
import subprocess
import shutil

TARGET_TRIPLE = "x86_64-pc-windows-gnu"
PKGID_URL = subprocess.check_output(["cargo", "pkgid"]).decode("utf-8").strip()
PACKAGE_NAME_AND_VERSION = PKGID_URL.split("/")[-1].replace("#", "_")
ARCHIVE_NAME = f"{PACKAGE_NAME_AND_VERSION}_Windows-x86-64"
OUTPUT_ARCHIVE_PATH = op.abspath(f"build/{ARCHIVE_NAME}.zip")

output_build_dir = "./build"
root_dir = op.abspath(op.dirname(__file__))
output_dir = op.join(root_dir, f"build/{ARCHIVE_NAME}")
if op.exists(output_dir):
    shutil.rmtree(output_dir)
os.makedirs(output_dir)

subprocess.check_call(["cross", "build", "--locked", "--release", "--target", f"{TARGET_TRIPLE}"])
build_dir = op.join(root_dir, f"target/{TARGET_TRIPLE}/release")
share_dir = op.join(root_dir, "share")

shutil.copy(op.join(build_dir, "moonwatcher.exe"), output_dir)
shutil.copy(op.join(share_dir, "default-config-windows.json"), op.join(output_dir, "config.json"))
shutil.copy(op.join(share_dir, "install_windows.bat"), output_dir)

if op.exists(OUTPUT_ARCHIVE_PATH):
    os.remove(OUTPUT_ARCHIVE_PATH)
subprocess.check_call(["zip", "-r", OUTPUT_ARCHIVE_PATH, ARCHIVE_NAME], cwd=output_build_dir)
print("Created archive with build:", OUTPUT_ARCHIVE_PATH)
