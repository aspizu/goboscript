from lark import Lark
from importlib import resources

parser = Lark(resources.read_text("data", "grammer.lark"), start="_start")
parse = parser.parse
