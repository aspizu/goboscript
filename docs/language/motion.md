# Motion

## Move n steps

```goboscript
move 10;
```

↺
## Turn ↻ n degrees

```goboscript
turn_right 15;
```

## Turn ↺ n degrees

```goboscript
turn_left 15;
```

## Go to

### Go to random position

```goboscript
goto_random_position;
```

### Go to mouse-pointer

```goboscript
goto_mouse_pointer;
```

### Go to sprite

This will make your sprite go to the position of another sprite named "gobo".

```goboscript
goto "gobo";
```

### Go to x y

```goboscript
goto 100, 140;
```

## Glide n seconds to 

### Glide n seconds to random position

```goboscript
glide_to_random_position 2;
```

### Glide n seconds to mouse-pointer

```goboscript
glide_to_mouse_pointer 2;
```

### Glide n seconds to sprite

This will make your sprite glide to the position of another sprite named "gobo".

```goboscript
glide "dango", 2;
```

### Glide n seconds to x y

The first two values provide the x and y coordinates. The third value is the amount of seconds it will take to glide there.

```goboscript
glide 100, 140, 2;
```

## Point in direction n

```goboscript
point_in_direction 90;
```

## Point towards

### Point towards mouse-pointer

```goboscript
point_towards_mouse_pointer;
```

### Point towards random direction

```goboscript
 point_towards_random_direction;
 ```

 ### Point towards sprite

 ```goboscript
 point_towards "gobo";
 ```

 ## Change x by n

  ```goboscript
change_x 10;
 ```

## Set x to n

  ```goboscript
set_x 0;
 ```

 ## Change y by n

  ```goboscript
change_y 10;
 ```

## Set y to n

  ```goboscript
set_y 0;
 ```

## If on edge, bounce

```goboscript
if_on_edge_bounce;
```

## Set rotation style

### Set rotation style left-right

```goboscript
set_rotation_style_left_right;
```

### Set rotation style don't rotate

```goboscript
set_rotation_style_do_not_rotate;
```

### Set rotation style all around

```goboscript
set_rotation_style_all_around;
```

## Reporter blocks

### x position

```goboscript
x_position()
```

### y position

```goboscript
y_position()
```

### direction

```goboscript
direction()
```