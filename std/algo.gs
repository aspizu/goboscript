# Sort `LIST` in ascending order using the insertion sorting algorithm.
%define INSERTION_SORT(LIST)                                                           \
    local i = 2;                                                                       \
    until i > length(LIST) {                                                           \
        local x = LIST[i];                                                             \
        local j = i;                                                                   \
        until j <= 1 or LIST[j - 1] <= x {                                             \
            LIST[j] = LIST[j - 1];                                                     \
            j--;                                                                       \
        }                                                                              \
        LIST[j] = x;                                                                   \
        i++;                                                                           \
    }

# Sort `LIST` in ascending order of the value of `FIELD` using the insertion sorting
# algorithm.
%define INSERTION_SORT_BY_FIELD(LIST,FIELD)                                            \
    local i = 2;                                                                       \
    until i > length(LIST) {                                                           \
        local x = LIST.FIELD[i];                                                       \
        local j = i;                                                                   \
        until j <= 1 or LIST.FIELD[j - 1] <= x {                                       \
            LIST.FIELD[j] = LIST.FIELD[j - 1];                                         \
            j--;                                                                       \
        }                                                                              \
        LIST.FIELD[j] = x;                                                             \
        i++;                                                                           \
    }

# Count the number of elements in `LIST` that satisfy `CMP`, and store the result in
# `STORE`. local `i` is the index of the current element.
%define COUNT(LIST,CMP,STORE)                                                          \
    local STORE = 0;                                                                   \
    local i = 1;                                                                       \
    repeat length(LIST) {                                                              \
        if CMP {                                                                       \
            STORE += 1;                                                                \
        }                                                                              \
        i++;                                                                           \
    }

# Sum the elements in `LIST` that satisfy `CMP`, and store the result in `STORE`.
# local `i` is the index of the current element.
%define SUM(LIST,CMP,STORE)                                                            \
    local STORE = 0;                                                                   \
    local i = 1;                                                                       \
    repeat length(LIST) {                                                              \
        if CMP {                                                                       \
            STORE += LIST[i];                                                          \
        }                                                                              \
        i++;                                                                           \
    }

# Find the largest element in `LIST` that satisfies `CMP`, and store the result in
# `STORE`. local `i` is the index of the current element.
%define AMAX(LIST,CMP,STORE)                                                           \
    local STORE = LIST[1];                                                             \
    local i = 1;                                                                       \
    repeat length(LIST) {                                                              \
        if CMP {                                                                       \
            if LIST[i] > STORE {                                                       \
                STORE = LIST[i];                                                       \
            }                                                                          \
        }                                                                              \
        i++;                                                                           \
    }

# Find the smallest element in `LIST` that satisfies `CMP`, and store the result in
# `STORE`. local `i` is the index of the current element.
%define AMIN(LIST,CMP,STORE)                                                           \
    local STORE = LIST[1];                                                             \
    local i = 1;                                                                       \
    repeat length(LIST) {                                                              \
        if CMP {                                                                       \
            if LIST[i] < STORE {                                                       \
                STORE = LIST[i];                                                       \
            }                                                                          \
        }                                                                              \
        i++;                                                                           \
    }

# Reverse `LIST` in place.
%define REVERSE(LIST)                                                                  \
    local i = 1;                                                                       \
    local j = length(LIST);                                                            \
    repeat length(LIST) / 2 {                                                          \
        local x = LIST[i];                                                             \
        LIST[i] = LIST[j];                                                             \
        LIST[j] = x;                                                                   \
        i++;                                                                           \
        j--;                                                                           \
    }
