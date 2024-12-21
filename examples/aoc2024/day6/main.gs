costumes "blank.svg";

list input = file ```input.txt```;

proc strsetchar string, index, char {
    strsetchar = "";
    local i = 1;
    repeat length($string) {
        if i == $index {
            strsetchar &= $char;
        } else {
            strsetchar &= $string[i];
        }
        i++;
    }
}

proc strfindchar string, char {
    strfindchar = 0;
    local i = 1;
    repeat length($string) {
        if $string[i] == $char {
            strfindchar = i;
        }
        i++;
    }
}

proc step {
    local nx = x + cos(dir);
    local ny = y + sin(dir);
    if input[ny][nx] == "#" {
        dir += 90;
    } else {
        x = nx;
        y = ny;
        strsetchar input[ny], nx, char: "X";
        input[ny] = strsetchar;
    }
}

proc run {
    until x < 0 or y < 0 or x > length(input[1]) or y > length(input) {
        step;
    }
}

proc count_x {
    count_x = 1;
    local i = 1;
    repeat length(input) {
        local j = 1;
        repeat length(input[1]) {
            if input[i][j] == "X" {
                count_x++;
            }
            j++;
        }
        i++;
    }
}

proc find_guard {
    local i = 1;
    repeat length(input) {
        local j = 1;
        repeat length(input[1]) {
            if input[i][j] == "^" {
                x = j;
                y = i;
            }
            j++;
        }
        i++;
    }
}

onflag {
    find_guard;
    dir = 0;
    run;
    count_x;
    say "Count X: " & count_x;
}
