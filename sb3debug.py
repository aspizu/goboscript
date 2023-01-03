import json
from sys import argv
from zipfile import ZipFile

with open("project.json", "w") as f:
    with ZipFile(argv[1]) as zf:
        f.write(json.dumps(json.loads(zf.read("project.json")), indent=2))
