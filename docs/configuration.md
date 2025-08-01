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

## Bitmap Resolution

Controls the resolution handling for bitmap images (PNG, BMP) in your project.

```toml
bitmap_resolution = 2 # default is 1
```

### How Bitmap Resolution Works

Scratch internally uses high-resolution bitmaps where each screen pixel corresponds to 4 image pixels (2x scale factor). This setting determines how goboscript handles this scaling:

- **`bitmap_resolution = 1`** (default): Your images are stored as-is, and Scratch automatically scales them up by 2x when the project loads. Use normal-sized images (e.g., 480x360 for full stage backdrops).

- **`bitmap_resolution = 2`**: Your images are treated as high-resolution and displayed at half their pixel size. You must provide double-sized images (e.g., 960x720 for full stage backdrops) to achieve the same visual size.

!!! NOTE
    This setting only affects bitmap formats (PNG, BMP). Vector formats (SVG) are unaffected.

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
