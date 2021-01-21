//! Git XSCommand Implementation
//! 

use super::{XSCommand, XSCommandErr, DefaultErr};
use std::{
    fs::File,
    os::unix::io::{FromRawFd, IntoRawFd},
    process::{Command, Stdio}
};
use xscommand_macros::XSCommand;

/// Git XSCommand
#[derive(XSCommand)]
pub struct Git<'a> {
    pub exe: Command,
    args: Vec<&'a str>,
    work_dir: Option<&'a str>,
}

impl<'a> Git<'a> {
    // git version
    pub fn version() -> Result<i32, i32> {
        let mut git = Git::new("git");
        git.set_args(vec!["version"]);
        match git.excute(None, None) {
            Ok(exit_code) => Ok(exit_code),
            Err(err) => Err(err.err_code()),
        }
    }

    // git clone
    pub fn clone(url: &str, workload: Option<&str>) -> Result<i32, i32> {
        let mut git = Git::new("git");
        git.set_args(vec!["clone", url]);
        if let Err(err) = git.set_workdir(workload) {
            return Err(err.err_code());
        };
        match git.excute(None, None) {
            Ok(exit_code) => Ok(exit_code),
            Err(err) => Err(err.err_code()),
        }
    }

    // git pull
    pub fn pull(workload: Option<&str>) -> Result<i32, i32> {
        let mut git = Git::new("git");
        git.set_args(vec!["pull"]);
        if let Err(err) = git.set_workdir(workload) {
            return Err(err.err_code());
        };
        match git.excute(None, None) {
            Ok(exit_code) => Ok(exit_code),
            Err(err) => Err(err.err_code()),
        }
    }

    // git log
    pub fn log(stdout: Option<&str>, stderr: Option<&str>, workload: Option<&str>) -> Result<i32, i32> {
        let mut git = Git::new("git");
        git.set_args(vec!["log"]);
        if let Err(err) = git.set_workdir(workload) {
            return Err(err.err_code());
        };
        match git.excute(stdout, stderr) {
            Ok(exit_code) => Ok(exit_code),
            Err(err) => Err(err.err_code()),
        }
    }
}

#[test]
fn test_to_string() {
    let mut git = Git::new("git");
    git.set_args(vec!["clone", "https://github.com/SKTT1Ryze/rust-xs-evaluation"]);
    assert_eq!(git.to_string(), String::from("git clone https://github.com/SKTT1Ryze/rust-xs-evaluation"));
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
    let mut git = Git::new("git");
    let args = vec!["--version"];
    git.set_args(args);
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
}

#[test]
fn test_git_log() {
    use std::fs;
    use std::path::Path;
    let workload = Path::new("git_log_test");
    if !workload.exists() {
        // workload not exists, create
        fs::create_dir(workload).unwrap();
    }
    let stdout_file = workload.join("git_log_stdout.txt");
    let stderr_file = workload.join("git_log_stderr.txt");
    let mut git = Git::new("git");
    let args = vec!["log"];
    git.set_args(args);
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
    let mut git = Git::new("git");
    let args = vec!["status", "--help"];
    git.set_args(args);
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
    let mut git = Git::new("git");
    let args = vec!["clone", repo];
    git.set_args(args);
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
    let mut git = Git::new("git");
    let args = vec!["pull"];
    git.set_args(args);
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
}
