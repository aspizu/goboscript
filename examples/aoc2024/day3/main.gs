costumes "blank.svg";

list input = file ```input.txt```;

proc join_input {
    join_input = "";
    local i = 1;
    repeat length(input) {
        join_input &= input[i];
        i++;
    }
}

proc run text, enable_do_donts {
    run = 0;
    local do = true;
    local i = 1;
    until i > length($text) {
        if $text[i] == "d" {
            i++;
            if $text[i] == "o" {
                i++;
                if $text[i] == "(" {
                    i++;
                    if $text[i] == ")" {
                        i++;
                        do = true;
                    }
                }
                elif $text[i] == "n" {
                    i++;
                    if $text[i] == "'" {
                        i++;
                        if $text[i] == "t" {
                            i++;
                            if $text[i] == "(" {
                                i++;
                                if $text[i] == ")" {
                                    i++;
                                    do = false;
                                }
                            }
                        }
                    }
                }
            }
        }
        elif $text[i] == "m" {
            i++;
            if $text[i] == "u" {
                i++;
                if $text[i] == "l" {
                    i++;
                    if $text[i] == "(" {
                        i++;
                        local x = "";
                        until $text[i] * 1 != $text[i] {
                            x &= $text[i];
                            i++;
                        }
                        if x != "" and $text[i] == "," {
                            i++;
                            local y = "";
                            until $text[i] * 1 != $text[i] {
                                y &= $text[i];
                                i++;
                            }
                            if y != "" and $text[i] == ")" {
                                i++;
                                if $enable_do_donts == false or do == true {
                                    run += x * y;
                                }
                            }
                        }
                    }
                }
            }
        }
        else {
            i++;
        }
    }
}

onflag {
    join_input;
    run join_input, false;
    without_do_donts = run;
    run join_input, true;
    say
        "Without do() and don't() enabled: "
        & without_do_donts
        & "\nWith do() and don't() enabled: "
        & run;
}
