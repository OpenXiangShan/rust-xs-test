/// Just XSCommand Implementation
/// 

use std::{
    fs::File,
    os::unix::io::{FromRawFd, IntoRawFd},
    process::{Command, Stdio}
};

use super::{XSCommand, DefaultErr};
use xscommand_macros::XSCommand;

/// Just XSCommand
#[derive(XSCommand)]
pub struct Just<'a> {
    pub exe: Command,
    args: Vec<&'a str>,
    work_dir: Option<&'a str>,
}

impl<'a> Just<'a> {
    pub fn run() -> i32 {
        todo!()
    }
}
#[test]
fn test_to_string() {
    let mut just = Just::new("just");
    just.set_args(vec!["run", "test"]);
    assert_eq!(just.to_string(), String::from("just run test"));
}

#[test]
fn test_just_version() {
    use std::fs;
    use std::path::Path;
    let workload = Path::new("just_version_test");
    if !workload.exists() {
        // workload not exists, create
        fs::create_dir(workload).unwrap();
    }
    let stdout_file = workload.join("just_version_stdout.txt");
    let stderr_file = workload.join("just_version_stderr.txt");
    let mut just = Just::new("just");
    let args = vec!["--version"];
    just.set_args(args);
    just.set_workdir(workload.to_str()).unwrap();
    match just.excute(stdout_file.to_str(), stderr_file.to_str()) {
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
fn test_just_help() {
    use std::fs;
    use std::path::Path;
    let workload = Path::new("just_help_test");
    if !workload.exists() {
        // workload not exists, create
        fs::create_dir(workload).unwrap();
    }
    let stdout_file = workload.join("just_help_stdout.txt");
    let stderr_file = workload.join("just_help_stderr.txt");
    let mut just = Just::new("just");
    let args = vec!["--help"];
    just.set_args(args);
    just.set_workdir(workload.to_str()).unwrap();
    match just.excute(stdout_file.to_str(), stderr_file.to_str()) {
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