costumes
    "/path/to/costume.png",
    "/path/to/costume.png",
    "/path/to/costume.png",
    "/path/to/costume.png",
    "/path/to/costume.png";

sounds
    "/path/to/sound.wav",
    "/path/to/sound.wav",
    "/path/to/sound.wav",
    "/path/to/sound.wav",
    "/path/to/sound.wav";

use "/path/to/module.gs";

def function 
    eeeeeeeeeeeeeee,
    eeeeeeeeeeeeeee,
    eeeeeeeeeeeeeee,
    eeeeeeeeeeeeeee,
    eeeeeeeeeeeeeee
{
    say false;
}

onflag {
    varset = 0;
    varchange += 0;
    varsub -= 0;
    varmul *= 0;
    vardiv /= 0;
    varmod %= 0;
    varjoin ++= 0;
    ofstatement.show 1, 2, 3;
    $gvarset = 0;
    $gvarchange += 0;
    $gvarsub -= 0;
    $gvarmul *= 0;
    $gvardiv /= 0;
    $gvarmod %= 0;
    $gvarjoin ++= 0;
    $gofstatement.show 1, 2, 3;
    lstset = [];
    lstchange[100] = 100;
    $glstset = [];
    $glstchange[100] = 100;
    if 1 = 1 {
        say true;
    }
    if 1 = 0 {
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
    say list.index(100);
    say $list.index(100);
    say list[100];
    say $list[100];
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
    say @argument;
    say $GLOBAL_VARIABLE;
    say local_variable;
}