costumes "blank.svg";

list input = file ```input.txt```;
struct Rule { left, right }
list Rule rules;
list pages;
list strsplitchar;

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

proc strsplitchar string, char {
    delete strsplitchar;
    local part = "";
    local i = 1;
    repeat length($string) {
        if $string[i] == $char {
            add part to strsplitchar;
            part = "";
        } else {
            part &= $string[i];
        }
        i++;
    }
    if length(part) > 0 {
        add part to strsplitchar;
    }
}

proc parse_input {
    delete rules;
    delete pages;
    local i = 1;
    repeat length(input) {
        strfindchar input[i], "|";
        if strfindchar > 0 {
            strsplitchar input[i], "|";
            add Rule { left = strsplitchar[1], right = strsplitchar[2] } to rules;
        } else {
            strfindchar input[i], ",";
            if strfindchar > 0 {
                strsplitchar input[i], ",";
                add length(strsplitchar) to pages;
                local j = 1;
                repeat length(strsplitchar) {
                    add strsplitchar[j] to pages;
                    j++;
                }
            }
        }
        i++;
    }
}

proc page_find_idx page_ptr, value {
    page_find_idx = 0;
    local i = $page_ptr + 1;
    repeat length(pages[$page_ptr]) {
        if pages[$page_ptr] == $value {
            page_find_idx = i - $page_ptr;
            stop_this_script;
        }
        i++;
    }
}

proc rule_in_page Rule rule, page_ptr {
    rule_in_page = true;
    page_find_idx $page_ptr, $rule.left;
    local left_idx = page_find_idx;
    page_find_idx $page_ptr, $rule.right;
    local right_idx = page_find_idx;
    if left_idx > 0 and right_idx > 0 {
        rule_in_page = left_idx < right_idx;
    }
}

proc rules_in_page page_ptr {
    local i = 1;
    repeat length(rules) {
        rule_in_page rules[i], $page_ptr;
        if rule_in_page == false {
            stop_this_script;
        }
        i++;
    }
}

proc middle_number page_ptr {
    middle_number = pages[$page_ptr + 1 + pages[$page_ptr] // 2];
}

proc main {
    parse_input;
    local sum = 0;
    local i = 1;
    until i > length(pages) {
        rules_in_page i;
        if rule_in_page == true {
            middle_number i;
            sum += middle_number;
        }
        i += pages[i];
    }
    say "Result: " & sum;
}

onflag {
    main;
}
