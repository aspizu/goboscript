def render(f, schema_name):
    with open(f"assets/{schema_name}_blockdefs.txt", "r") as fp:
        f.write(schema_name + " = {\n")
        for i in fp.readlines():
            x = [j.strip() for j in i.split()]
            f.write(f"    {x[0]!r}: ({x[1]!r}, {tuple(x[2].split(','))!r}),\n")
        f.write("}\n\n")


with open("proscript/blockdefs.py", "w") as f:
    render(f, "reporter")
    render(f, "statement")
