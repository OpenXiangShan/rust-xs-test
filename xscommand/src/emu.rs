/// Emu XSCommand Implementation
/// 

use super::{XSCommand, DefaultErr};
use std::{
    fs::File,
    os::unix::io::{FromRawFd, IntoRawFd},
    process::{Command, Stdio}
};
use xscommand_macros::XSCommand;

#[derive(XSCommand)]
pub struct Emu<'a> {
    pub exe: Command,
    args: Vec<&'a str>,
    work_dir: Option<&'a str>,
}

#[test]
fn test_emu_help() {
    use std::fs;
    use std::path::Path;
    let workload = Path::new("emu_help_test");
    if !workload.exists() {
        // workload not exists, create
        fs::create_dir(workload).unwrap();
    }
    let stdout_file = workload.join("emu_help_stdout.txt");
    let stderr_file = workload.join("emu_help_stderr.txt");
    let mut emu = Emu::set_exe("emu");
    let args = vec!["--help"];
    emu.set_args(args).unwrap();
    emu.set_workdir(workload.to_str()).unwrap();
    match emu.excute(stdout_file.to_str(), stderr_file.to_str()) {
        Ok(exit_code) => {
            assert_eq!(exit_code, 0);
        },
        Err(errtype) => panic!("ErrType: {:?}", errtype),
    }
    // TODO: check the content of `stdout` and `stderr`
    // remove a dir after removing all its contents
    if workload.exists() {
        fs::remove_dir_all(workload).unwrap();
    }
}