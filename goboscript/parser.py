from lark import Lark
from importlib.resources import read_text


parser = Lark(read_text("resources", "grammer.lark"))
