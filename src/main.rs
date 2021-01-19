/// Auto Background Test Routine for XiangShan Processor
/// XiangShan: https://github.com/RISCVERS/XiangShan
/// Never panic

extern crate xscommand;
extern crate simple_logger;
extern crate threadpool;
extern crate thread_id;
extern crate chrono;
extern crate psutil;
extern crate toml;
extern crate serde;
extern crate serde_derive;

pub mod config;

#[allow(unused_imports)]
use std::{
    fs,
    path::Path,process::{Command, exit},
    thread, time::Duration,
    vec,
    io::{prelude::*, BufWriter},
    sync::Arc,
};
use config::Config;
use simple_logger::SimpleLogger;
#[allow(unused_imports)]
use xscommand::{
    XSCommand,
    XSCommandErr,
    git::Git,
    make::Make,
    emu::Emu,
    numactl::Numactl,
};
use threadpool::ThreadPool;
use chrono::prelude::*;

const WORKERS_NUM: usize = 3;
const WORK_ROOT: &str = "/home/ccc/rust_xs_test_workload";
const SLEEP_TIME: u64 = 120;
const IMG: &str = "/bigdata/zyy/checkpoints_profiles/betapoint_profile_06/gcc_200/0/_8000000000_.gz";
const THREAD_NUM: usize = 8;
const NEMU_HOME: &str = "/home/ccc/NEMU";
const AM_HOME: &str = "/home/ccc/nexus-am";

