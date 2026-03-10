fn main() {
    lalrpop::process_src().unwrap();

    if let Ok(output) = std::process::Command::new("git")
        .args(["rev-parse", "--short", "HEAD"])
        .output()
    {
        let git_hash = std::str::from_utf8(&output.stdout).unwrap().trim();
        println!("cargo:rustc-env=GIT_HASH={git_hash}");
    }
}
