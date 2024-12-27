costumes "blank.svg";

list input = file ```input.txt```;
list split;
list reports;

proc split string, sep {
    delete split;
    local part = "";
    local i = 1;
    repeat length($string) {
        if $string[i] == $sep {
            add part to split;
            part = "";
        }
        else {
            part &= $string[i];
        }
        i++;
    }
    if length(part) > 0 {
        add part to split;
    }
}

proc parse_input {
    delete reports;
    local i = 1;
    repeat length(input) {
        split input[i], sep: " ";
        add length(split) to reports;
        local j = 1;
        repeat length(split) {
            add split[j] to reports;
            j++;
        }
        i++;
    }
}

proc is_report_safe idx, skip_idx {
    is_report_safe = false;
    local i = $idx + 1;
    local dir = 0;
    if $skip_idx == 1 {
        i++;
    }
    until i - $idx + 1 > reports[$idx] - ($skip_idx == reports[$idx]) {
        if i + 1 == $idx + $skip_idx {
            local difference = reports[i + 2] - reports[i];
            i++;
        }
        else {
            local difference = reports[i + 1] - reports[i];
        }
        local distance = abs(difference);
        local newdir = difference / distance;
        if distance < 1 or distance > 3 or (dir != 0 and dir != newdir) {
            stop_this_script;
        }
        dir = newdir;
        i++;
    }
    is_report_safe = true;
}

proc is_report_safe_with_problem_dampener idx {
    local i = 1;
    repeat reports[$idx] {
        is_report_safe $idx, i;
        if is_report_safe == true {
            stop_this_script;
        }
        i++;
    }
}

proc count_safe_reports {
    count_safe_reports = 0;
    local i = 1;
    until i > length(reports) {
        is_report_safe i, skip_idx: 0;
        if is_report_safe == true {
            count_safe_reports++;
        }
        i += reports[i] + 1;
    }
}

proc count_safe_reports_with_problem_dampener {
    count_safe_reports_with_problem_dampener = 0;
    local i = 1;
    until i > length(reports) {
        is_report_safe i, skip_idx: 0;
        if is_report_safe == false {
            is_report_safe_with_problem_dampener i;
        }
        count_safe_reports_with_problem_dampener += is_report_safe;
        i += reports[i] + 1;
    }
}

onflag {
    parse_input;
    count_safe_reports;
    count_safe_reports_with_problem_dampener;
    say
        "Safe reports: "
        & count_safe_reports
        & "\nSafe reports with problem dampener: "
        & count_safe_reports_with_problem_dampener;
}
