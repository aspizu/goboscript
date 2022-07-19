import sys
import project
from pathlib import Path

project_dir = Path(sys.argv[1])
output_pth = Path(sys.argv[2])

project.build_gm_project(project_dir).export(output_pth.as_posix())
# ^^^ FIXME: Gobomatic should switch to pathlib from strings
