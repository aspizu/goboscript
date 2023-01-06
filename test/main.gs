costumes "blank.svg";

onflag {
    split "One,Two,Three,Four", ","; (* This is a comment *)
    x = 0;
    x += costume();
}

def split string, sep {
    splitted[];
    key = "";
    i = 1;
    until i > length($string) {
        if letter(i, $string) = $sep {
            splitted.add key;
            key = "";
        } else {
            key = key & letter(i, $string);    
        }
        i = i + 1;
    }
    splitted.add key;
}
