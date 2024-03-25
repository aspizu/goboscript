costumes "blank.svg";

onflag {
    play_sound_until_done "sound";
    play_sound_until_done volume();
    start_sound "sound";
    start_sound volume();
    stop_all_sounds;
    change_pitch_effect 25;
    change_pan_effect 25;
    set_pitch_effect 25;
    set_pan_effect 25;
    clear_sound_effects;
    change_volume 25;
    set_volume 0;
}
