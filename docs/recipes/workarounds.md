# Workarounds

 - **[Switch to previous costume](#switch-to-previous-costume)**
 - **[Switch to last costume](#switch-to-last-costume)**
 - **[Sense if sprite is shown or hidden](#sense-if-sprite-is-shown-or-hidden)**
 - **[Set pen color to a RGB (or RGBA) value](#set-pen-color-to-a-rgb-or-rgba-value)**
 - **[Point towards X, Y](#point-towards-x-y)**
 - **[Rotate around X, Y](#rotate-around-x-y)**

## Switch to previous costume

```goboscript
%define previous_costume switch_costume "previous_costume"
```

## Switch to last costume

```goboscript
%define last_costume switch_costume 0
```

## Sense if sprite is shown or hidden

```goboscript
proc _show { visible = true; show; }
proc _hide { visible = false; show; }
%define show _show
%define hide _hide
```

You can extend this recipe to implement sensing of other sprite properties, which
Scratch does not provide a reporter block for. For example, you can implement
sensing of the ghost effect of a sprite.

## Set pen color to a RGB (or RGBA) value

Use the `RGB()` and `RGBA()` macros provided in the standard library header `std/math`.

```goboscript
%include std/math
set_pen_color RGB(red, green, blue);
```

## Point towards X, Y

```goboscript
proc point_towards_xy x, y {
    point_in_direction atan($x - x_position() / $y - y_position());
    if $y < y_position() {
        turn_right 180;
    }
}
```

## Rotate around X, Y

```goboscript
%include std/math

proc rotate_around x, y, degrees {
    local dir = direction();
    local dist = DIST($x, $y, x_position(), y_position());
    point_towards_xy $x, $y;
    goto
        $x + dist * sin(direction() + 180 + $degrees),
        $y + dist * cos(direction() + 180 + $degrees);
    point_in_direction dir;
    turn_right $degrees;
}
```
