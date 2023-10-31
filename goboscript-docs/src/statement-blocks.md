# Statement Blocks
```goboscript
move steps;
```
```blocks
move (steps:: custom) steps
```
```goboscript
turnright degrees;
```
```blocks
turn cw (degrees:: custom) degrees
```
```goboscript
turnleft degrees;
```
```blocks
turn ccw (degrees:: custom) degrees
```
```goboscript
gotosprite to;
```
```blocks
go to (to:: custom)
```
```goboscript
gotomouse;
```
```blocks
go to (mouse-pointer v)
```
```goboscript
gotorandom;
```
```blocks
go to (random position v)
```
```goboscript
goto x, y;
```
```blocks
go to x: (x:: custom) y: (y:: custom)
```
```goboscript
glidetosprite secs, to;
```
```blocks
glide (secs:: custom) secs to (to:: custom)
```
```goboscript
glidetomouse secs;
```
```blocks
glide (secs:: custom) secs to (mouse-pointer v)
```
```goboscript
glidetorandom secs;
```
```blocks
glide (secs:: custom) secs to (random position v)
```
```goboscript
glide secs, x, y;
```
```blocks
glide (secs:: custom) secs to x: (x:: custom) y: (y:: custom)
```
```goboscript
point direction;
```
```blocks
point in direction (direction:: custom)
```
```goboscript
pointtowards towards;
```
```blocks
point towards (towards:: custom)
```
```goboscript
pointmouse;
```
```blocks
point towards (mouse-pointer v)
```
```goboscript
changex dx;
```
```blocks
change x by (dx:: custom)
```
```goboscript
changey dy;
```
```blocks
change y by (dy:: custom)
```
```goboscript
setx x;
```
```blocks
set x to (x:: custom)
```
```goboscript
sety y;
```
```blocks
set y to (y:: custom)
```
```goboscript
ifonedgebounce;
```
```blocks
if on edge, bounce
```
```goboscript
rotateflip;
```
```blocks
set rotation style [left-right v]
```
```goboscript
rotateany;
```
```blocks
set rotation style [all around v]
```
```goboscript
rotatenone;
```
```blocks
set rotation style [don't rotate v]
```
```goboscript
say message;
```
```blocks
say (message:: custom)
```
```goboscript
sayfor secs, message;
```
```blocks
say (secs:: custom) for (message:: custom) seconds
```
```goboscript
think message;
```
```blocks
think (message:: custom)
```
```goboscript
thinkfor secs, message;
```
```blocks
think (secs:: custom) for (message:: custom) seconds
```
```goboscript
switchcostume costume;
```
```blocks
switch costume to (costume:: custom)
```
```goboscript
nextcostume;
```
```blocks
next costume
```
```goboscript
switchbackdrop backdrop;
```
```blocks
switch backdrop to (backdrop:: custom)
```
```goboscript
nextbackdrop;
```
```blocks
next backdrop
```
```goboscript
changesize change;
```
```blocks
change size by (change:: custom)
```
```goboscript
setsize size;
```
```blocks
set size to (size:: custom) %
```
```goboscript
cleargraphiceffects;
```
```blocks
clear graphic effects
```
```goboscript
hide;
```
```blocks
hide
```
```goboscript
show;
```
```blocks
show
```
```goboscript
gotofront;
```
```blocks
go to [front v] layer
```
```goboscript
gotoback;
```
```blocks
go to [back v] layer
```
```goboscript
goforward num;
```
```blocks
go [forward v] (num:: custom) layers
```
```goboscript
gobackward num;
```
```blocks
 go [backward v] (num:: custom) layers
```
```goboscript
setcoloreffect value;
```
```blocks
set [color v] effect to (value:: custom)
```
```goboscript
changecoloreffect change;
```
```blocks
change [color v] effect by (change:: custom)
```
```goboscript
setfisheyeeffect value;
```
```blocks
set [fisheye v] effect to (value:: custom)
```
```goboscript
changefisheyeeffect change;
```
```blocks
change [fisheye v] effect by (change:: custom)
```
```goboscript
setwhirleffect value;
```
```blocks
set [whirl v] effect to (value:: custom)
```
```goboscript
changewhirleffect change;
```
```blocks
change [whirl v] effect by (change:: custom)
```
```goboscript
setpixelateeffect value;
```
```blocks
set [pixelate v] effect to (value:: custom)
```
```goboscript
changepixelateeffect change;
```
```blocks
change [pixelate v] effect by (change:: custom)
```
```goboscript
setmosaiceffect value;
```
```blocks
set [mosaic v] effect to (value:: custom)
```
```goboscript
changemosaiceffect change;
```
```blocks
change [mosaic v] effect by (change:: custom)
```
```goboscript
setbrightnesseffect value;
```
```blocks
set [brightness v] effect to (value:: custom)
```
```goboscript
changebrightnesseffect change;
```
```blocks
change [brightness v] effect by (change:: custom)
```
```goboscript
setghosteffect value;
```
```blocks
set [ghost v] effect to (value:: custom)
```
```goboscript
changeghosteffect change;
```
```blocks
change [ghost v] effect by (change:: custom)
```
```goboscript
playsound sound_menu;
```
```blocks
play sound (sound_menu:: custom) until done
```
```goboscript
startsound sound_menu;
```
```blocks
start sound (sound_menu:: custom)
```
```goboscript
stopallsounds;
```
```blocks
stop all sounds
```
```goboscript
clearsoundeffects;
```
```blocks
clear sound effects
```
```goboscript
changevolume volume;
```
```blocks
change volume by (volume:: custom)
```
```goboscript
setvolume volume;
```
```blocks
set volume to (volume:: custom)%
```
```goboscript
setpitcheffect value;
```
```blocks
set [pitch v] effect to (value:: custom)
```
```goboscript
changepitcheffect change;
```
```blocks
change [pitch v] effect by (change:: custom)
```
```goboscript
setpaneffect value;
```
```blocks
set [pan left/right v] effect to (value:: custom)
```
```goboscript
changepaneffect change;
```
```blocks
change [pan left/right v] effect by (change:: custom)
```
```goboscript
broadcast broadcast_input;
```
```blocks
broadcast (broadcast_input:: custom)
```
```goboscript
broadcastandwait broadcast_input;
```
```blocks
broadcast (broadcast_input:: custom) and wait
```
```goboscript
wait duration;
```
```blocks
wait (duration:: custom) seconds
```
```goboscript
waituntil condition;
```
```blocks
wait until <condition:: custom>
```
```goboscript
cloneself;
```
```blocks
create clone of (myself v)
```
```goboscript
clone clone_option;
```
```blocks
create clone of (clone_option:: custom)
```
```goboscript
deleteclone;
```
```blocks
delete this clone
```
```goboscript
stopall;
```
```blocks
stop [all v]
```
```goboscript
return;
```
```blocks
stop [this script v]
```
```goboscript
stopother;
```
```blocks
stop [other scripts in sprite v]
```
```goboscript
ask question;
```
```blocks
ask (question:: custom) and wait
```
```goboscript
draggable;
```
```blocks
set drag mode [draggable v]
```
```goboscript
notdraggable;
```
```blocks
set drag mode [not draggable v]
```
```goboscript
resettimer;
```
```blocks
reset timer
```
```goboscript
clear;
```
```blocks
erase all
```
```goboscript
stamp;
```
```blocks
stamp
```
```goboscript
pendown;
```
```blocks
pen down
```
```goboscript
penup;
```
```blocks
pen up
```
```goboscript
setpencolor color;
```
```blocks
set pen color to (color:: custom)
```
```goboscript
setpensize size;
```
```blocks
set pen size to (size:: custom)
```
```goboscript
changepensize size;
```
```blocks
change pen size by (size:: custom)
```
```goboscript
setpenhue value;
```
```blocks
set pen [color v] to (value:: custom)
```
```goboscript
changepenhue value;
```
```blocks
change pen [color v] by (value:: custom)
```
```goboscript
setpensaturation value;
```
```blocks
set pen [saturation v] to (value:: custom)
```
```goboscript
changepensaturation value;
```
```blocks
change pen [saturation v] by (value:: custom)
```
```goboscript
setpenbrightness value;
```
```blocks
set pen [brightness v] to (value:: custom)
```
```goboscript
changepenbrightness value;
```
```blocks
change pen [brightness v] by (value:: custom)
```
```goboscript
setpentransparency value;
```
```blocks
set pen [transparency v] to (value:: custom)
```
```goboscript
changepentransparency value;
```
```blocks
change pen [transparency v] by (value:: custom)
```
This file was auto-generated.
