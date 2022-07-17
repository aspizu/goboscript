from parser import parse


with open("examples/demo/main.gs") as fp:
    file = fp.read()
    tree = parse(file)
    print(tree)
