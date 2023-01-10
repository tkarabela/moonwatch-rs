#!/usr/bin/env python3
"""
Build x86_64-unknown-linux-gnu using local Cargo.

This is for native compilation on Linux.

"""

import os.path as op
import os
import subprocess
import shutil

TARGET_TRIPLE = "x86_64-unknown-linux-gnu"

root_dir = op.abspath(op.dirname(__file__))
output_dir = op.join(root_dir, f"build/{TARGET_TRIPLE}")
os.makedirs(output_dir, exist_ok=True)

subprocess.check_call(["cargo", "build", "--locked", "--release", "--target", f"{TARGET_TRIPLE}"])
build_dir = op.join(root_dir, f"target/{TARGET_TRIPLE}/release")
share_dir = op.join(root_dir, "share")

shutil.copy(op.join(build_dir, "moonwatcher"), output_dir)
shutil.copy(op.join(share_dir, "default-config-unix.json"), op.join(output_dir, "config.json"))
shutil.copy(op.join(share_dir, "install_unix.py"), output_dir)
shutil.copy(op.join(share_dir, "moonwatch-rs.service"), output_dir)
