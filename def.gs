def function_name arg1, arg2, arg3 {
  say @arg1;
  say @arg2;
  say @arg3;
  .x = 0; // this provides function_name.x is a local variable
  say (1 + 2) * 3;
}

onflag {
  say "Hello World";
  goto 100, 200;
  goto
    100,
    200;
  foo
    100, 200,
    300;
  
  if condition {
    say 1;
  }
  elif condition {
    say 2;
  }
  else {
    say 3;
  }
  
  forever {
    do_some_stuff;
  }
  
  repeat 10 {
    do_some_stuff_10_times;
  }
  
  until condition {
    do_some_stuff
  }
}

onflag {
  x  = 0;  // this statement provides x is a local variable
  $x = 0; // this statement provides $x is a global variable
  x  =
    round(100000);

  // lists
  x = [];           // this provides x is a local list and delete all of x
  x = [1, 2, 3, 4]; // delete all of x and fill x with constants
  x =
    [1, 2, 3, 4, 5];
  x = [
    1,
    2,
    3,
    4
  ];
  x[1] = 10;
  say x[1];
  x:add 10; // add 10 to x
  say x:length;
}
