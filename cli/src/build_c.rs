use std::{path::PathBuf, process::Command};

pub struct Settings {
    pub path: PathBuf,

    pub out_path: PathBuf,
}

pub fn generate_build_command(opts: &Settings) -> Command {
    let mut clang = Command::new("clang");
    clang.args(&["-c", "-emit-llvm"]);
    clang.arg(opts.path.as_os_str());

    clang.arg("-o");
    clang.arg(opts.out_path.as_os_str());

    clang
}
