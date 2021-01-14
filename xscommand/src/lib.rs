//! `xscommand` crate is for abstraction of command used in XiangShan development 
//! like `sh`, `git` and `emu` which is a simulator in the development of XiangShan
//! 
//! The project use this crate should init the [logger](https://github.com/rust-lang/log) in the main.rs, like this:  
//! ```no_run
//! extern crate simple_logger;
//! use simple_logger::SimpleLogger;
//! 
//! fn main() {
//!     let logger = SimpleLogger::new();
//!     logger.init().unwrap();
//! }
//! ```
//! 
//! Never panic in this crate

pub mod git;
pub mod make;

use std::fmt::Debug;

/// Command used in
/// XiangShan development
pub trait XSCommand<'a, T: XSCommandErr + Debug> {
    /// Create a command
    fn new() -> Self;
    /// Set arguments
    fn set_args(&mut self, args: Vec<&'a str>) -> Result<(), T>;
    /// Get arguments
    fn get_args(&self) -> Vec<&str>;
    /// Set the working dir for the XSCommand
    fn set_workdir(&mut self, work_dir: Option<&'a str>) -> Result<(), T>;
    /// Excute the command
    /// Return exit code 
    fn excute(&mut self, stdout: Option<&str>, stderr: Option<&str>) -> Result<i32, T>;
    fn to_string(&self) -> String;
}

/// XSCommand Error
pub trait XSCommandErr{
    /// to &str
    fn as_str(&self) -> &str;
    /// return specified code
    fn err_code(&self) -> i32;
}
#[derive(Debug)]
/// Default Error Type for XSCommand
pub enum DefaultErr {
    SetArgsErr,
    SetWorkDirErr,
    ExcuteErr(i32),
}

impl XSCommandErr for DefaultErr {
    fn as_str(&self) -> &str {
        match self {
            DefaultErr::SetArgsErr => "Default Set Args Error",
            DefaultErr::SetWorkDirErr  => "Default Set workload Error",
            DefaultErr::ExcuteErr(_) => "Default Excute Err",
        }
    }
    fn err_code(&self) -> i32 {
        todo!()
    }
}