fn main() -> ! {
    println!("Hello, rust-xs-test!");
    // init simple logger
    let logger = SimpleLogger::new();
    logger.init().unwrap();
    let f = fs::read_to_string("config.toml").unwrap();
    let config: Config = toml::from_str(f.as_str()).unwrap();
    let config = Arc::new(config);
    let workers_num = if let Some(num) = config.workers_num() { num } else { WORKERS_NUM };
    let sleep_time = if let Some(time) = config.sleep_time() { time } else { SLEEP_TIME };
    // create a thread pool
    let pool = ThreadPool::new(workers_num);
    loop {
        if pool.active_count() >= workers_num {
            thread::sleep(Duration::from_secs(sleep_time));
            continue;
        }
        let config = Arc::clone(&config);
        pool.execute(move || {
            let time = get_workload_name();
            let work_root = if let Some(dir) =  config.work_root() { dir } else { WORK_ROOT };
            let work_root = Path::new(work_root);
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
            match Git::clone(repo_url, workload.to_str()) {
                Ok(exit_code) => {
                    log::info!("git clone {} exit with {}", repo_url, exit_code);
                    if exit_code != 0 {
                        log::error!("exit code not zero, thread {} exit.", thread_id::get());
                        exit(-1);
                    }
                },
                Err(err_code) => {
                    log::error!("git clone {} error with {}, thread {} exit.", repo_url, err_code, thread_id::get());
                    exit(-1);
                }
            }
            let workload = workload.join(url2path(repo_url));
            // enter XiangShan and make init
            match Make::init(workload.to_str()) {
                Ok(exit_code) => {
                    log::info!("make init in {:?} exit with {}", workload, exit_code);
                    if exit_code != 0 {
                        log::error!("exit code not zero, thread {} exit.", thread_id::get());
                        exit(-1);
                    }
                },
                Err(err_code) => {
                    log::error!("make init in {:?} error with {}, thread {} exit.", workload, err_code, thread_id::get());
                    exit(-1);
                }
            }
            // change the ram size
            let ram_h = workload.join("src/test/csrc/ram.h");
            let ram_h_contents = match fs::read_to_string(&ram_h) {
                Ok(content) => content,
                Err(_) => {
                    log::error!("failed to read ram.h, thread {} exit.", thread_id::get());
                    exit(-1);
                }
            };
            let new_contents: Vec<&str> = ram_h_contents.lines().map(|f| {
                if f.contains("#define EMU_RAM_SIZE (256 * 1024 * 1024UL)") {
                    "// #define EMU_RAM_SIZE (256 * 1024 * 1024UL)"
                } else if f.contains("// #define EMU_RAM_SIZE (8 * 1024 * 1024 * 1024UL)") {
                    "#define EMU_RAM_SIZE (8 * 1024 * 1024 * 1024UL)"
                } else {
                    f
                }
            }).collect();
            {
                let f = match fs::File::create(ram_h) {
                    Ok(ram_f) => ram_f,
                    Err(_) => {
                        log::error!("failed to open ram.h, thread {} exit", thread_id::get());
                        exit(-1);
                    }
                };
                let mut buf_writer = BufWriter::new(f);
                for line in new_contents {
                    match buf_writer.write(line.as_bytes()) {
                        Ok(_) => {},
                        Err(_) => {
                            log::error!("BufWriter write line error, thread {} exit.", thread_id::get());
                            exit(-1);
                        }
                    }
                    match buf_writer.write(b"\n") {
                        Ok(_) => {},
                        Err(_) => {
                            log::error!("BufWriter write \\n error, thread {} exit.", thread_id::get());
                            exit(-1);
                        }
                    }
                }
            }
            // numatcl -C 0-255 make build/emu EMU_TRACE=1 SIM_ARGS="--disable-all" EMU_THREADS=thread_num -j256
            let thread_num = if let Some(num) = config.thread_num() { num } else { THREAD_NUM };
            let nemu_home = if let Some(path) = config.nemu_home() { path } else { NEMU_HOME };
            let am_home = if let Some(path) = config.am_home() { path } else { AM_HOME };
            match Numactl::make_emu(workload.to_str(), nemu_home, am_home, thread_num) {
                Ok(exit_code) => {
                    log::info!("make emu in {:?} exit with {}", workload, exit_code);
                    if exit_code != 0 {
                        log::error!("exit code not zero, thread {} exit.", thread_id::get());
                        exit(-1);
                    }
                },
                Err(err_code) => {
                    log::error!("make emu in {:?} error with {}, thread {} exit.", workload, err_code, thread_id::get());
                    exit(-1);
                }
            }
            // create ../emu_res dir && 
            // numactl -C [] emu -I 1000000 -i /bigdata/zyy/checkpoints_profiles/betapoint_profile_06/gcc_200/0/_8000000000_.gz
            let res_dir = workload.join("../emu_res");
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
            let emu = if let Some(path) = emu_path.to_str() {
                log::info!("create emu in {}", path);
                path
            } else {
                log::error!("no path in emu_path, thread {} exit", thread_id::get());
                exit(-1);
            };
            let img = if let Some(path) = config.img() { path } else { IMG };
            match Numactl::run_emu(
                workload.to_str(),
                stdout_f.to_str(),
                stderr_f.to_str(),
                emu,
                img,
                nemu_home,
                am_home,
                thread_num
            ) {
                    Ok(exit_code) => {
                        log::info!("run emu in {:?} exit with {}", workload, exit_code);
                        if exit_code != 0 {
                            log::error!("exit code not zero, thread {} exit.", thread_id::get());
                            exit(-1);
                        }
                    },
                    Err(err_code) => {
                        log::error!("run emu in {:?} error with {}, thread {} exit.", workload, err_code, thread_id::get());
                        exit(-1);
                    }
                }
            log::info!("thread {} return 0", thread_id::get());
            
        });
        thread::sleep(Duration::from_secs(sleep_time));
        assert!(pool.active_count() <= workers_num);
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

#[test]
fn test_read_config() {
    let toml_f = fs::read_to_string("config.toml").unwrap();
    let conf: Config = toml::from_str(toml_f.as_str()).unwrap();
    assert_eq!(conf.workers_num(), Some(1));
    assert_eq!(conf.work_root(), Some("/home/ccc/rust_xs_test_workload"));
    assert_eq!(conf.sleep_time(), Some(120));
    assert_eq!(conf.img(), Some("/bigdata/zyy/checkpoints_profiles/betapoint_profile_06/gcc_200/0/_8000000000_.gz"));
    assert_eq!(conf.thread_num(), Some(8));
    assert_eq!(conf.nemu_home(), Some("/home/ccc/NEMU"));
    assert_eq!(conf.am_home(), Some("/home/ccc/nexus-am"));
}