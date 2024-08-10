#!/usr/bin/env python3
import os.path as op
import os
import shutil
import subprocess
import sys

build_dir = op.abspath(op.dirname(__file__))
install_dir = op.expanduser("~/.moonwatch-rs")

print("Testing availability of dependencies (gnome-screensaver-command, xprintidle, xdotool)")
for cmd in (["gnome-screensaver", "-h"], ["xprintidle", "-v"], ["xdotool", "-v"]):
    try:
        subprocess.check_call(cmd)
    except subprocess.CalledProcessError:
        print("Warning -", cmd, "failed, please install it before using moonwatch-rs")
        sys.exit(1)

print("Stopping moonwatch-rs service")
rv = subprocess.call(["systemctl", "--user", "stop", "moonwatch-rs"])
print("systemctl returned code", rv)

print("Installing into", install_dir)
if not os.path.exists(install_dir):
    print("Creating directory", install_dir)
    os.makedirs(install_dir)

shutil.copy(op.join(build_dir, "moonwatcher"), install_dir)
if op.exists(op.join(install_dir, "config.json")):
    print("config.json already exists, not copying default")
else:
    print("copying default config.json")
    shutil.copy(op.join(build_dir, "config.json"), install_dir)

print("Setting up Systemd user service")
systemd_user_dir = op.expanduser("~/.config/systemd/user")
os.makedirs(systemd_user_dir, exist_ok=True)
shutil.copy(op.join(build_dir, "moonwatch-rs.service"), systemd_user_dir)

cmd = ["systemctl", "--user", "enable", "moonwatch-rs"]
print("Enabling moonwatch-rs service:", cmd)
subprocess.check_call(cmd)

print("Starting moonwatch-rs service")
rv = subprocess.call(["systemctl", "--user", "start", "moonwatch-rs"])
print("systemctl returned code", rv)
