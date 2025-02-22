# Sprites

A sprite is a `.gs` file in the root of the project directory, all other `.gs` files
(which are inside sub-directories of the project directory, or outside of the project
directory, are not sprites.) are header files.

## Properties

Sprite properties can be set using statements similar to blocks which set those
properties at runtime.

!!! note
    These statements are top-level, outside any declaration block.

### Sprite default X position

```goboscript
set_x 100;
```

### Sprite default Y position

```goboscript
set_y 100;
```

### Sprite default size

```goboscript
set_size 100;
```

### Sprite default direction

```goboscript
point_in_direction 100;
```

### Sprite default volume

```goboscript
set_volume 100;
```

### Set Sprite visibility to hidden

```goboscript
hide;
```

### Set Sprite rotation style to **left-right**

```goboscript
set_rotation_style_left_right;
```

### Set Sprite rotation style to **all around**

```goboscript
set_rotation_style_all_around;
```

### Set Sprite rotation style to **don't rotate**

```goboscript
set_rotation_style_do_not_rotate;
```
