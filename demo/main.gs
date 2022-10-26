costumes "demo/assets/blank.svg";

nowarp def foo_bar {
  repeat 10 {
    changex 10;
    changey $var;
  }
}

def foo a, b, c {
  say @a;
  say @b;
  say @c;
}

onflag {
  $var = "global variable";
  $var += 10;
  $var *= 10;
  var = 10;
  foo 1 + var, $var - var, 3;
  foo_bar;
  say "Hello, \"World\" \\\n";
  if true = true {
    say 1;
  }
  /* elif true = false {
    say 2;
  } else {
    say 3;
  } */
  repeat 10 {
    move 10;
  }
  forever {
    say "aspizu was here";
  }
}
