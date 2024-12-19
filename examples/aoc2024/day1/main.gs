# Add a single costume to the sprite.
# The costume is named "blank" as it comes from "blank.svg".
costumes "blank.svg";

# Define two lists:
# 'input' is populated by reading from "day1.txt".
# 'list1' and 'list2' are empty and will be used later.
list input = ``` cat day1.txt ```;
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
        split_once_left &= $string[i];  # Append the current character to the left part.
        i += 1;  # Move to the next character.
    }

    i += 1;  # Skip over the separator.

    # Skip any additional separators (if there are consecutive ones).
    until $string[i] != $sep or i > length($string) {
        i += 1;
    }

    split_once_right = "";  # Initialize the right part of the split.

    # Build the right part of the split until the end of the string is reached.
    until i > length($string) {
        split_once_right &= $string[i];  # Append the current character to the right part.
        i += 1;  # Move to the next character.
    }
}

# Define a procedure to parse the input list into two separate lists: 'list1' and 'list2'.
proc parse_input {
    delete list1;  # Clear 'list1'.
    delete list2;  # Clear 'list2'.
    local i = 1;   # Local index variable.

    # Loop through each item in the input list.
    repeat length(input) {
        split_once input[i], " ";  # Split the current input item at the space character.
        add split_once_left to list1;  # Add the left part to 'list1'.
        add split_once_right to list2; # Add the right part to 'list2'.
        i += 1;  # Move to the next input item.
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
            list1[j] = list1[j - 1];  # Move the larger element up.
            j -= 1;  # Move to the previous position.
        }

        list1[j] = x;  # Insert the current item at the correct position.
        i += 1;  # Move to the next item.
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
            list2[j] = list2[j - 1];  # Move the larger element up.
            j -= 1;  # Move to the previous position.
        }

        list2[j] = x;  # Insert the current item at the correct position.
        i += 1;  # Move to the next item.
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
        i += 1;  # Move to the next item.
    }
}

# Define a procedure to calculate the total distance between corresponding elements in 'list1' and 'list2'.
proc get_total_distance {
    total_distance = 0;  # Initialize the total distance.
    local i = 1;  # Start from the first item.

    # Loop through each pair of elements in 'list1' and 'list2'.
    repeat length(list1) {
        total_distance += abs(list1[i] - list2[i]);  # Add the absolute difference to the total.
        i += 1;  # Move to the next pair.
    }
}

# Define a procedure to calculate a similarity score between 'list1' and 'list2'.
proc get_similarity_score {
    similarity_score = 0;  # Initialize the similarity score.
    local i = 1;  # Start from the first item.

    # Loop through each item in 'list1'.
    repeat length(list1) {
        count_list2 list1[i];  # Count how many times the current item appears in 'list2'.
        similarity_score += list1[i] * count_list2;  # Add the weighted value to the score.
        i += 1;  # Move to the next item.
    }
}

# Main program starts when the green flag is clicked.
onflag {
    parse_input;  # Split input into 'list1' and 'list2'.
    sort_list1;   # Sort 'list1' in ascending order.
    sort_list2;   # Sort 'list2' in ascending order.
    get_total_distance;  # Calculate the total distance between elements of 'list1' and 'list2'.
    get_similarity_score;  # Calculate the similarity score between 'list1' and 'list2'.

    # Display the total distance and similarity score.
    say "Total Distance: " & total_distance & ", Similarity Score: " & similarity_score;
}
