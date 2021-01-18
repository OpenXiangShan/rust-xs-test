/// Make XSCommand implementation
/// 

use std::{
    fs::File,
    os::unix::io::{FromRawFd, IntoRawFd},
    process::{Command, Stdio}
};

use super::{XSCommand, XSCommandErr, DefaultErr};
use xscommand_macros::XSCommand;

/// Make XSCommand
#[derive(XSCommand)]
pub struct Make<'a> {
    pub exe: Command,
    args: Vec<&'a str>,
    work_dir: Option<&'a str>,
}

impl<'a> Make<'a> {
    pub fn init(workload: Option<&str>) -> Result<i32, i32> {
        let mut make = Make::new("make");
        make.set_args(vec!["init"]);
        if let Err(err) = make.set_workdir(workload) {
            return Err(err.err_code());
        };
        match make.excute(None, None) {
            Ok(exit_code) => Ok(exit_code),
            Err(err) => Err(err.err_code()),
        }
    }

    pub fn emu(workload: Option<&str>) -> Result<i32, i32> {
        let mut make = Make::new("make");
        make.set_args(vec!["build/emu", "EMU_TARCE=1", "SIM_ARGS=\"--disable-all\"", "EMU_THREADS=8", "-j10"]);
        if let Err(err) = make.set_workdir(workload) {
            return Err(err.err_code());
        };
        match make.excute(None, None) {
            Ok(exit_code) => Ok(exit_code),
            Err(err) => Err(err.err_code()),
        }
    }
}

#[test]
fn test_to_string() {
    let mut make = Make::new("make");
    make.set_args(vec!["emu", "EMU_TRACE=1", "SIM_ARGS=\"--disable-all\"", "EMU_THREADS=8", "-j10"]);
    assert_eq!(make.to_string(), String::from("make emu EMU_TRACE=1 SIM_ARGS=\"--disable-all\" EMU_THREADS=8 -j10"));
}

#[test]
fn test_make_version() {
    use std::fs;
    use std::path::Path;
    let workload = Path::new("make_version_test");
    if !workload.exists() {
        // workload not exists, create
        fs::create_dir(workload).unwrap();
    }
    let stdout_file = workload.join("make_version_stdout.txt");
    let stderr_file = workload.join("make_version_stderr.txt");
    let mut make = Make::new("make");
    let args = vec!["--version"];
    make.set_args(args);
    make.set_workdir(workload.to_str()).unwrap();
    match make.excute(stdout_file.to_str(), stderr_file.to_str()) {
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

#[test]
fn test_make_help() {
    use std::fs;
    use std::path::Path;
    let workload = Path::new("make_help_test");
    if !workload.exists() {
        // workload not exists, create
        fs::create_dir(workload).unwrap();
    }
    let stdout_file = workload.join("make_help_stdout.txt");
    let stderr_file = workload.join("make_help_stderr.txt");
    let mut make = Make::new("make");
    let args = vec!["--help"];
    make.set_args(args);
    make.set_workdir(workload.to_str()).unwrap();
    match make.excute(stdout_file.to_str(), stderr_file.to_str()) {
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