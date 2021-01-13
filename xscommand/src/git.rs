//! Git Command Implementation
//! 

use super::{XSCommand, XSCommandErr};
use std::{
    process::{Command, Stdio},
    os::unix::io::{FromRawFd, IntoRawFd},
    fs::File,
};
#[allow(unused_imports)]
use simple_logger::SimpleLogger;

/// Git Command
pub struct Git<'a> {
    exe: Command,
    args: Vec<&'a str>,
    work_dir: Option<&'a str>,
}


impl<'a> XSCommand<'a, GitErr> for Git<'a> {
    fn new() -> Self {
        let git = Command::new("git");
        let args = Vec::new();
        Self {
            exe: git,
            args,
            work_dir: None,
        }
    }

    fn set_args(&mut self, args: Vec<&'a str>) -> Result<(), GitErr> {
        // TODO: check the rationality of args
        self.args = args;
        Ok(())
    }

    fn get_args(&self) -> Vec<&str> {
        // let mut args = Vec::new();
        // for arg in &self.args {
        //     args.push(*arg);
        // }
        // I can write beautiful code like this now!
        let args: Vec<&str> = self.args.iter().map(|a| *a).collect();
        args
    }

    fn set_workdir(&mut self, work_dir: &'a str) -> Result<(), GitErr> {
        // TODO: check the rationality of workdir
        self.work_dir = Some(work_dir);
        Ok(())
    }

    fn excute(&mut self, stdout: Option<&str>, stderr: Option<&str>) -> Result<i32, GitErr> {
        for arg in &self.args {
            self.exe.arg(arg);
        }
        log::info!("git excute args: {:?}", self.args);
        if let Some(stdout_path) = stdout {
            let stdout_fd = File::create(stdout_path).unwrap().into_raw_fd();
            let std_out = unsafe { Stdio::from_raw_fd(stdout_fd) };
            self.exe.stdout(std_out);
        }
        if let Some(stderr_path) = stderr {
            let stderr_fd = File::create(stderr_path).unwrap().into_raw_fd();
            let err_out = unsafe { Stdio::from_raw_fd(stderr_fd) };
            self.exe.stderr(err_out);
        }
        if let Some(dir) = self.work_dir {
            self.exe.current_dir(dir);
        }
        let res = self.exe.status();
        match res {
            Ok(exit_status) => {
                if let Some(exit_code) = exit_status.code() {
                    log::info!("git excute with exit code: {}", exit_code);
                    Ok(exit_code)
                } else {
                    // TODO: Return GitErr
                    todo!()
                }
            },
            Err(_) => {
                // TODO: Error Handler or GitErr
                todo!();
            }
        }
    }
}

#[derive(Debug)]
pub enum GitErr {
    // TODO
}

impl XSCommandErr for GitErr {
    fn as_str(&self) -> &str {
        todo!()
    }

    fn err_code(&self) -> i32 {
        todo!()
    }
}

#[test]
fn test_git_version() {
    #[allow(unused_imports)]
    use std::fs::{remove_file, File};
    let stdout_file = "git_version_stdout.txt";
    let stderr_file = "git_version_stderr.txt";
    let mut git = Git::new();
    let args = vec!["--version"];
    git.set_args(args).unwrap();
    match git.excute(Some(stdout_file), Some(stderr_file)) {
        Ok(exit_code) => {
            assert_eq!(exit_code, 0);
        },
        Err(errtype) => panic!("ErrType: {:?}", errtype),
    }
    // TODO: check the content of `stdout.txt` and `stderr.txt` 
    // File::create(stdout_file).unwrap();
    use std::path::Path;
    if Path::new(stdout_file).exists() {
        remove_file(stdout_file).unwrap();
    }
    // File::create(stderr_file).unwrap();
    if Path::new(stderr_file).exists() {
        remove_file(stderr_file).unwrap();
    }
}

// TODO: add more test
#[test]
fn test_git_status() {
    #[allow(unused_imports)]
    use std::fs::{remove_file, File};
    let stdout_file = "git_status_stdout.txt";
    let stderr_file = "git_status_stderr.txt";
    let mut git = Git::new();
    let args = vec!["status", "--help"];
    match git.set_args(args) {
        Err(errtype) => panic!("ErrType: {:?}", errtype),
        Ok(_) => {},
    }
    match git.excute(Some(stdout_file), Some(stderr_file)) {
        Ok(exit_code) => {
            assert_eq!(exit_code, 0);
        },
        Err(errtype) => panic!("ErrType: {:?}", errtype),
    }
    // TODO: check the content of `stdout.txt` and `stderr.txt`
    use std::path::Path;
    if Path::new(stdout_file).exists() {
        remove_file(stdout_file).unwrap();
    }
    // File::create(stderr_file).unwrap();
    if Path::new(stderr_file).exists() {
        remove_file(stderr_file).unwrap();
    }
}

#[test]
fn test_git_clone() {
    #[allow(unused_imports)]
    use std::fs::{remove_file, File};
    use std::process::Command;
    let stdout_file = "git_clone_stdout.txt";
    let stderr_file = "git_clone_stderr.txt";
    let repo = "https://github.com/SKTT1Ryze/rust-xs-evaluation";
    let repo_path = "rust-xs-evaluation";
    let mut git = Git::new();
    let args = vec!["clone", repo];
    match git.set_args(args) {
        Err(errtype) => panic!("ErrType: {:?}", errtype),
        Ok(_) => {},
    }
    match git.excute(Some(stdout_file), Some(stderr_file)) {
        Ok(exit_code) => {
            assert_eq!(exit_code, 0);
        },
        Err(errtype) => panic!("ErrType: {:?}", errtype),
    }
    // TODO: check the content of `stdout.txt` and `stderr.txt`
    use std::path::Path;
    if Path::new(stdout_file).exists() {
        remove_file(stdout_file).unwrap();
    }
    // File::create(stderr_file).unwrap();
    if Path::new(stderr_file).exists() {
        remove_file(stderr_file).unwrap();
    }
    let mut rm = Command::new("rm")
        .arg("-rf")
        .arg(repo_path)
        .spawn()
        .unwrap();
    rm.wait().unwrap();
}

#[test]
fn test_git_pull() {
    #[allow(unused_imports)]
    use std::fs::{remove_file, File};
    use std::process::Command;
    let stdout_file = "git_pull_stdout.txt";
    let stderr_file = "git_pull_stderr.txt";
    let repo = "https://github.com/SKTT1Ryze/rust-xs-evaluation";
    let repo_path = "rust-xs-evaluation";
    let mut git_clone = Command::new("git")
        .arg("clone")
        .arg(repo)
        .spawn()
        .unwrap();
    git_clone.wait().unwrap();
    let mut git = Git::new();
    let args = vec!["pull"];
    git.set_args(args).unwrap();
    match git.excute(Some(stdout_file), Some(stderr_file)) {
        Ok(exit_code) => {
            assert_eq!(exit_code, 0);
        },
        Err(errtype) => panic!("ErrType: {:?}", errtype),
    }
    // TODO: check the content of `stdout.txt` and `stderr.txt`
    use std::path::Path;
    if Path::new(stdout_file).exists() {
        remove_file(stdout_file).unwrap();
    }
    // File::create(stderr_file).unwrap();
    if Path::new(stderr_file).exists() {
        remove_file(stderr_file).unwrap();
    }
    let mut rm = Command::new("rm")
        .arg("-rf")
        .arg(repo_path)
        .spawn()
        .unwrap();
    rm.wait().unwrap();
}