def rectangle x, y, w, h {
  goto @x, @y;
  pendown;
  goto @x + @w - 1, @y;
  goto @x + @w - 1, @y + @h - 1;
  goto @x, @y + @h - 1;
  goto @x, @y;
  penup;
}

onflag {
  rectangle 0, 0, 100, 100;
  if true = true {
    say "if part";
  } else {
    say "else part";
  }
  x = variable;
}
