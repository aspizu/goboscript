costumes "blank.svg";

onflag {
    split "One,Two,Three,Four", ",";
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
