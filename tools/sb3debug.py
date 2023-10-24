#!/usr/bin/env python
from __future__ import annotations
import json
from sys import argv
from pathlib import Path
from zipfile import ZipFile

with Path("project.json").open("w") as f, ZipFile(argv[1]) as zf:
    f.write(json.dumps(json.loads(zf.read("project.json")), indent=2))
