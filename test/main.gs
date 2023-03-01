costumes "blank.svg";

def foo {
    local bar = 100;
    foo2;
}

def foo2 {
   local bar = 200;
}

onflag {
    list[];
    say list.length;
}
