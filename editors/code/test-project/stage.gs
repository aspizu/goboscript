%include assets/macros

costumes "assets/blank.svg";

enum Mode { Idle, Running }

cloud score;

var mode = Mode.Idle;

list leaderboard = [0, 0, 0];

onflag {
    score = 0;
    mode = Mode.Running;
    broadcast "reset";
}

on "reset" {
    delete leaderboard;
    add 0 to leaderboard;
    add 0 to leaderboard;
    add 0 to leaderboard;
}

onbackdrop "city" {
    switch_backdrop "blank";
}

ontimer > 300 {
    score += 1;
}
