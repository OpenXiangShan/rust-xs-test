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
// impl<'a> XSCommand<'a, DefaultErr> for Make<'a> {
//     fn new() -> Self {
//         let make = Command::new("make");
//         let args = Vec::new();
//         Self {
//             exe: make,
//             args,
//             work_dir: None,
//         }
//     }

//     fn set_args(&mut self, args: Vec<&'a str>) -> Result<(), DefaultErr> {
//         // TODO: check the rationality of args
//         // Consider create a list to store available argements
//         self.args = args;
//         Ok(())
//     }

//     fn get_args(&self) -> Vec<&str> {
//         // let mut args = Vec::new();
//         // for arg in &self.args {
//         //     args.push(*arg);
//         // }
//         // I can write beautiful code like this now!
//         let args: Vec<&str> = self.args.iter().map(|a| *a).collect();
//         args
//     }

//     fn set_workdir(&mut self, work_dir: Option<&'a str>) -> Result<(), DefaultErr> {
//         // TODO: check the rationality of workdir
//         // Consider checking if the workdir readable and writable
//         self.work_dir = work_dir;
//         Ok(())
//     }

//     fn excute(&mut self, stdout: Option<&str>, stderr: Option<&str>) -> Result<i32, DefaultErr> {
//         for arg in &self.args {
//             self.exe.arg(arg);
//         }
//         let workload = if let Some(dir) = self.work_dir { dir } else { "./" };
//         log::info!("make excute args: {:?} in workload: {}", self.args, workload);
//         // TODO: use clouse here to reduce code lines
//         if let Some(stdout_path) = stdout {
//             let stdout_fd = match File::create(stdout_path) {
//                 Ok(fd) => {
//                     fd.into_raw_fd()
//                 },
//                 Err(_) => {
//                     // TODO: return MakeExcuteErr(err_code)
//                     todo!()
//                 }
//             };
//             // let stdout_fd = File::create(stdout_path).unwrap().into_raw_fd();
//             let std_out = unsafe { Stdio::from_raw_fd(stdout_fd) };
//             self.exe.stdout(std_out);
//         }
//         if let Some(stderr_path) = stderr {
//             let stderr_fd = match File::create(stderr_path) {
//                 Ok(fd) => {
//                     fd.into_raw_fd()
//                 },
//                 Err(_) => {
//                     // TODO: return MakeExcuteErr(err_code)
//                     todo!()
//                 }
//             };
//             let err_out = unsafe { Stdio::from_raw_fd(stderr_fd) };
//             self.exe.stderr(err_out);
//         }
//         if let Some(dir) = self.work_dir {
//             self.exe.current_dir(dir);
//         }
//         // Block here until command return
//         let res = self.exe.status();
//         match res {
//             Ok(exit_status) => {
//                 if let Some(exit_code) = exit_status.code() {
//                     log::info!("git excute with exit code: {}", exit_code);
//                     Ok(exit_code)
//                 } else {
//                     // TODO: return MakeExcuteErr(err_code)
//                     todo!()
//                 }
//             },
//             Err(_) => {
//                 // TODO: Error Handler or return MakeExcuteErr(err_code)
//                 todo!();
//             }
//         }
//     }

//     fn to_string(&self) -> String {
//         let mut name = String::from("make");
//         for arg in &self.args {
//             name.push_str(" ");
//             name.push_str(*arg);
//         }
//         name
//     }
// }

// #[derive(Debug)]
// pub enum MakeErr {
//     MakeSetArgsErr,
//     MakeSetWorkDirErr,
//     MakeExcuteErr(i32),
// }

// impl XSCommandErr for MakeErr {
//     fn as_str(&self) -> &str {
//         match self {
//             MakeErr::MakeSetArgsErr => "Make Set Args Error",
//             MakeErr::MakeSetWorkDirErr => "Make Set WorkDir Error",
//             MakeErr::MakeExcuteErr(_) => "Make Excute Error",
//         }
//     }

//     fn err_code(&self) -> i32 {
//         todo!()
//     }
// }

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