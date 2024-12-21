costumes "blank.svg";

list input = file ```input.txt```;

proc count_xmas {
    count_xmas = 0;
    local i = 1;
    repeat length(input) {
        local j = 1;
        repeat length(input[1]) {
            count_xmas += (
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

proc count_x_mas {
    count_x_mas = 0;
    local i = 1;
    repeat length(input) {
        local j = 1;
        repeat length(input[1]) {
            local a = input[i    ][j    ];
            local b = input[i    ][j + 2];
            local c = input[i + 2][j    ];
            local d = input[i + 2][j + 2];
            count_x_mas += input[i + 1][j + 1] == "A" and ((
                    a == "M"
                and b == "S"
                and c == "M"
                and d == "S"
            ) or (
                    a == "S"
                and b == "M"
                and c == "S"
                and d == "M"
            ) or (
                    a == "M"
                and b == "M"
                and c == "S"
                and d == "S"
            ) or (
                    a == "S"
                and b == "S"
                and c == "M"
                and d == "M"
            ));
            j++;
        }
        i++;
    }
}

onflag {
    count_xmas;
    count_x_mas;
    say "Count XMAS: " & count_xmas & "\nCount X-MAS: " & count_x_mas;
}
