use std::{
    fs,
    path::PathBuf,
};

pub enum FmtError {
    AnyhowError(anyhow::Error),
}

impl<T> From<T> for FmtError
where T: Into<anyhow::Error>
{
    fn from(value: T) -> Self {
        Self::AnyhowError(value.into())
    }
}

pub fn format_file(path: PathBuf) -> Result<(), FmtError> {
    let mut src = fs::read(&path).unwrap();
    format_buffer_inplace(&mut src)?;
    fs::write(path, src).unwrap();
    Ok(())
}

fn format_buffer_inplace(src: &mut Vec<u8>) -> Result<(), FmtError> {
    let max_line_length = 88;
    let mut lines: Vec<Vec<u8>> = Vec::new();
    let mut current_line = Vec::new();
    
    // Split buffer into lines
    for &byte in src.iter() {
        if byte == b'\n' {
            lines.push(current_line);
            current_line = Vec::new();
        } else {
            current_line.push(byte);
        }
    }
    if !current_line.is_empty() {
        lines.push(current_line);
    }
    
    let mut i = 0;
    while i < lines.len() {
        let line = &lines[i];
        
        // Only process multi-line directives (start with '%' and end with '\')
        if line.starts_with(b"%") && line.ends_with(b"\\") {
            // Find the complete multi-line directive
            let mut directive_lines = vec![i];
            let mut j = i + 1;
            
            // Keep adding lines while they end with backslash
            while j < lines.len() {
                let current = &lines[j];
                if current.ends_with(b"\\") {
                    directive_lines.push(j);
                    j += 1;
                } else {
                    break;
                }
            }
            
            // Format each line in the directive
            for &line_idx in &directive_lines {
                let line_mut = &mut lines[line_idx];
                if line_mut.ends_with(b"\\") {
                    // Trim trailing spaces before backslash
                    let mut content_end = line_mut.len().saturating_sub(2);
                    while content_end > 0 && line_mut[content_end] == b' ' {
                        content_end -= 1;
                    }
                    line_mut.truncate(content_end + 2);

                    let current_length = line_mut.len() - 1; // without backslash
                    let target_column = max_line_length - 1;
                    if current_length < target_column && current_length > 0 {
                        let spaces_needed = target_column - current_length;
                        for _ in 0..spaces_needed {
                            line_mut.insert(line_mut.len() - 1, b' ');
                        }
                    }
                }
            }
            // Skip past this directive block
            i = j;
        } else {
            i += 1;
        }
    }
    
    // Rebuild the buffer
    src.clear();
    for (idx, line) in lines.iter().enumerate() {
        src.extend_from_slice(line);
        if idx < lines.len() - 1 {
            src.push(b'\n');
        }
    }

    Ok(())
}
