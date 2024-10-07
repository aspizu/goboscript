# Configuration

goboscript uses a `goboscript.toml` configuration file to store project-specific
configuration.

## Turbowarp options

goboscript can generate a turbowarp configuration comment inside the Stage.
[https://docs.turbowarp.org/advanced-settings](https://docs.turbowarp.org/advanced-settings)

### Frame Rate

```toml
frame_rate = 60
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
no_miscellaneous_limits = true
```

### No Sprite Fencing

```toml
no_sprite_fencing = true
```

### Frame Interpolation

```toml
frame_interpolation = true
```

### High Quality Pen

```toml
high_quality_pen = true
```

### Stage Width & Height

```toml
stage_width = 640
stage_height = 480
```
