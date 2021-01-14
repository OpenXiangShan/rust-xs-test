/// Auto Background Test Routine for XiangShan Processor
/// XiangShan: https://github.com/RISCVERS/XiangShan
/// Never panic

extern crate xscommand;
extern crate simple_logger;

use std::{
    path::Path,
    fs,
};
use simple_logger::SimpleLogger;
use xscommand::{
    XSCommand,
    git::Git,
};

fn main() {
    println!("Hello, rust-xs-test!");
    // init simple logger
    let logger = SimpleLogger::new();
    let workload = Path::new("temp");
    let stdout = workload.join("stdout");
    let stderr = workload.join("stderr");
    if !workload.exists() {
        fs::create_dir(workload).unwrap(); 
     }
    logger.init().unwrap();
    let mut git = Git::new();
    git.set_args(vec!["log"]).unwrap();
    git.set_workdir(workload.to_str()).unwrap();
    let exit_code = git.excute(stdout.to_str(), stderr.to_str()).unwrap();
    log::info!("git exit with code: {}", exit_code);
    if workload.exists() {
       fs::remove_dir_all(workload).unwrap(); 
    }
}
