from __future__ import annotations
import sys

# fmt: off
reset = "\033[0m"
bold = "\033[1m"
italic = "\033[3m"
ulined = "\033[4m"

black = "\033[30m"
red = "\033[31m"
green = "\033[32m"
yellow = "\033[33m"
blue = "\033[34m"
pink = "\033[35m"
cyan = "\033[36m"
white = "\033[37m"

bgblack = "\033[40m"
bgred = "\033[41m"
bggreen = "\033[42m"
bgyellow = "\033[43m"
bgblue = "\033[44m"
bgpink = "\033[45m"
bgcyan = "\033[46m"
bgwhite = "\033[47m"

brblack = "\033[90m"
brred = "\033[91m"
brgreen = "\033[92m"
bryellow = "\033[93m"
brblue = "\033[94m"
brpink = "\033[95m"
brcyan = "\033[96m"
brwhite = "\033[97m"

brbgblack = "\033[100m"
brbgred = "\033[101m"
brbggreen = "\033[102m"
brbgyellow = "\033[103m"
brbgblue = "\033[104m"
brbgpink = "\033[105m"
brbgcyan = "\033[106m"
brbgwhite = "\033[107m"
# fmt: on


def f():
    sys.stdout.flush()


def w(s: str):
    sys.stdout.write(s)


def wf(s: str):
    w(s)
    f()


def m(x: int, y: int):
    w(f"\u001b[{y+1};{x+1}H")


def ml(c: int):
    w(f"\033[{c}D")


def mr(c: int):
    w(f"\033[{c}C")


def mu(c: int):
    w(f"\033[{c}A")


def md(c: int):
    w(f"\033[{c}B")


def h():
    m(0, 0)


def c():
    w("\u001b[2J\u001b[3J")


def chf():
    c()
    h()
    f()
