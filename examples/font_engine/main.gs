costumes "blank.svg" as "@ascii/_";

list azf_font = ```
TIME=$(stat font.svg -c %Y)
[ $TIME -eq $(cat font.svg.time) ] && cat font.svg.cached && exit
echo $TIME > font.svg.time
python3 convert_font.py font.svg > font.svg.cached
cat font.svg.cached
```;
list self = ``` cat main.gs ```;

struct azf {
    sx,
    sy,
    dx,
    dy,
    x,
    y,
    x2,
    start_x,
    start_y,
    ptr,
    cmd
}

onflag {
    azf azf = azf {
        sx = 1,        # scale-x
        sy = 0-1,      # scale-y (should be negative)
        dx = 2,        # distance between characters
        dy = 3,        # distance between lines
        x = 0-230,     # x-position to draw at (will be mutated)
        y = 170,       # y-position to draw at (will be mutated)
        x2 = 230,      # x-position to clip text at
        start_x = 0,   # --.
        start_y = 0,   #   |- (internal state)
        ptr = 0,       #   |
        cmd = 0        # --`
    };
    forever {
        azf.x = 0-230;
        azf.y = 170;
        render;
        if key_pressed("up arrow") {
            scroll += 1;
        }
        if key_pressed("down arrow") {
            scroll -= 1;
            if scroll < 0 {
                scroll = 0;
            }
        }
    }
}

proc render {
    erase_all;
    i = scroll;
    repeat 340 // (azf_font[2] + azf.dy)  {
        azf_draw_string self[i], 0-230;
        azf.y += azf.sy * (azf_font[2] + azf.dy);
        i += 1;
        if i > length(self) {
            stop_this_script;
        }
    }
}

proc azf_draw_string string, x {
    azf.x = $x;
    local i = 1;
    repeat length($string) {
        azf_draw_char $string[i];
        azf.x += azf.sx * (azf_font[1] + azf.dx);
        if azf.x >= azf.x2 {
            azf.x = $x;
            azf.y += azf.sy * (azf_font[2] + azf.dy);
        }
        i += 1;
    }
}

proc azf_draw_char char {
    switch_costume "_" & $char;
    azf.ptr = azf_font[2 + costume_number()];
    until azf.ptr == 0 {
        azf_step;
    }
}

proc azf_step {
    if azf_font[azf.ptr] == "M" 
    or azf_font[azf.ptr] == "L"
    or azf_font[azf.ptr] == "H"
    or azf_font[azf.ptr] == "V"
    or azf_font[azf.ptr] == "Z"
    or azf_font[azf.ptr] == "#" {
        azf.cmd = azf_font[azf.ptr];
        azf.ptr += 1;
    }
    if azf.cmd == "M" {
        goto
            azf.x + azf.sx * azf_font[azf.ptr],
            azf.y + azf.sy * azf_font[azf.ptr + 1];
        azf.start_x = x_position();
        azf.start_y = y_position();
        azf.ptr += 2;
    }
    elif azf.cmd == "L" {
        pen_down;
        goto
            azf.x + azf.sx * azf_font[azf.ptr],
            azf.y + azf.sy * azf_font[azf.ptr + 1];
        pen_up;
        azf.ptr += 2;
    }
    elif azf.cmd == "H" {
        pen_down;
        set_x azf.x + azf.sx * azf_font[azf.ptr];
        pen_up;
        azf.ptr += 1;
    }
    elif azf.cmd == "V" {
        pen_down;
        set_y azf.y + azf.sy * azf_font[azf.ptr];
        pen_up;
        azf.ptr += 1;
    }
    elif azf.cmd == "Z" {
        pen_down;
        goto azf.start_x, azf.start_y;
        pen_up;
    }
    elif azf.cmd == "#" {
        azf.ptr = 0;
    }
}
