import sys
from pathlib import Path
from zipfile import ZipFile

with ZipFile(Path(sys.argv[1])) as sb3:
  print(sb3.filelist)
  with Path("tools/dumped_project.json").open("wb") as f:
    f.write(sb3.read("project.json"))
