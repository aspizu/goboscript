costumes "blank.svg";

onflag {
    foo = 1;
    wait 1;
    repeat foo {
        wait_until 1 < 2;
    }
    if 1 < 2 {
        clone foo;
    }
    elif 1 >= 2 {
        stop_this_script;
    }
    else {
        clone "friend";
    }
    forever {
        clone;
        stop_all;
    }
}
