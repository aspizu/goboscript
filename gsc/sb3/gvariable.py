class gVariable:
    def __init__(self, name: str) -> None:
        self.name = name
        self.value = "0"

    def __rich_repr__(self):
        yield self.name
