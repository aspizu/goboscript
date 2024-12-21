costumes "blank.svg";

list input = file ```input.txt```;
list list1;
list list2;

proc split_once string, sep {
    local i = 1;
    split_once_left = "";
    until $string[i] == $sep or i > length($string) {
        split_once_left &= $string[i];
        i++;
    }
    i++;
    until $string[i] != $sep or i > length($string) {
        i++;
    }
    split_once_right = "";
    until i > length($string) {
        split_once_right &= $string[i];
        i++;
    }
}

proc parse_input {
    delete list1;
    delete list2;
    local i = 1;
    repeat length(input) {
        split_once input[i], " ";
        add split_once_left to list1;
        add split_once_right to list2;
        i++;
    }
}

proc sort_list1 {
    local i = 2;
    until i > length(list1) {
        local x = list1[i];
        local j = i;
        until j <= 1 or list1[j - 1] <= x {
            list1[j] = list1[j - 1];
            j--;
        }
        list1[j] = x;
        i++;
    }
}

proc sort_list2 {
    local i = 2;
    until i > length(list2) {
        local x = list2[i];
        local j = i;
        until j <= 1 or list2[j - 1] <= x {
            list2[j] = list2[j - 1];
            j--;
        }
        list2[j] = x;
        i++;
    }
}

proc count_list2 value {
    count_list2 = 0;
    local i = 1;
    repeat length(list2) {
        if list2[i] == $value {
            count_list2 += 1;
        }
        i++;
    }
}

proc get_total_distance {
    total_distance = 0;
    local i = 1;
    repeat length(list1) {
        total_distance += abs(list1[i] - list2[i]);
        i++;
    }
}

proc get_similarity_score {
    similarity_score = 0;
    local i = 1;
    repeat length(list1) {
        count_list2 list1[i];
        similarity_score += list1[i] * count_list2;
        i++;
    }
}

onflag {
    parse_input;
    sort_list1;
    sort_list2;
    get_total_distance;
    get_similarity_score;
    say "Total Distance: " & total_distance & "\nSimilarity Score: " & similarity_score;
}
