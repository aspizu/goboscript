costumes "blank.svg";

macro stdSplit! string, char, list, v0, v1 {
  v0! = 1;
  v1! = "";
  repeat length(string!) {
    if letter(v0!, string!) = char! {
      list!.add v1!;
      v1! = "";
    } else {
      v1! &= letter(v0!, string!);
    }
    v0! += 1;
  }
}

onflag {
  output[];
  v0 = 0;
  v1 = 0;
  stdSplit! "one,two,three", ",", output, v0, v1;
}
