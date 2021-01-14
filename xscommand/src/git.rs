//! Git Command Implementation
//! 

use super::{XSCommand, XSCommandErr};
use std::{
    fs::File,
    os::unix::io::{FromRawFd, IntoRawFd},
    process::{Command, Stdio}
};

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

    fn set_workdir(&mut self, work_dir: Option<&'a str>) -> Result<(), GitErr> {
        // TODO: check the rationality of workdir
        self.work_dir = work_dir;
        Ok(())
    }

    fn excute(&mut self, stdout: Option<&str>, stderr: Option<&str>) -> Result<i32, GitErr> {
        for arg in &self.args {
            self.exe.arg(arg);
        }
        let workload = if let Some(dir) = self.work_dir { dir } else { "./" };
        log::info!("git excute args: {:?} in workload: {}", self.args, workload);
        // TODO: use clouse here to reduce code lines
        if let Some(stdout_path) = stdout {
            // TODO: should not panic here
            let stdout_fd = File::create(stdout_path).unwrap().into_raw_fd();
            let std_out = unsafe { Stdio::from_raw_fd(stdout_fd) };
            self.exe.stdout(std_out);
        }
        if let Some(stderr_path) = stderr {
            // TODO: should not panic here
            let stderr_fd = File::create(stderr_path).unwrap().into_raw_fd();
            let err_out = unsafe { Stdio::from_raw_fd(stderr_fd) };
            self.exe.stderr(err_out);
        }
        if let Some(dir) = self.work_dir {
            self.exe.current_dir(dir);
        }
        // Block here until command return
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
    use std::fs;
    use std::path::Path;
    let workload = Path::new("git_version_test");
    if !workload.exists() {
        // workload not exists, create
        fs::create_dir(workload).unwrap();
    }
    let stdout_file = workload.join("git_version_stdout.txt");
    let stderr_file = workload.join("git_version_stderr.txt");
    let mut git = Git::new();
    let args = vec!["--version"];
    git.set_args(args).unwrap();
    git.set_workdir(workload.to_str()).unwrap();
    match git.excute(stdout_file.to_str(), stderr_file.to_str()) {
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
    // if stdout_file.exists() {
    //     fs::remove_file(stdout_file).unwrap();
    // }
    // if stderr_file.exists() {
    //     fs::remove_file(stderr_file).unwrap();
    // }
}

// TODO: add more test
#[test]
fn test_git_status() {
    use std::fs;
    use std::path::Path;
    let workload = Path::new("git_status_test");
    if !workload.exists() {
        // workload not exists, create
        fs::create_dir(workload).unwrap();
    }
    let stdout_file = workload.join("git_status_stdout.txt");
    let stderr_file = workload.join("git_status_stderr.txt");
    let mut git = Git::new();
    let args = vec!["status", "--help"];
    match git.set_args(args) {
        Err(errtype) => panic!("ErrType: {:?}", errtype),
        Ok(_) => {},
    }
    git.set_workdir(workload.to_str()).unwrap();
    match git.excute(stdout_file.to_str(), stderr_file.to_str()) {
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
    // if stdout_file.exists() {
    //     fs::remove_file(stdout_file).unwrap();
    // }
    // if stderr_file.exists() {
    //     fs::remove_file(stderr_file).unwrap();
    // }
}

#[test]
fn test_git_clone() {
    use std::fs;
    use std::path::Path;
    let workload = Path::new("git_clone_test");
    if !workload.exists() {
        // workload not exists, create
        fs::create_dir(workload).unwrap();
    }
    let stdout_file = workload.join("git_clone_stdout.txt");
    let stderr_file = workload.join("git_clone_stderr.txt");
    let repo = "https://github.com/SKTT1Ryze/rust-xs-evaluation";
    let repo_path = workload.join("rust-xs-evaluation");
    if repo_path.exists() {
        fs::remove_dir_all(repo_path).unwrap();
    }
    let mut git = Git::new();
    let args = vec!["clone", repo];
    match git.set_args(args) {
        Err(errtype) => panic!("ErrType: {:?}", errtype),
        Ok(_) => {},
    }
    git.set_workdir(workload.to_str()).unwrap();
    match git.excute(stdout_file.to_str(), stderr_file.to_str()) {
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
    // if stdout_file.exists() {
    //     fs::remove_file(stdout_file).unwrap();
    // }
    // // File::create(stderr_file).unwrap();
    // if stderr_file.exists() {
    //     fs::remove_file(stderr_file).unwrap();
    // }
    
}

#[test]
fn test_git_pull() {
    use std::fs;
    use std::path::Path;
    use std::process::Command;
    let workload = Path::new("git_pull_test");
    if !workload.exists() {
        // workload not exists, create
        fs::create_dir(workload).unwrap();
    }
    let stdout_file = workload.join("git_pull_stdout.txt");
    let stderr_file = workload.join("git_pull_stderr.txt");
    let repo = "https://github.com/SKTT1Ryze/rust-xs-evaluation";
    let repo_path = workload.join("rust-xs-evaluation");
    if repo_path.exists() {
        fs::remove_dir_all(repo_path).unwrap();
    }
    let mut git_clone = Command::new("git")
        .arg("clone")
        .arg(repo)
        .current_dir(workload)
        .spawn()
        .unwrap();
    git_clone.wait().unwrap();
    let mut git = Git::new();
    let args = vec!["pull"];
    git.set_args(args).unwrap();
    git.set_workdir(workload.to_str()).unwrap();
    match git.excute(stdout_file.to_str(), stderr_file.to_str()) {
        Ok(exit_code) => {
            assert_eq!(exit_code, 0);
        },
        Err(errtype) => panic!("ErrType: {:?}", errtype),
    }
    // TODO: check the content of `stdout` and `stderr`
    if workload.exists() {
        fs::remove_dir_all(workload).unwrap();
    }
    // if stdout_file.exists() {
    //     fs::remove_file(stdout_file).unwrap();
    // }
    // // File::create(stderr_file).unwrap();
    // if stderr_file.exists() {
    //     fs::remove_file(stderr_file).unwrap();
    // }
    
}