costumes "dango.svg";

# Blocks
onflag {
    # Motion
    move 10;
    turn_left 45;
    turn_right 45;
    goto_random_position;
    goto_mouse_pointer;
    goto "dango";
    goto "" & foo;
    goto 10, 20;
    glide_to_random_position 1;
    glide_to_mouse_pointer 1;
    glide "dango", 1;
    glide "" & foo, 1;
    glide 10, 20, 1;
    point_in_direction 45;
    point_towards_mouse_pointer;
    point_towards_random_direction;
    point_towards "dango";
    point_towards "" & foo;
    change_x 10;
    set_x 0;
    change_y 10;
    set_y 0;
    if_on_edge_bounce;
    set_rotation_style_left_right;
    set_rotation_style_do_not_rotate;
    set_rotation_style_all_around;
    # Looks
    say "with duration", 2;
    say "without duration";
    think "with duration", 2;
    think "without duration";
    switch_costume "dango";
    switch_costume foo;
    next_costume;
    switch_backdrop "dango";
    switch_backdrop foo;
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
    set_ghost_effect 0;
    clear_graphic_effects;
    show;
    hide;
    goto_front;
    goto_back;
    go_forward 1;
    go_backward 1;
    # Sound
    play_sound_until_done "sound";
    play_sound_until_done foo;
    start_sound "sound";
    start_sound foo;
    stop_all_sounds;
    change_pitch_effect 25;
    change_pan_effect 25;
    set_pitch_effect 25;
    set_pan_effect 25;
    clear_sound_effects;
    change_volume 25;
    set_volume 0;
    # Events
    broadcast "message";
    broadcast_and_wait "message";
    broadcast foo;
    broadcast_and_wait foo;
    # Control
    wait 1;
    wait_until 1 < 2;
    #stop_other_scripts;
    clone "friend";
    clone foo;
    clone;
    # Sensing
    ask "Question?";
    set_draggable;
    set_not_draggable;
    reset_timer;
    # Pen
    erase_all;
    stamp;
    pen_down;
    pen_up;
    set_pen_color "#FF00FF";
    change_pen_hue 25;
    change_pen_saturation 25;
    change_pen_brightness 25;
    change_pen_transparency 25;
    set_pen_hue 0;
    set_pen_saturation 100;
    set_pen_brightness 100;
    set_pen_transparency 0;
    change_pen_size 25;
    set_pen_size 1;
    # Music
    rest 1;
    set_tempo 60;
    change_tempo 25;
    play_note 60, 1;
    # Scratch Addons
    breakpoint;
    warn "message";
    error "message";
    log "message";
}

onflag {
    stop_all;
}

onflag {
    stop_this_script;
}

onclone {
    delete_this_clone;
}

# Reporters
onflag {
    # Motion
    say x_position();
    say y_position();
    say direction();
    say costume_number();
    say costume_name();
    say backdrop_number();
    say backdrop_name();
    say size();
    # Sound
    say volume();
    # Sensing
    say touching_mouse_pointer();
    say touching_edge();
    say touching("friend");
    say touching(foo);
    say touching_color("#FF00FF");
    say distance_to_mouse_pointer();
    say distance_to("friend");
    say distance_to(foo);
    say answer();
    say mouse_down();
    say mouse_x();
    say mouse_y();
    say loudness();
    say timer();
    say current_year();
    say current_month();
    say current_date();
    say current_day_of_week();
    say current_hour();
    say current_minute();
    say current_second();
    say days_since_2000();
    say username();
    # Operators
    say random(1, 6);
    # Music
    foo = tempo();
    # Unary Operators
    say -foo;
    say not (1 < 2);
    say length foo;
    say round foo;
    say abs foo;
    say floor foo;
    say ceil foo;
    say sqrt foo;
    say sin foo;
    say cos foo;
    say tan foo;
    say asin foo;
    say acos foo;
    say atan foo;
    say ln foo;
    say log foo;
    say antiln foo;
    say antilog foo;
}
