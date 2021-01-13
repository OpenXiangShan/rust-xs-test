//! Git Command Implementation
//! 

use super::XSCommand;
use super::XSCommandErr;
use std::process::Command;


/// Git Command
pub struct Git<'a> {
    exe: Command,
    args: Vec<&'a str>,
}


impl<'a> XSCommand<'a, GitErr> for Git<'a> {
    fn new() -> Self {
        let git = Command::new("git");
        let args = Vec::new();
        Self {
            exe: git,
            args,
        }
    }

    fn set_args(&mut self, args: Vec<&'a str>) -> Result<(), GitErr> {
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

    fn excute(&mut self, _res_path: &str) -> Result<Option<i32>, GitErr> {
        for arg in &self.args {
            self.exe.arg(arg);
        }
        let output = self.exe.output();
        match output {
            Ok(res) => {
                // TODO: write the stdout and stderr to file
                Ok(res.status.code())
            },
            Err(_) => {
                // TODO: write the stdout and stderr to file
                let args: Vec<String> = self.args.iter().map(|a| String::from(*a)).collect();
                Err(GitErr::GitExcuteErr(args))
            }
        }        
    }
}

pub enum GitErr {
    GitSetArgsErr,
    GitExcuteErr(Vec<String>),
    // todo
}

impl XSCommandErr for GitErr {
    fn as_str(&self) -> &str {
        match self {
            GitErr::GitSetArgsErr => "git set args err",
            GitErr::GitExcuteErr(_) => "git excute err",
        }
    }

    fn err_code(&self) -> i32 {
        todo!()
    }
}

#[test]
fn test_git() {
    unimplemented!();
}


