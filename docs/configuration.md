# Configuration

goboscript uses a `goboscript.toml` configuration file to store project-specific
configuration.

## Standard Library Version

If not provided, the latest version is picked (Updates fetched daily)

```toml
std = "2.1.0" # default is unset
```

## Sprites layer order

Specify the order in which sprites are layered, by default the order is undefined.

```toml
layers = ["sprite_name_1", "sprite_name_2"]
```

## Turbowarp options

goboscript can generate a turbowarp configuration comment inside the Stage.
[https://docs.turbowarp.org/advanced-settings](https://docs.turbowarp.org/advanced-settings)

### Frame Rate

```toml
frame_rate = 60 # default is 30
```

### Max Clones

#### Default

```toml
max_clones = 300
```

#### Infinite Clones

```toml
max_clones = inf
```

### No Miscellaneous Limits

```toml
no_miscellaneous_limits = true # default is false
```

### No Sprite Fencing

```toml
no_sprite_fencing = true # default is false
```

### Frame Interpolation

```toml
frame_interpolation = true # default is false
```

### High Quality Pen

```toml
high_quality_pen = true # default is false
```

### Stage Width & Height

```toml
stage_width = 640 # default is 480
stage_height = 480 # default is 360
```
