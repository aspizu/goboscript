from pathlib import Path

from gbuild import build_gproject
from gerror import gError

try:
    project = build_gproject(Path("example"))
    project.package(Path("example/build.sb3"))
except gError as e:
    e.print()
    exit(1)
