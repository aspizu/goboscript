costumes "std:blank.svg";

def foo {
    local_list   = [];
    $global_list = [];
    .dot_list    = [];

    say local_list   : length();
    say $global_list : length();
    say .dot_list    : length();
}
