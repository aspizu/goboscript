costumes "blank.svg";

onflag {
    move 10;
    turn_left 45;
    turn_right 45;
    goto_random_position;
    goto_mouse_pointer;
    goto "dango";
    goto x_position();
    goto 10, 20;
    glide_to_random_position 1;
    glide_to_mouse_pointer 1;
    glide "dango", 1;
    glide y_position(), 1;
    glide 10, 20, 1;
    point_in_direction 45;
    point_towards_mouse_pointer;
    point_towards_random_direction;
    point_towards "dango";
    point_towards direction();
    change_x 10;
    set_x 0;
    change_y 10;
    set_y 0;
    if_on_edge_bounce;
    set_rotation_style_left_right;
    set_rotation_style_do_not_rotate;
    set_rotation_style_all_around;
}
