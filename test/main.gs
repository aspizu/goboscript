costumes "blank.svg";

macro b -> 0;
macro a -> 1 + !b();

onflag {
  say !a();
}
