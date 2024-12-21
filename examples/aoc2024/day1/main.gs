# Add a single costume to the sprite.
# The costume is named "blank" as it comes from "blank.svg".
costumes "blank.svg";

list input = file ```input.txt```;
list list1;
list list2;

# Define a procedure to split a string at the first occurrence of a separator.
# Arguments:
#   string - The string to be split.
#   sep - The separator to split the string at.
proc split_once string, sep {
    local i = 1;  # Local variable to track the current index in the string.
    split_once_left = "";  # Initialize the left part of the split.

    # Build the left part of the split until the separator is found or the end is reached.
    until $string[i] == $sep or i > length($string) {
        split_once_left &= $string[i];
        i++;
    }
    i++;
    until $string[i] != $sep or i > length($string) {
        i++;
    }

    split_once_right = "";  # Initialize the right part of the split.

    # Build the right part of the split until the end of the string is reached.
    until i > length($string) {
        split_once_right &= $string[i];
        i++;
    }
}

# Define a procedure to parse the input list into two separate lists: 'list1' and 'list2'.
proc parse_input {
    delete list1;  # Clear 'list1'.
    delete list2;  # Clear 'list2'.
    local i = 1;   # Local index variable.

    # Loop through each item in the input list.
    repeat length(input) {
        split_once input[i], " ";
        add split_once_left to list1;
        add split_once_right to list2;
        i++;
    }
}

# Define a procedure to sort 'list1' using insertion sort.
proc sort_list1 {
    local i = 2;  # Start sorting from the second item.
    until i > length(list1) {
        local x = list1[i];  # Store the current item.
        local j = i;  # Start comparing backward from the current position.

        # Shift larger elements one position to the right.
        until j <= 1 or list1[j - 1] <= x {
            list1[j] = list1[j - 1];
            j--;
        }
        list1[j] = x;
        i++;
    }
}

# Define a procedure to sort 'list2' using insertion sort (same logic as sort_list1).
proc sort_list2 {
    local i = 2;  # Start sorting from the second item.
    until i > length(list2) {
        local x = list2[i];  # Store the current item.
        local j = i;  # Start comparing backward from the current position.

        # Shift larger elements one position to the right.
        until j <= 1 or list2[j - 1] <= x {
            list2[j] = list2[j - 1];
            j--;
        }
        list2[j] = x;
        i++;
    }
}

# Define a procedure to count how many times a specific value appears in 'list2'.
# Argument:
#   value - The value to count.
proc count_list2 value {
    count_list2 = 0;  # Initialize the counter.
    local i = 1;  # Start from the first item.

    # Loop through each item in 'list2'.
    repeat length(list2) {
        if list2[i] == $value {  # If the current item matches the value:
            count_list2 += 1;  # Increment the counter.
        }
        i++;
    }
}

# Define a procedure to calculate the total distance between corresponding elements in 'list1' and 'list2'.
proc get_total_distance {
    total_distance = 0;  # Initialize the total distance.
    local i = 1;  # Start from the first item.

    # Loop through each pair of elements in 'list1' and 'list2'.
    repeat length(list1) {
        total_distance += abs(list1[i] - list2[i]);
        i++;
    }
}

# Define a procedure to calculate a similarity score between 'list1' and 'list2'.
proc get_similarity_score {
    similarity_score = 0;  # Initialize the similarity score.
    local i = 1;  # Start from the first item.

    # Loop through each item in 'list1'.
    repeat length(list1) {
        count_list2 list1[i];
        similarity_score += list1[i] * count_list2;
        i++;
    }
}

# Main program starts when the green flag is clicked.
onflag {
    parse_input;
    sort_list1;
    sort_list2;
    get_total_distance;
    get_similarity_score;
    say "Total Distance: " & total_distance & "\nSimilarity Score: " & similarity_score;
}
