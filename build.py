#!/usr/bin/env python3
import os.path as op
import os
import subprocess
import shutil

root_dir = op.abspath(op.dirname(__file__))
output_dir = op.join(root_dir, "build")
os.makedirs(output_dir, exist_ok=True)

subprocess.check_call(["cargo", "build", "--release"])
build_dir = op.join(root_dir, "target/release")
share_dir = op.join(root_dir, "share")

shutil.copy(op.join(build_dir, "moonwatcher"), output_dir)
shutil.copy(op.join(share_dir, "config-sample.json"), op.join(output_dir, "config.json"))
shutil.copy(op.join(share_dir, "install_unix.py"), output_dir)
shutil.copy(op.join(share_dir, "moonwatch-rs.service"), output_dir)
