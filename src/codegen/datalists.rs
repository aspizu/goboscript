use std::{
    cell::RefCell,
    io::{
        self,
        BufRead,
        BufReader,
    },
    path::{
        Path,
        PathBuf,
    },
    rc::Rc,
};

use crate::{
    diagnostic::DiagnosticKind,
    misc::SmolStr,
    vfs::VFS,
};

fn resolve_path(base_path: Option<&Path>, path: &str, input: &Path) -> PathBuf {
    // Handle absolute paths
    if Path::new(path).is_absolute() {
        return PathBuf::from(path);
    }

    // "./" prefix: relative to sprite directory (or input for stage)
    if let Some(stripped) = path.strip_prefix("./") {
        let base_dir = if let Some(base_path) = base_path {
            base_path.parent().unwrap_or_else(|| Path::new("."))
        } else {
            input.parent().unwrap_or(input)
        };
        return normalize_path(base_dir.join(stripped));
    }

    // For sprites: any path starting with "../" should also be relative to sprite directory
    if path.starts_with("../") && base_path.is_some() {
        let base_dir = base_path
            .unwrap()
            .parent()
            .unwrap_or_else(|| Path::new("."));
        return normalize_path(base_dir.join(path));
    }

    // No prefix: relative to project root (input's parent directory or input itself)
    let project_root = input.parent().unwrap_or(input);
    normalize_path(project_root.join(path))
}

fn normalize_path(path: PathBuf) -> PathBuf {
    // Try to normalize manually first for more predictable behavior in tests
    let mut result = PathBuf::new();

    for component in path.components() {
        match component {
            std::path::Component::ParentDir => {
                // Only pop if we have something to pop and it's not already at root
                if !result.as_os_str().is_empty()
                    && result
                        .components()
                        .last()
                        .map(|c| !matches!(c, std::path::Component::ParentDir))
                        .unwrap_or(false)
                {
                    result.pop();
                } else {
                    // If we can't go up further, keep the parent dir
                    result.push("..");
                }
            }
            std::path::Component::CurDir => {
                // Skip current directory components
            }
            _ => result.push(component),
        }
    }
    result
}

pub fn read_list(
    fs: Rc<RefCell<dyn VFS>>,
    input: &Path,
    path: &SmolStr,
    base_path: Option<&Path>,
) -> Result<Vec<SmolStr>, DiagnosticKind> {
    let (_, ext) = path.rsplit_once('.').unwrap_or_default();
    let resolved_path = resolve_path(base_path, path, input);
    let mut fs = fs.borrow_mut();
    let mut file = fs
        .read_file(&resolved_path)
        .map_err(|err| DiagnosticKind::IOError(err.to_string().into()))?;
    match ext {
        _ => read_list_text(&mut file),
    }
    .map_err(|err| DiagnosticKind::IOError(err.to_string().into()))
}

fn read_list_text(file: &mut Box<dyn io::Read + '_>) -> Result<Vec<SmolStr>, io::Error> {
    let file = BufReader::new(file);
    Ok(file.lines().into_iter().flatten().map(Into::into).collect())
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;

    #[test]
    fn test_resolve_path_relative_to_project_root() {
        let base_path = PathBuf::from("/project/sprites/sprite.gbo");
        let input = PathBuf::from("/project/stage.gbo");

        // Test no prefix: relative to project root
        let result = resolve_path(Some(&base_path), "data.txt", &input);
        assert_eq!(result, PathBuf::from("/project/data.txt"));
    }

    #[test]
    fn test_resolve_path_current_dir() {
        let base_path = PathBuf::from("/project/sprites/sprite.gbo");
        let input = PathBuf::from("/project/stage.gbo");

        // Test ./ prefix: relative to sprite directory
        let result = resolve_path(Some(&base_path), "./data.txt", &input);
        assert_eq!(result, PathBuf::from("/project/sprites/data.txt"));
    }

    #[test]
    fn test_resolve_path_parent_dir() {
        let base_path = PathBuf::from("/project/sprites/sprite.gbo");
        let input = PathBuf::from("/project/stage.gbo");

        // Test ../ prefix: relative to sprite directory, go up one level
        let result = resolve_path(Some(&base_path), "../data.txt", &input);
        assert_eq!(result, PathBuf::from("/project/data.txt"));
    }

    #[test]
    fn test_resolve_path_multiple_parents() {
        let base_path = PathBuf::from("/project/sprites/subdir/sprite.gbo");
        let input = PathBuf::from("/project/stage.gbo");

        // Test multiple ../: relative to sprite directory, go up multiple levels
        let result = resolve_path(Some(&base_path), "../../data.txt", &input);
        assert_eq!(result, PathBuf::from("/project/data.txt"));
    }

    #[test]
    fn test_resolve_path_no_base() {
        let input = PathBuf::from("/project/stage.gbo");

        // Test with no base_path (stage case): relative to project root
        let result = resolve_path(None, "data.txt", &input);
        assert_eq!(result, PathBuf::from("/project/data.txt"));
    }

    #[test]
    fn test_resolve_path_no_base_current_dir() {
        let input = PathBuf::from("/project/stage.gbo");

        // Test ./ prefix with no base_path: relative to stage directory
        let result = resolve_path(None, "./data.txt", &input);
        assert_eq!(result, PathBuf::from("/project/data.txt"));
    }

    #[test]
    fn test_resolve_path_absolute() {
        let base_path = PathBuf::from("/project/sprites/sprite.gbo");
        let input = PathBuf::from("/project/stage.gbo");

        // Test absolute path
        let result = resolve_path(Some(&base_path), "/absolute/data.txt", &input);
        assert_eq!(result, PathBuf::from("/absolute/data.txt"));
    }
}
