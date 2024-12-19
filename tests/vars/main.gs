costumes "blank.svg";

proc main {
    var = 1;
    local local_var = 2;
    global_var = 3;
    say var + local_var + global_var;
    var += local_var;
    var -= local_var;
    var *= local_var;
    var /= local_var;
    var //= local_var;
    var %= local_var;
    var &= local_var;
}

onflag {
    main;
}
