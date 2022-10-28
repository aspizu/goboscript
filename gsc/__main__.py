from pathlib import Path
from sys import argv

from gparser.gbuild import build_gproject
from gparser.gexception import gError

try:
    build_gproject(Path(argv[1])).package(Path(argv[2]))
except gError as e:
    e.print()
    exit(1)
