//! BusyBox XSCommand Implementation
//!
use super::{XSCommand, XSCommandErr, DefaultErr};
use std::{
    fs::File,
    os::unix::io::{FromRawFd, IntoRawFd},
    process::{Command, Stdio}
};
use xscommand_macros::XSCommand;

/// BusyBox XSCommand
#[derive(XSCommand)]
pub struct BusyBox<'a> {
    pub exe: Command,
    args: Vec<&'a str>,
    work_dir: Option<&'a str>,
}

impl<'a> BusyBox<'a> {
    // CP
    pub fn cp(source: &str, destination: &str, workload: Option<&str>) -> Result<i32, i32> {
        let mut cp = BusyBox::new("cp");
        cp.set_args(vec![source, destination]);
        if let Err(err) = cp.set_workdir(workload) {
            return Err(err.err_code());
        };
        match cp.excute(None, None) {
            Ok(exit_code) => Ok(exit_code),
            Err(err) => Err(err.err_code()),
        }
    }

    // mv
    pub fn mv(source: &str, destination: &str, workload: Option<&str>) -> Result<i32, i32> {
        let mut mv = BusyBox::new("mv");
        mv.set_args(vec![source, destination]);
        if let Err(err) = mv.set_workdir(workload) {
            return Err(err.err_code());
        };
        match mv.excute(None, None) {
            Ok(exit_code) => Ok(exit_code),
            Err(err) => Err(err.err_code()),
        }
    }

}