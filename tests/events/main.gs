costumes "blank.svg";

onflag {
    foo = 1;
    broadcast "message";
    broadcast_and_wait "message";
    broadcast foo;
    broadcast_and_wait foo;
}
