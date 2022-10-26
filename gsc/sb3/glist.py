class gList:
    def __init__(self, name: str) -> None:
        self.name = name
        self.values = []

    def __rich_repr__(self):
        yield self.name
