costumes "_blank.svg";

def proc1 arg1, arg2, arg3 {
  say @arg1;
  say @arg2;
  say @arg3;
}

onflag {
  say add(100, sub(200, 400));
  proc1 1, 2, 3;
}
