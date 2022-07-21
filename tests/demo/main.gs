costumes "assets/blank.svg";


def function arg1, arg2, arg3 {
    say @arg1;
    say @arg2;
    say @arg3;
}

onflag {
    variable = 0;
    function 1 + 2, 1 * 2, variable ++ 2;
}
