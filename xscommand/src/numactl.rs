/// Numactl XSCommand Implementation
/// 

use super::{XSCommand, DefaultErr};
use std::{
    fs::File,
    os::unix::io::{FromRawFd, IntoRawFd},
    process::{Command, Stdio}
};
use xscommand_macros::XSCommand;

#[derive(XSCommand)]
pub struct Numactl<'a> {
    pub exe: Command,
    args: Vec<&'a str>,
    work_dir: Option<&'a str>,
}

impl<'a> Numactl<'a> {
    pub fn make_emu() -> i32 {
        todo!()
    }
    pub fn run_emu() -> i32 {
        todo!()
    }
}
