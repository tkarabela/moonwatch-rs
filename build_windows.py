#!/usr/bin/env python3
"""
Build x86_64-pc-windows-gnu using Cross.

This is for cross-compiling from Linux to Windows.

"""

import os.path as op
import os
import subprocess
import shutil

TARGET_TRIPLE = "x86_64-pc-windows-gnu"

root_dir = op.abspath(op.dirname(__file__))
output_dir = op.join(root_dir, f"build/{TARGET_TRIPLE}")
os.makedirs(output_dir, exist_ok=True)

subprocess.check_call(["cross", "build", "--locked", "--release", "--target", f"{TARGET_TRIPLE}"])
build_dir = op.join(root_dir, f"target/{TARGET_TRIPLE}/release")
share_dir = op.join(root_dir, "share")

shutil.copy(op.join(build_dir, "moonwatcher.exe"), output_dir)
shutil.copy(op.join(share_dir, "default-config-windows.json"), op.join(output_dir, "config.json"))
shutil.copy(op.join(share_dir, "install_windows.bat"), output_dir)
