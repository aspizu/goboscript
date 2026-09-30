const KEYS: &[&str] = &[
    "a",
    "b",
    "c",
    "d",
    "e",
    "f",
    "g",
    "h",
    "i",
    "j",
    "k",
    "l",
    "m",
    "n",
    "o",
    "p",
    "q",
    "r",
    "s",
    "t",
    "u",
    "v",
    "w",
    "x",
    "y",
    "z",
    "0",
    "1",
    "2",
    "3",
    "4",
    "5",
    "6",
    "7",
    "8",
    "9",
    "space",
    "up arrow",
    "down arrow",
    "right arrow",
    "left arrow",
    "enter",
    "any",
    "backspace",
    "delete",
    "shift",
    "caps lock",
    "scroll lock",
    "control",
    "escape",
    "insert",
    "home",
    "end",
    "page up",
    "page down",
];

const KEY_CHARS: &str = "-,.`=[]\\;'/!@#$%^&*()_+{}|:\"?<>~";

pub fn all_keys<'a>() -> impl Iterator<Item = &'a str> {
    KEYS.iter()
        .copied()
        .chain(KEY_CHARS.split("").filter(|key| !key.is_empty()))
}

pub fn is_key(s: &str) -> bool {
    KEYS.contains(&s) || KEY_CHARS.split("").any(|key| !key.is_empty() && key == s)
}
