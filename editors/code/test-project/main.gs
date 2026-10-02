%include assets/macros

%define SPEED 5
%define GREETING "Hello, world!"
%define SQUARE(x) ((x) * (x))
%define MIN(a, b) if ((a) < (b)) (a) else (b)
%define CORNER -100, 100
%define CENTER 0,                                                                      \
               0
%define UNUSED_MACRO 1
%undef UNUSED_MACRO

%if SHOWCASE
%define TAG "showcase"
%else
%define TAG "plain"
%endif

%if not HIDDEN
%define HIDDEN 0
%endif

# Pre-processor directives do not require a semicolon.

costumes "assets/blank.svg" as "blank";
sounds "assets/pop.wav";

set_x -100;
set_y 0;
set_size 50;
set_volume 100;
point_in_direction 90;
set_rotation_style_left_right;
# hide;

struct Vec {
    x = 0,
    y = 0
}

struct Particle {
    mass = 1,
    charge = -1
}

enum Direction { North, East, South, West }

enum Named { A = "a", B = "b" }

cloud score;

var lives = MAX_LIVES;
var hex = 0xFF;
var binary = 0b1010;
var octal = 0o17;
var big = 1_000_000;
var precise = 1.5e+3;
var tab = "a\tb";
var quote = "say \"hi\"";
var unicode = "\u0041";
var greeting = GREETING;
var direction = Direction.North;
var Vec home = Vec { x: 0, y: 0 };
var Particle spark = Particle { mass: 2 };

list inventory;
list scores = [1, 2, 3];
list blanks = [""; 25];
list Vec trail;
list lore "assets/data.txt";

proc teleport Vec target {
    goto $target.x, $target.y;
}

nowarp proc advance Vec vel {
    change_x $vel.x;
    change_y $vel.y;
}

proc greet name = "world", punctuation = "!" {
    say "Hello, " & $name & $punctuation;
}

func sum(x, y) {
    return $x + $y;
}

func scale(Vec v, k) Vec {
    return Vec { x: $v.x * $k, y: $v.y * $k };
}

proc demo_values {
    say hex;
    say binary;
    say octal;
    say big;
    say precise;
    say tab;
    say quote;
    say unicode;
    say greeting;
    say direction;
    say home.x;
    say spark.mass;
    say spark.charge;
    say length scores;
    say length blanks;
    say length lore;
    say STRINGIFY(MAX_LIVES);
    CONCAT(demo_, lists);
}

proc demo_lists {
    delete inventory;
    add "sword" to inventory;
    insert "shield" at inventory[1];
    inventory[1] = "bow";
    inventory[2] &= "!";
    local found = "bow" in inventory;
    if "sword" not in inventory {
        add "sword" to inventory;
    } elif length inventory > 5 {
        delete inventory[1];
    } else {
        show inventory;
        hide inventory;
    }
    say if (found) "found" else "missing";
}

proc demo_operators {
    local n = round 1.5;
    n += abs -3;
    n -= floor 1.7;
    n *= ceil 1.2;
    n /= sqrt 16;
    n //= 3;
    n %= 7;
    n &= "!";
    n++;
    n--;
    n = SQUARE(4);
    n = MIN(n, 10);
    local label = if (n > 5) "big" else "small";
    say label;
    say sum(1, 2);
}

proc demo_structs {
    local Vec v = Vec { x: 1, y: 2 };
    v.x = 3;
    v.x += 1;
    v.x++;
    v.x--;
    v.y //= 2;
    trail[1] = Vec { x: 1, y: 1 };
    trail[2].x = 9;
    trail[2].y += 1;
    trail[2].x++;
    teleport Vec { x: 0, y: 0 };
    advance Vec { x: SPEED, y: -1 };
    greet "goboscript", "?";
    greet punctuation: "!";
    local Vec scaled = scale(v, 2);
    say scaled.x;
    say Named.A;
    say TAG;
}

onflag {
    goto CORNER;
    goto CENTER;
    score = 0;
    lives = MAX_LIVES;
    demo_values;
    demo_lists;
    demo_operators;
    forever {
        until score >= 100 {
            score += 1;
        }
        if score == 100 and not (lives != 0) {
            say "game over";
        }
    }
}

onkey "space" {
    broadcast "jump";
}

onclick {
    start_sound "pop.wav";
    clone;
    change_size 10;
}

onclone {
    move 10;
    turn_left 15;
    if_on_edge_bounce;
}

onloudness > 30 {
    set_volume 50;
}

on "jump" {
    change_y 10;
    wait 0.5;
}

onbackdrop "city" {
    demo_structs;
}
