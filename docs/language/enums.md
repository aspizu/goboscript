# Enums

Enums are a way to define a set of incrementing integers.

```goboscript
enum Direction {
    North, #    0
    East,  #    1
    South, #    2
    West   #    3
}
```

## Get enum value

```goboscript
say Direction.North;
```

## Use enums as a way to emulate structs

```goboscript
enum Person {
    Name,
    Age,
    Gender
}

delete persons;
say "persons[index].Name = " & persons[index + Person.Name];
say "persons[index].Age = " & persons[index + Person.Age];
say "persons[index].Gender = " & persons[index + Person.Gender];
```
