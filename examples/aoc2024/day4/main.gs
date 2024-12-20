costumes "blank.svg";

list input = file ```input.txt```;

proc count {
    count = 0;
    local i = 1;
    repeat length(input) {
        local j = 1;
        repeat length(input[0]) {
            count += (
                    input[i][j    ] == "X"
                and input[i][j + 1] == "M"
                and input[i][j + 2] == "A"
                and input[i][j + 3] == "S"
            )
            + (
                    input[i][j    ] == "S"
                and input[i][j + 1] == "A"
                and input[i][j + 2] == "M"
                and input[i][j + 3] == "X"
            )
            + (
                    input[i    ][j] == "X"
                and input[i + 1][j] == "M"
                and input[i + 2][j] == "A"
                and input[i + 3][j] == "S"
            )
            + (
                    input[i    ][j] == "S"
                and input[i + 1][j] == "A"
                and input[i + 2][j] == "M"
                and input[i + 3][j] == "X"
            )
            + (
                    input[i    ][j    ] == "X"
                and input[i + 1][j + 1] == "M"
                and input[i + 2][j + 2] == "A"
                and input[i + 3][j + 3] == "S"
            )
            + (
                    input[i    ][j    ] == "S"
                and input[i + 1][j + 1] == "A"
                and input[i + 2][j + 2] == "M"
                and input[i + 3][j + 3] == "X"
            )
            + (
                    input[i    ][j + 3] == "X"
                and input[i + 1][j + 2] == "M"
                and input[i + 2][j + 1] == "A"
                and input[i + 3][j    ] == "S"
            )
            + (
                    input[i    ][j + 3] == "S"
                and input[i + 1][j + 2] == "A"
                and input[i + 2][j + 1] == "M"
                and input[i + 3][j    ] == "X"
            );
            j++;
        }
        i++;
    }
}

onflag {
    # delete input;
    # add "XMAS" to input;
    # add "...." to input;
    # add "...." to input;
    # add "...." to input;

    # add "SAMX" to input;
    # add "...." to input;
    # add "...." to input;
    # add "...." to input;
    
    # add "X..." to input;
    # add "M..." to input;
    # add "A..." to input;
    # add "S..." to input;

    # add "S..." to input;
    # add "A..." to input;
    # add "M..." to input;
    # add "X..." to input;

    # add "X..." to input;
    # add ".M.." to input;
    # add "..A." to input;
    # add "...S" to input;

    # add "S..." to input;
    # add ".A.." to input;
    # add "..M." to input;
    # add "...X" to input;

    # add "...S" to input;
    # add "..A." to input;
    # add ".M.." to input;
    # add "X..." to input;

    # add "...X" to input;
    # add "..M." to input;
    # add ".A.." to input;
    # add "S..." to input;

    count;
    say "Count: " & count;
}
