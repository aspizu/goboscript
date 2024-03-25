costumes "blank.svg";

onflag {
    say "with duration", 2;
    say "without duration";
    think "with duration", 2;
    think "without duration";
    switch_costume "dango";
    switch_costume costume_number();
    next_costume;
    switch_backdrop "dango";
    switch_backdrop backdrop_number();
    next_backdrop;
    change_size 10;
    set_size 100;
    change_color_effect 25;
    change_fisheye_effect 25;
    change_whirl_effect 25;
    change_pixelate_effect 25;
    change_mosaic_effect 25;
    change_brightness_effect 25;
    change_ghost_effect 25;
    set_color_effect 0;
    set_fisheye_effect 0;
    set_whirl_effect 0;
    set_pixelate_effect 0;
    set_mosaic_effect 0;
    set_brightness_effect 0;
    set_ghost_effect costume_name();
    clear_graphic_effects;
    show;
    hide;
    goto_front;
    goto_back;
    go_forward backdrop_name();
    go_backward size();
}
