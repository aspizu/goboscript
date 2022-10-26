from pathlib import Path

from rich import print

from gparser.gparser import gparser
from gparser.gsprite_interpreter import gSpriteInterpreter

tree = gparser.parse(Path("demo/main.gs").read_text())
inter = gSpriteInterpreter(tree)
s = inter.to_gSprite("main.gs")
print(s)
