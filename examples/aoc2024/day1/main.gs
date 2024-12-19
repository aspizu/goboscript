costumes "blank.svg";

list input = ``` cat day1.txt ```;
list list1;
list list2;

proc split_once string, sep {
    local i = 1;
    split_once_left = "";
    until $string[i] == $sep or i > length($string) {
        split_once_left &= $string[i];
        i += 1;
    }
    i += 1;
    until $string[i] != $sep or i > length($string) {
        i += 1;
    }
    split_once_right = "";
    until i > length($string) {
        split_once_right &= $string[i];
        i += 1;
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
        i += 1;
    }
}

proc sort_list1 {
    local i = 2;
    until i > length(list1) {
        local x = list1[i];
        local j = i;
        until j <= 1 or list1[j - 1] <= x {
            list1[j] = list1[j - 1];
            j -= 1;
        }
        list1[j] = x;
        i += 1;
    }
}

proc sort_list2 {
    local i = 2;
    until i > length(list2) {
        local x = list2[i];
        local j = i;
        until j <= 1 or list2[j - 1] <= x {
            list2[j] = list2[j - 1];
            j -= 1;
        }
        list2[j] = x;
        i += 1;
    }
}

proc count_list2 value {
    count_list2 = 0;
    local i = 1;
    repeat length(list2) {
        if list2[i] == $value {
            count_list2 += 1;
        }
        i += 1;
    }
}

proc get_total_distance {
    total_distance = 0;
    local i = 1;
    repeat length(list1) {
        total_distance += abs(list1[i] - list2[i]);
        i += 1;
    }
}

proc get_similarity_score {
    similarity_score = 0;
    local i = 1;
    repeat length(list1) {
        count_list2 list1[i];
        similarity_score += list1[i] * count_list2;
        i += 1;
    }
}

onflag {
    parse_input;
    sort_list1;
    sort_list2;
    get_total_distance;
    get_similarity_score;
    say "Total Distance: " & total_distance & ", Similarity Score: " & similarity_score;
}
