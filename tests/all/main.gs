costumes "blank.svg";

use "modules/module.gs";

def function arg1, arg2, arg3 {
    say @arg1;
}

onflag {
    function "string", 3.14159, true;
    var = 0;
    var += 0;
    var -= 0;
    var *= 0;
    var /= 0;
    var %= 0;
    var ++= 0;
    var.show;
    $gvar = 0;
    $gvar += 0;
    $gvar -= 0;
    $gvar *= 0;
    $gvar /= 0;
    $gvar %= 0;
    $gvar ++= 0;
    $gvar.show;
    lst = [];
    lst[100] = 100;
    $glst = [];
    $glst[100] = 100;
    if 1 = 1 {
        say true;
    }
    if 1 = 0 {
        say false;
    } elseif 1 = 1 {
        say false;
    } elseif 1 = 1 {
        say false;
    } else {
        say false;
    }
    repeat 100 {
        say false;
    }
    until false = true {
        say true;
    }
}

onkey "space" {
    say round(10);
    say lst.index(100);
    say $glst.index(100);
    say lst[100];
    say $glst[100];
    say !1 = 1;
    say 1 < 1 & 1 > 1;
    say 1 = 1 | 1 = 1;
    say "hello " ++ "world";
    say -200;
    say 1 + 1;
    say 2 - 1;
    say 3 * 4;
    say 3 / 3;
    say 1 % 3;
    say false;
    say $gvar;
    say var;
}
