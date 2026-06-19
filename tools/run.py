#!/usr/bin/env python3
import argparse
import shlex
import subprocess
import sys
from pathlib import Path

argparser = argparse.ArgumentParser()
argparser.add_argument(
    "projects",
    nargs="*",
    type=Path,
    help="goboscript project directories (default: playground/)",
    default=[Path("playground/")],
)
argparser.add_argument("--release", "-r", action="store_true", help="use release mode")
argparser.add_argument(
    "--parallel",
    "-p",
    action="store_true",
    help="use GNU parallel to build all projects",
)
argparser.add_argument(
    "--validate",
    "-v",
    action="store_true",
    help="validate project.json using the official schema",
)
args = argparser.parse_args()

if returncode := subprocess.run(
    shlex.split("cargo build" + (" --release" if args.release else ""))
).returncode:
    sys.exit(returncode)

goboscript = Path("target")
goboscript = goboscript.joinpath("release" if args.release else "debug", "goboscript")
if args.parallel:
    if returncode := subprocess.run(
        ["parallel", goboscript, "build", "{}", ":::", *args.projects]
    ).returncode:
        sys.exit(returncode)
else:
    for project in args.projects:
        if returncode := subprocess.run(
            [goboscript, "build", project]
        ).returncode:
            sys.exit(returncode)

sb3py = Path(__file__).parent.joinpath("sb3.py")
for project in args.projects:
    if returncode := subprocess.run(
        [
            sys.executable,
            sb3py,
            project.joinpath(project.stem + ".sb3"),
            *(["-v"] if args.validate else []),
        ]
    ).returncode:
        sys.exit(returncode)
