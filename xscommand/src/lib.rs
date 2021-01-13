//! `xscommand` crate is for abstraction of command used in XiangShan development 
//! like `sh`, `git` and `emu` which is a simulator in the development of XiangShan
//! 

pub mod git;

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
    /// Excute the command
    /// Return exit code 
    fn excute(&mut self, stdout: Option<&str>, stderr: Option<&str>) -> Result<i32, T>;
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
    ExcuteErr,
}

impl XSCommandErr for DefaultErr {
    fn as_str(&self) -> &str {
        match self {
            DefaultErr::SetArgsErr => "default set args err",
            DefaultErr::ExcuteErr => "default excute err",
        }
    }
    fn err_code(&self) -> i32 {
        todo!()
    }
}