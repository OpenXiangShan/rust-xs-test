/// Auto Background Test Routine for XiangShan Processor
/// XiangShan: https://github.com/RISCVERS/XiangShan
/// Never panic

extern crate xscommand;
extern crate simple_logger;
extern crate threadpool;
extern crate thread_id;
extern crate chrono;
#[allow(unused_imports)]
use std::{fs, path::Path, process::{Command, exit}, thread, time::Duration, vec};
use simple_logger::SimpleLogger;
use xscommand::{
    XSCommand,
    XSCommandErr,
    git::Git,
    make::Make,
    emu::Emu,
};
use threadpool::ThreadPool;
use chrono::prelude::*;

const WORKERS_NUM: usize = 5;
const WORK_ROOT: &str = "/home/ccc/rust_xs_test_workload";
const SLEEP_TIME: u64 = 120;
const EMU_TARGET: &str = "/bigdata/zyy/checkpoints_profiles/betapoint_profile_06/gcc_200/0/_8000000000_.gz";
const NEMU_HOME: &str = "/home/ccc/NEMU";
const AM_HOME: &str = "/home/ccc/nexus-am";

fn main() -> ! {
    println!("Hello, rust-xs-test!");
    // init simple logger
    let logger = SimpleLogger::new();
    logger.init().unwrap();
    // create a thread pool
    let pool = ThreadPool::new(WORKERS_NUM);

    loop {
        if pool.active_count() >= WORKERS_NUM {
            thread::sleep(Duration::from_secs(SLEEP_TIME));
            continue;
        }
        pool.execute(move || {
            // TODO: use function to return `time`
            let time = get_workload_name();
            let work_root = Path::new(WORK_ROOT);
            let workload = work_root.join(time);
            if !workload.exists() {
                match fs::create_dir_all(workload.as_path()) {
                    Ok(_) => {}, // do nothing
                    Err(msg) => {
                        log::error!("Failed in creating workload {:?} with msg
                        {} , thread {} exit.", workload, msg, thread_id::get());
                        // TODO: specify exit code
                        exit(-1);
                    },
                }
            }
            let repo_url = "https://github.com/RISCVERS/XiangShan.git";
            // git clone XiangShan
            {
                let stdout_f = workload.join("stdout.txt");
                let stderr_f = workload.join("stderr.txt");
                let mut git = Git::set_exe("git");
                let args = vec!["clone", repo_url];
                // use macro to reduce below code lines
                match git.set_args(args) {
                    Ok(_) => {}, // do nothing
                    Err(git_err) => {
                        log::error!("{}, thread {} exit.", git_err.as_str(), thread_id::get());
                        // TODO: specify exit code
                        exit(-1);
                    }
                }
                match git.set_workdir(workload.to_str()) {
                    Ok(_) => {}, // do nothing
                    Err(git_err) => {
                        log::error!("{}, thread {} exit.", git_err.as_str(), thread_id::get());
                        // TODO: specify exit code
                        exit(-1);
                    }
                }
                match git.excute(stdout_f.to_str(), stderr_f.to_str()) {
                    Ok(exit_code) => {
                        log::info!("git with args: {:?} excute return {}", git.get_args(), exit_code);
                    },
                    Err(git_err) => {
                        log::error!("{}, thread {} exit.", git_err.as_str(), thread_id::get());
                        // TODO: specify exit code
                        exit(-1);
                    }
                }
                // git drop here
            }
            let workload = workload.join(url2path(repo_url));
            // enter XiangShan and make init
            {
                let stdout_f = workload.join("stdout.txt");
                let stderr_f = workload.join("stderr.txt");
                let mut make = Make::set_exe("make");
                match make.set_args(vec!["init"]) {
                    Ok(_) => {}, // do nothing
                    Err(make_err) => {
                        log::error!("{}, thread {} exit", make_err.as_str(), thread_id::get());
                        // TODO: specify exit code
                        exit(-1);
                    }
                }
                match make.set_workdir(workload.to_str()) {
                    Ok(_) => {}, // do nothing
                    Err(make_err) => {
                        log::error!("{}, thread {} exit", make_err.as_str(), thread_id::get());
                        // TODO: specify exit code
                        exit(-1);
                    }
                }
                match make.excute(stdout_f.to_str(), stderr_f.to_str()) {
                    Ok(exit_code) => {
                        log::info!("make with args: {:?} excute return {}", make.get_args(), exit_code);
                    },
                    Err(make_err) => {
                        log::error!("{}, thread {} exit.", make_err.as_str(), thread_id::get());
                        // TODO: specify exit code
                        exit(-1);
                    }
                }
                // make drop here
            }
            // make emu
            {
                let stdout_f = workload.join("stdout.txt");
                let stderr_f = workload.join("stderr.txt");
                let mut make = Make::set_exe("make");
                match make.set_args(vec!["emu"]) {
                    Ok(_) => {}, // do nothing
                    Err(make_err) => {
                        log::error!("{}, thread {} exit", make_err.as_str(), thread_id::get());
                        // TODO: specify exit code
                        exit(-1);
                    }
                }
                match make.set_workdir(workload.to_str()) {
                    Ok(_) => {}, // do nothing
                    Err(make_err) => {
                        log::error!("{}, thread {} exit", make_err.as_str(), thread_id::get());
                        // TODO: specify exit code
                        exit(-1);
                    }
                }
                make.exe.env("NEMU_HOME", NEMU_HOME);
                make.exe.env("AM_HOME", AM_HOME);
                match make.excute(stdout_f.to_str(), stderr_f.to_str()) {
                    Ok(exit_code) => {
                        log::info!("make with args: {:?} excute return {}", make.get_args(), exit_code);
                    },
                    Err(make_err) => {
                        log::error!("{}, thread {} exit.", make_err.as_str(), thread_id::get());
                        // TODO: specify exit code
                        exit(-1);
                    }
                }
                // make drop here
            }
            // create ../emu_res dir && 
            // emu -I 1000000 -i /bigdata/zyy/checkpoints_profiles/betapoint_profile_06/gcc_200/0/_8000000000_.gz
            let res_dir = workload.join("../emu_res");
            {
                if !res_dir.exists() {
                    match fs::create_dir_all(res_dir.as_path()) {
                        Ok(_) => {}, // do nothing
                        Err(msg) => {
                            log::error!("Failed in creating res_dir {:?} with msg
                            {} , thread {} exit.", res_dir, msg, thread_id::get());
                            // TODO: specify exit code
                            exit(-1);
                        },
                    }
                }
                let stdout_f = res_dir.join("stdout.txt");
                let stderr_f = res_dir.join("stderr.txt");
                let emu_path = workload.join("./build/emu");
                let mut emu = if let Some(path) = emu_path.to_str() {
                    log::info!("create emu in {}", path);
                    Emu::set_exe(path)
                } else {
                    log::info!("no path in emu_path, thread {} exit", thread_id::get());
                    exit(-1);
                };                
                match emu.set_args(vec!["-I", "1000000", "-i", EMU_TARGET]) {
                    Ok(_) => {}, // do nothing
                    Err(emu_err) => {
                        log::error!("{}, thread {} exit", emu_err.as_str(), thread_id::get());
                        // TODO: specify exit code
                        exit(-1);
                    }
                }
                match emu.set_workdir(workload.to_str()) {
                    Ok(_) => {}, // do nothing
                    Err(emu_err) => {
                        log::error!("{}, thread {} exit", emu_err.as_str(), thread_id::get());
                        // TODO: specify exit code
                        exit(-1);
                    }
                }
                // set env `NOOP_HOME` to currnet workload
                emu.exe.env("NOOP_HOME", workload.to_path_buf());
                emu.exe.env("NEMU_HOME", NEMU_HOME);
                emu.exe.env("AM_HOME", AM_HOME);
                match emu.excute(stdout_f.to_str(), stderr_f.to_str()) {
                    Ok(exit_code) => {
                        log::info!("make with args: {:?} excute return {}", emu.get_args(), exit_code);
                    },
                    Err(emu_err) => {
                        log::error!("{}, thread {} exit.", emu_err.as_str(), thread_id::get());
                        // TODO: specify exit code
                        exit(-1);
                    }
                }
            }
            log::info!("thread {} return 0", thread_id::get());
            
        });
        thread::sleep(Duration::from_secs(SLEEP_TIME));
        assert!(pool.active_count() <= WORKERS_NUM);
    }

}

fn get_workload_name() -> String {
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
    time
}

fn url2path(url: &str) -> &str {
    let url: Vec<&str> = url.split('/').collect();
    let path = url[url.len() - 1];
    &path[..(path.len() - 4)]
}

#[test]
fn test_url2path() {
    assert_eq!(url2path("https://github.com/RISCVERS/XiangShan.git"), "XiangShan");
}