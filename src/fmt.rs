use std::{fs, path::PathBuf};

pub enum FmtError {}

pub fn format_file(path: PathBuf) -> Result<(), FmtError> {
    let mut src = fs::read(&path).unwrap();
    format_buffer_inplace(&mut src)?;
    fs::write(path, src).unwrap();
    Ok(())
}

fn format_buffer_inplace(src: &mut Vec<u8>) -> Result<(), FmtError> {
    let max_line_length = 88;
    let mut i = 0;
    let mut line_begin = true;
    while i < src.len() {
        if line_begin && src[i] == b'%' {
            loop {
                let begin = i;
                i += src[i..].split(|c| *c == b'\n').next().unwrap().len();
                if src[i - 1] != b'\\' {
                    i += 1;
                    break;
                }
                i -= 1;
                let mut slash = i;
                let diff = slash - begin;
                if diff >= (max_line_length - 1) {
                    for _ in 0..(diff - (max_line_length - 1)) {
                        src.remove(slash - 1);
                        slash -= 1;
                        i -= 1;
                    }
                } else {
                    for _ in 0..((max_line_length - 1) - diff) {
                        src.insert(slash, b' ');
                        i += 1;
                    }
                }
                i += 2;
            }
        } else {
            line_begin = src[i] == b'\n';
            i += 1;
        }
    }
    Ok(())
}
