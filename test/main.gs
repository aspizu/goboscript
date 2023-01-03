costumes "blank.svg";

def line x1, y1, x2, y2 {
    goto $x1, $y1;
    pendown;
    goto $x2, $y2;
    penup;
}

nowarp def branch x, y, dir, iter {
    if $iter < MAXITER {
        line $x, $y, $x + sin($dir)*LEN, $y + cos($dir)*LEN;
        branch $x + sin($dir)*LEN, $y + cos($dir)*LEN, $dir+TURN, $iter+1;
        branch $x + sin($dir)*LEN, $y + cos($dir)*LEN, $dir-TURN, $iter+1;
    }
}

onflag {
    clear;
    LEN     = 20;
    MAXITER = 8;
    TURN    = 34;
    branch 0, 0, 0, 0;
}
