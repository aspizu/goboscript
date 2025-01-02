# Enums

Enum variants will automatically be assigned a value starting from 0.

```goboscript
enum Direction {
    North, #    0
    East,  #    1
    South, #    2
    West   #    3
}
```

An explicit value can be given to an enum variant.

```goboscript
enum Direction {
    North = "North",
    East = "East",
    South = "South",
    West = "West"
}
```

Explicit values and implicit values can be mixed.

```goboscript
enum Direction {
    A = "A",
    B, #       0
    C = "C",
    D, #       1
    E = 2,
    F, #       3

}

## Get enum value

```goboscript
say Direction.North;
```
