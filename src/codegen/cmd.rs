use std::{
    cell::RefCell,
    env::consts::OS,
    io::{
        self,
        BufRead,
        BufReader,
        Write,
    },
    path::Path,
    process::{
        Command,
        Stdio,
    },
    rc::Rc,
};

use crate::{
    ast::Cmd,
    vfs::VFS,
};

pub fn cmd_to_list(
    fs: Rc<RefCell<dyn VFS>>,
    cmd: &Cmd,
    input: &Path,
) -> Result<Vec<String>, (Option<io::Error>, Option<Vec<u8>>)> {
    if cmd
        .program
        .as_ref()
        .is_some_and(|program| &*program.name == "file")
    {
        let mut fs = fs.borrow_mut();
        let file = fs
            .read_file(&input.join(&*cmd.cmd))
            .map_err(|err| (Some(err), None))?;
        let reader = BufReader::new(file);
        let lines: Vec<String> = reader.lines().map_while(Result::ok).collect();
        return Ok(lines);
    }
    let mut child = if let Some(program) = &cmd.program {
        Command::new(&*program.name)
            .current_dir(input)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|err| (Some(err), None))?
    } else if OS == "windows" {
        unimplemented!()
    } else {
        Command::new("bash")
            .args(["-e", "-u", "-o", "pipefail"])
            .current_dir(input)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .unwrap()
    };
    let mut stdin = child.stdin.take().unwrap();
    stdin.write_all(cmd.cmd.as_bytes()).unwrap();
    stdin.flush().unwrap();
    drop(stdin);
    let output = child.wait_with_output().unwrap();
    if output.status.success() {
        let mut lines: Vec<String> = output.stdout.lines().map_while(Result::ok).collect();
        if lines.last().is_some_and(|line| line.is_empty()) {
            lines.pop();
        }
        Ok(lines)
    } else {
        Err((None, Some(output.stderr)))
    }
}
