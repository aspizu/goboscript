# Sensing Reporters

### distance to [mouse-pointer]

```goboscript
dist = distance_to_mouse_pointer();
```

```scratchblocks
(distance to [mouse-pointer v])
```

### distance to ()

```goboscript
dist = distance_to("sprite_name");
```

```scratchblocks
(distance to (sprite_name v))
```

### touching [mouse-pointer]?

```goboscript
is_touching = touching_mouse_pointer();
```

```scratchblocks
<touching [mouse-pointer v]?>
```

### touching [edge]?

```goboscript
is_touching = touching_edge();
```

```scratchblocks
<touching [edge v]?>
```

### touching ()?

```goboscript
is_touching = touching("sprite_name");
```

```scratchblocks
<touching (sprite_name v)?>
```

### key () pressed?

```goboscript
is_pressed = key_pressed("up arrow");
```

```scratchblocks
<key (up arrow v) pressed?>
```

### mouse down?

```goboscript
is_mouse_down = mouse_down();
```

```scratchblocks
<mouse down?>
```

### mouse x

```goboscript
mx = mouse_x();
```

```scratchblocks
(mouse x)
```

### mouse y

```goboscript
my = mouse_y();
```

```scratchblocks
(mouse y)
```

### loudness

```goboscript
sound_level = loudness();
```

```scratchblocks
(loudness)
```

### timer

```goboscript
time_elapsed = timer();
```

```scratchblocks
(timer)
```

### current [year]

```goboscript
year = current_year();
```

```scratchblocks
(current [year v])
```

### current [month]

```goboscript
month = current_month();
```

```scratchblocks
(current [month v])
```

### current [date]

```goboscript
date = current_date();
```

```scratchblocks
(current [date v])
```

### current [day of week]

```goboscript
day = current_day_of_week();
```

```scratchblocks
(current [day of week v])
```

### current [hour]

```goboscript
hour = current_hour();
```

```scratchblocks
(current [hour v])
```

### current [minute]

```goboscript
minute = current_minute();
```

```scratchblocks
(current [minute v])
```

### current [second]

```goboscript
second = current_second();
```

```scratchblocks
(current [second v])
```

### days since 2000

```goboscript
days = days_since_2000();
```

```scratchblocks
(days since 2000)
```

### username

```goboscript
user = username();
```

```scratchblocks
(username)
```

### online?

```goboscript
is_online = online();
```

```scratchblocks
<online?>
```

### touching color ()?

```goboscript
is_touching_color = touching_color(0xff0000);
```

```scratchblocks
<touching color (#ff0000)?>
```

### color () is touching ()?

```goboscript
is_color_touching = color_is_touching_color(0xff0000, 0x00ff00);
```

```scratchblocks
<color (#ff0000) is touching (#00ff00)?>
```

### answer

```goboscript
user_answer = answer();
```

```scratchblocks
(answer)
```

### difficult to name thing of thing block

```goboscript
backdrop_number = "Stage"."backdrop #";
```

```scratchblocks
([backdrop #] of (Stage v))
```
