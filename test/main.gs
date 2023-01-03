costumes "blank.svg";

macro dist x1, y1, x2, y2 -> sqrt((!x2-!x1)*(!x2-!x1)+(!y2-!y1)*(!y2-!y1));

onflag {
    say !dist(1, 2, 3, 4);
}
