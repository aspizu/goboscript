costumes "blank.svg";

onflag {
    ask "What is your name?";
    foo = answer();
    say touching_mouse_pointer();
    say touching_edge();
    say touching("foo");
    say touching(foo);
    say touching_color("#ff0000");
    say color_is_touching_color("#ff0000", "#00ff00");
    say key_pressed("space");
    say key_pressed(foo);
    say mouse_down();
    say mouse_x();
    say mouse_y();
    set_drag_mode_draggable;
    set_drag_mode_not_draggable;
    say loudness();
    say timer();
    reset_timer;
    say current_year();
    say current_month();
    say current_date();
    say current_day_of_week();
    say current_hour();
    say current_minute();
    say current_second();
    say days_since_2000();
    say username();
    say direction of foo;
}
