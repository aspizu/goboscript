costumes "blank.svg";

macro A v -> !v+1;

onflag {
    v = 10;
    say !A(v);
}
