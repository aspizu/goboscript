costumes "blank.svg";

onflag {
    lhs = 1;
    rhs = 2;
    say lhs + rhs;
    say lhs - rhs;
    say lhs * rhs;
    say lhs / rhs;
    say lhs // rhs;
    say random(lhs, rhs);
    say lhs > rhs;
    say lhs >= rhs;
    say lhs < rhs;
    say lhs <= rhs;
    say lhs == rhs;
    say lhs != rhs;
    say key_pressed("up arrow") and key_pressed("down arrow");
    say key_pressed("up arrow") or key_pressed("down arrow");
    say not key_pressed("up arrow");
    say lhs & rhs;
    say lhs[rhs];
    say length rhs;
    say rhs in lhs;
    say lhs % rhs;
    say round lhs;
    say abs lhs;
    say floor lhs;
    say ceil lhs;
    say sqrt lhs;
    say sin lhs;
    say cos lhs;
    say tan lhs;
    say asin lhs;
    say acos lhs;
    say atan lhs;
    say ln lhs;
    say log lhs;
    say antiln lhs;
    say antilog lhs;
}
