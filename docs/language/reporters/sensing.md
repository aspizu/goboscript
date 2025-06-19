# Sensing Reporters

### distance to [mouse-pointer]

```goboscript
dist = distance_to_mouse_pointer();
```

```_ {.scratchblocks}
(distance to [mouse-pointer v])
```

### distance to ()

```goboscript
dist = distance_to("sprite_name");
```

```_ {.scratchblocks}
(distance to (sprite_name v))
```

### touching [mouse-pointer]?

```goboscript
is_touching = touching_mouse_pointer();
```

```_ {.scratchblocks}
<touching [mouse-pointer v]?>
```

### touching [edge]?

```goboscript
is_touching = touching_edge();
```

```_ {.scratchblocks}
<touching [edge v]?>
```

### touching ()?

```goboscript
is_touching = touching("sprite_name");
```

```_ {.scratchblocks}
<touching (sprite_name v)?>
```

### key () pressed?

```goboscript
is_pressed = key_pressed("up arrow");
```

```_ {.scratchblocks}
<key (up arrow v) pressed?>
```

### mouse down?

```goboscript
is_mouse_down = mouse_down();
```

```_ {.scratchblocks}
<mouse down?>
```

### mouse x

```goboscript
mx = mouse_x();
```

```_ {.scratchblocks}
(mouse x)
```

### mouse y

```goboscript
my = mouse_y();
```

```_ {.scratchblocks}
(mouse y)
```

### loudness

```goboscript
sound_level = loudness();
```

```_ {.scratchblocks}
(loudness)
```

### timer

```goboscript
time_elapsed = timer();
```

```_ {.scratchblocks}
(timer)
```

### current [year]

```goboscript
year = current_year();
```

```_ {.scratchblocks}
(current [year v])
```

### current [month]

```goboscript
month = current_month();
```

```_ {.scratchblocks}
(current [month v])
```

### current [date]

```goboscript
date = current_date();
```

```_ {.scratchblocks}
(current [date v])
```

### current [day of week]

```goboscript
day = current_day_of_week();
```

```_ {.scratchblocks}
(current [day of week v])
```

### current [hour]

```goboscript
hour = current_hour();
```

```_ {.scratchblocks}
(current [hour v])
```

### current [minute]

```goboscript
minute = current_minute();
```

```_ {.scratchblocks}
(current [minute v])
```

### current [second]

```goboscript
second = current_second();
```

```_ {.scratchblocks}
(current [second v])
```

### days since 2000

```goboscript
days = days_since_2000();
```

```_ {.scratchblocks}
(days since 2000)
```

### username

```goboscript
user = username();
```

```_ {.scratchblocks}
(username)
```

### touching color ()?

```goboscript
is_touching_color = touching_color(0xff0000);
```

```_ {.scratchblocks}
<touching color (#ff0000)?>
```

### color () is touching ()?

```goboscript
is_color_touching = color_is_touching_color(0xff0000, 0x00ff00);
```

```_ {.scratchblocks}
<color (#ff0000) is touching (#00ff00)?>
```

### answer

```goboscript
user_answer = answer();
```

```_ {.scratchblocks}
(answer)
```
