#!/usr/bin/env python3
import argparse
import shutil
import subprocess
import sys
from pathlib import Path
from zipfile import ZipFile

argparser = argparse.ArgumentParser()
argparser.add_argument("projects", nargs="+", type=Path, help=".sb3 project files")
argparser.add_argument(
    "--diff",
    "-d",
    action="store_true",
    help="show diff of project.json, atleast two projects must be specified",
)
argparser.add_argument(
    "--validate",
    "-v",
    action="store_true",
    help="validate project.json using the official schema",
)
argparser.add_argument(
    "--patch",
    "-p",
    action="store_true",
    help="patch existing project.json into .sb3 file",
)

if "--" in sys.argv:
    idx = sys.argv.index("--")
    args = argparser.parse_args(sys.argv[1:idx])
    extra = sys.argv[idx + 1 :]
else:
    args = argparser.parse_args(sys.argv[1:])
    extra = []


pathids: list[Path] = []
for path in args.projects:
    pathid: Path = path.parent.joinpath(f"{path.stem}.json")
    pathids.append(pathid)
    if args.patch:
        with ZipFile(path, "w") as zf:
            try:
                zf.write(pathid, "project.json")
            except (FileNotFoundError, PermissionError) as err:
                sys.stderr.write(f"error: failed to patch {err}\n")
                sys.exit(1)
    with ZipFile(path) as zf:
        with zf.open("project.json") as f:
            with pathid.open("wb") as f2:
                shutil.copyfileobj(f, f2)
if returncode := subprocess.run(
    ["prettier", "-w", *pathids], stdout=subprocess.DEVNULL
).returncode:
    sys.exit(returncode)


if args.diff:
    if len(pathids) < 2:
        sys.stderr.write("error: --diff requires atleast two projects\n")
        sys.exit(1)
    subprocess.run(["delta", *extra, "--side-by-side", *pathids])

if args.validate:
    sb3ts = Path(__file__).parent.joinpath("sb3.ts")
    for pathid in pathids:
        if returncode := subprocess.run(
            ["bun", "--bun", "run", sb3ts, pathid]
        ).returncode:
            sys.exit(returncode)
