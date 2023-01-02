costumes "blank.svg";

onflag {
    broadcast "my_event";
}

on "my_event" {
    say "my event recieved";
    if not 2<3 {
        ...
    }
}
