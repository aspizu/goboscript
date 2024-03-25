const KEYS: &[&str] = &[
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
    KEYS.iter().copied().chain(KEY_CHARS.split(""))
}

pub fn is_key(s: &str) -> bool {
    KEYS.contains(&s) || KEY_CHARS.split("").any(|c| c == s)
}
