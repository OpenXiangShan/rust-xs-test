/// Auto Background Test Routine for XiangShan Processor
/// XiangShan: https://github.com/RISCVERS/XiangShan
/// Never panic

extern crate xscommand;
extern crate simple_logger;
extern crate threadpool;
extern crate thread_id;
extern crate chrono;

use std::{fs, path::Path, process::exit, thread, time::Duration, vec};
use simple_logger::SimpleLogger;
use xscommand::{
    XSCommand,
    XSCommandErr,
    git::Git,
};
use threadpool::ThreadPool;
use chrono::{LocalResult, prelude::*};

const WORKERS_NUM: usize = 5;
const WORK_ROOT: &str = "/home/hustccc/rust_xs_test_workload";

fn main() -> ! {
    println!("Hello, rust-xs-test!");
    // init simple logger
    let logger = SimpleLogger::new();
    logger.init().unwrap();
    // create a thread pool
    let pool = ThreadPool::new(WORKERS_NUM);

    loop {
        if pool.active_count() >= WORKERS_NUM {
            thread::sleep(Duration::from_secs(10));
            continue;
        }
        pool.execute(move || {
            // TODO: use function to return `time`
            let local = Local::now();
            let mut time = local.year().to_string();
            let month = local.month().to_string();
            let day = local.day().to_string();
            let hour = local.hour().to_string();
            let min = local.minute().to_string();
            let sec = local.second().to_string();
            let underline = String::from("_");
            time.push_str(underline.as_str());
            time.push_str(month.as_str());
            time.push_str(underline.as_str());
            time.push_str(day.as_str());
            time.push_str(underline.as_str());
            time.push_str(hour.as_str());
            time.push_str(underline.as_str());
            time.push_str(min.as_str());
            time.push_str(underline.as_str());
            time.push_str(sec.as_str());
            let work_root = Path::new(WORK_ROOT);
            let workload = work_root.join(time);
            if !workload.exists() {
                match fs::create_dir_all(workload.as_path()) {
                    Ok(_) => {}, // do nothing
                    Err(msg) => {
                        log::error!("Failed in creating workload {:?} with msg
                        {} , thread {} exit.", workload, msg, thread_id::get());
                        // TODO: specify exit code
                        exit(1);
                    },
                }
            }
            let stdout_f = workload.join("stdout.txt");
            let stderr_f = workload.join("stderr.txt");
            let repo_url = "https://github.com/SKTT1Ryze/rust-order.git";
            let _repo_path = workload.join("rust-xs-evaluation");
            let mut git = Git::new();
            let args = vec!["clone", repo_url];
            // use macro to reduce below code lines
            match git.set_args(args) {
                Ok(_) => {}, // do nothing
                Err(git_err) => {
                    log::error!("{}, thread {} exit.", git_err.as_str(), thread_id::get());
                    // TODO: specify exit code
                    exit(1);
                }
            }
            match git.set_workdir(workload.to_str()) {
                Ok(_) => {}, // do nothing
                Err(git_err) => {
                    log::error!("{}, thread {} exit.", git_err.as_str(), thread_id::get());
                    // TODO: specify exit code
                    exit(1);
                }
            }
            match git.excute(stdout_f.to_str(), stderr_f.to_str()) {
                Ok(exit_code) => {
                    log::info!("git with args: {:?} excute return {}", git.get_args(), exit_code);
                },
                Err(git_err) => {
                    log::error!("{}, thread {} exit.", git_err.as_str(), thread_id::get());
                    // TODO: specify exit code
                    exit(1);
                }
            }
            log::info!("thread {} return 0", thread_id::get());
            
        });
        thread::sleep(Duration::from_secs(10));
        assert!(pool.active_count() <= WORKERS_NUM);
    }

}
