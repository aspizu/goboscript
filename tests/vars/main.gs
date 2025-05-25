costumes "blank.svg";

proc main {
    v = 1;
    local local_v = 2;
    global_v = 3;
    say v + local_v + global_v;
    v += local_v;
    v -= local_v;
    v *= local_v;
    v /= local_v;
    v //= local_v;
    v %= local_v;
    v &= local_v;
}

onflag {
    main;
}
