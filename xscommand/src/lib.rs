//! `xscommand` crate is for abstraction of command used in XiangShan development 
//! like `sh`, `git` and `emu` which is a simulator in the development of XiangShan
//! 

/// Command used in
/// XiangShan development
pub trait XSCommand<T: XSCommandErr> {
    fn empty() -> Self;
    fn set_exe(&mut self, path: &str) -> Result<(), T>;
    fn set_args(&mut self, args: Vec<&str>) -> Result<(), T>;
    fn get_args(&self) -> Vec<&str>;
    fn excute(&self) -> Result<(), T>;
}

/// XSCommand Error
pub trait XSCommandErr{
    fn as_str(&self) -> &str;
    fn err_code(&self) -> u8;
}

pub enum DefaultErr {
    SetExeErr,
    SetArgsErr,
    ExcuteErr,
}

impl XSCommandErr for DefaultErr {
    fn as_str(&self) -> &str {
        match self {
            DefaultErr::SetExeErr => "default set exe err",
            DefaultErr::SetArgsErr => "default set args err",
            DefaultErr::ExcuteErr => "default excute err",
        }
    }
    fn err_code(&self) -> u8 {
        todo!()
    }
}