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

