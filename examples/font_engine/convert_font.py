import re
import sys
import xml.etree.ElementTree as ET

namespaces = {
    "svg": "http://www.w3.org/2000/svg",
    "inkscape": "http://www.inkscape.org/namespaces/inkscape",
}

CHARSET = "".join(map(chr, range(ord(" "), ord("~") + 1)))

tree = ET.parse(sys.argv[1])
root = tree.getroot()

paths = root.findall(".//svg:path", namespaces)

width = int(root.get("width"))
height = int(root.get("height"))
print(width)
print(height)


def modulate(d: list[str]) -> list[str]:
    cmd = None
    i = 0
    while i < len(d):
        if d[i].isalpha():
            cmd = d[i]
            i += 1
        if cmd.upper() in "ML":
            if cmd.isupper():
                d[i] = int(float(d[i])) % (width * 2)
            i += 2
        elif cmd.upper() == "H":
            if cmd.isupper():
                d[i] = int(float(d[i])) % (width * 2)
            i += 1
        elif cmd.upper() == "V":
            i += 1
    return d


characters = {}
for path in paths:
    d = path.get("d")
    label = path.get("{%s}label" % namespaces["inkscape"])
    characters[label] = modulate(re.split(r"[,\s]+", d))

i = 0
for ch in CHARSET:
    d = characters.get(ch, [])
    d.append("#")
    print(3 + len(CHARSET) + i)
    i += len(d)
for ch in CHARSET:
    d: list[str] = characters.get(ch, ["#"])
    for x in d:
        if str(x).islower() and x not in "Zz":
            x = "d" + x
        print(x)

# M x y : go to x, y
# X x   : set x to x
# H dx : change x by dx
# Y y   : set y to y
# V dy : change y by dy
# D     : pen down
# U     : pen up
