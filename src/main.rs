/// Auto Background Test Routine for XiangShan Processor
/// XiangShan: https://github.com/RISCVERS/XiangShan
/// Never panic

extern crate xscommand;
extern crate simple_logger;
extern crate threadpool;
extern crate thread_id;
extern crate chrono;
extern crate psutil;

#[allow(unused_imports)]
use std::{fs, path::Path, process::{Command, exit}, thread, time::Duration, vec};
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
            // {
            //     let stdout_f = workload.join("stdout.txt");
            //     let stderr_f = workload.join("stderr.txt");
            //     let mut git = Git::new("git");
            //     let args = vec!["clone", repo_url];
            //     // use macro to reduce below code lines
            //     git.set_args(args);
            //     match git.set_workdir(workload.to_str()) {
            //         Ok(_) => {}, // do nothing
            //         Err(git_err) => {
            //             log::error!("{}, thread {} exit.", git_err.as_str(), thread_id::get());
            //             // TODO: specify exit code
            //             exit(-1);
            //         }
            //     }
            //     match git.excute(stdout_f.to_str(), stderr_f.to_str()) {
            //         Ok(exit_code) => {
            //             log::info!("git with args: {:?} excute return {}", git.get_args(), exit_code);
            //             if exit_code != 0 {
            //                 log::error!("exit not zero, thread {} exit.", thread_id::get());
            //                 exit(-1);
            //             }
            //         },
            //         Err(git_err) => {
            //             log::error!("{}, thread {} exit.", git_err.as_str(), thread_id::get());
            //             // TODO: specify exit code
            //             exit(-1);
            //         }
            //     }
            //     // git drop here
            // }
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
            // {
            //     let stdout_f = workload.join("stdout.txt");
            //     let stderr_f = workload.join("stderr.txt");
            //     let mut make = Make::new("make");
            //     make.set_args(vec!["init"]);
            //     match make.set_workdir(workload.to_str()) {
            //         Ok(_) => {}, // do nothing
            //         Err(make_err) => {
            //             log::error!("{}, thread {} exit", make_err.as_str(), thread_id::get());
            //             // TODO: specify exit code
            //             exit(-1);
            //         }
            //     }
            //     match make.excute(stdout_f.to_str(), stderr_f.to_str()) {
            //         Ok(exit_code) => {
            //             log::info!("make with args: {:?} excute return {}", make.get_args(), exit_code);
            //             if exit_code != 0 {
            //                 log::error!("exit not zero, thread {} exit.", thread_id::get());
            //                 exit(-1);
            //             }
            //         },
            //         Err(make_err) => {
            //             log::error!("{}, thread {} exit.", make_err.as_str(), thread_id::get());
            //             // TODO: specify exit code
            //             exit(-1);
            //         }
            //     }
            //     // make drop here
            // }
            // numatcl -C 0-255 make build/emu EMU_TRACE=1 SIM_ARGS="--disable-all" EMU_THREADS=8 -j256
            match Numactl::make_emu(workload.to_str(), NEMU_HOME, AM_HOME) {
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
            // {
            //     let stdout_f = workload.join("stdout.txt");
            //     let stderr_f = workload.join("stderr.txt");
            //     let mut make = Make::new("numactl");
            //     make.set_args(vec!["-C", "0-255", "make", "build/emu", "EMU_TARCE=1", "SIM_ARGS=\"--disable-all\"", "EMU_THREADS=8", "-j256"]);
            //     match make.set_workdir(workload.to_str()) {
            //         Ok(_) => {}, // do nothing
            //         Err(make_err) => {
            //             log::error!("{}, thread {} exit", make_err.as_str(), thread_id::get());
            //             // TODO: specify exit code
            //             exit(-1);
            //         }
            //     }
            //     make.exe.env("NEMU_HOME", NEMU_HOME);
            //     make.exe.env("AM_HOME", AM_HOME);
            //     match make.excute(stdout_f.to_str(), stderr_f.to_str()) {
            //         Ok(exit_code) => {
            //             log::info!("make with args: {:?} excute return {}", make.get_args(), exit_code);
            //             if exit_code != 0 {
            //                 log::error!("exit not zero, thread {} exit.", thread_id::get());
            //                 exit(-1);
            //             }
            //         },
            //         Err(make_err) => {
            //             log::error!("{}, thread {} exit.", make_err.as_str(), thread_id::get());
            //             // TODO: specify exit code
            //             exit(-1);
            //         }
            //     }
            //     // make drop here
            // }
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
            match Numactl::run_emu(
                workload.to_str(),
                stdout_f.to_str(),
                stderr_f.to_str(),
                emu,
                EMU_TARGET,
                NEMU_HOME,
                AM_HOME,
                8) {
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
            // {
            //     if !res_dir.exists() {
            //         match fs::create_dir_all(res_dir.as_path()) {
            //             Ok(_) => {}, // do nothing
            //             Err(msg) => {
            //                 log::error!("Failed in creating res_dir {:?} with msg
            //                 {} , thread {} exit.", res_dir, msg, thread_id::get());
            //                 // TODO: specify exit code
            //                 exit(-1);
            //             },
            //         }
            //     }
            //     let stdout_f = res_dir.join("stdout.txt");
            //     let stderr_f = res_dir.join("stderr.txt");
            //     let emu_path = workload.join("./build/emu");
            //     let emu = if let Some(path) = emu_path.to_str() {
            //         log::info!("create emu in {}", path);
            //         path
            //     } else {
            //         log::error!("no path in emu_path, thread {} exit", thread_id::get());
            //         exit(-1);
            //     };
            //     let mut cpu_percent_collector = match psutil::cpu::CpuPercentCollector::new() {
            //         Ok(collector) => collector,
            //         Err(_) => {
            //             log::error!("new CpuPercentCollector error, thread {} exit.", thread_id::get());
            //             exit(-1);
            //         }
            //     };
            //     let mut numactl = Numactl::new("numactl");
            //     let cpu_percents = match cpu_percent_collector.cpu_percent_percpu() {
            //         Ok(percents) => percents,
            //         Err(_) => {
            //             log::error!("get cpu_percent_percpu error, thread {} exit.", thread_id::get());
            //             exit(-1);
            //         }
            //     };
            //     let (mut begin, mut end) = (0, 0);
            //     let mut count = 0;
            //     for _ in 0..3 {
            //         begin = 0;
            //         end = 0;
            //         for i in 0..cpu_percents.len() {
            //             if cpu_percents[i] > 50.0 {
            //                 begin += 1;
            //                 end = begin;
            //             } else {
            //                 end += 1;
            //             }
            //             if (end - begin) >= 7 {
            //                 // found 8 free cpus, break
            //                 break;
            //             }
            //         }
            //         if (end - begin) >= 7 {
            //             log::info!("found avaiable 8 cpus, ready to run emu.");
            //             break;
            //         }
            //         count += 1;
            //         thread::sleep(Duration::from_secs(SLEEP_TIME * 100));
            //     }
            //     if count >= 3 {
            //         log::error!("no 8 cpus avaiable, thread {} exit.", thread_id::get());
            //         exit(-1);
            //     }
            //     let mut cpu_ids = begin.to_string();
            //     cpu_ids.push_str("-");
            //     cpu_ids.push_str(end.to_string().as_str());
            //     log::info!("avaiable cpus: {}", cpu_ids);
            //     numactl.set_args(vec!["-C", cpu_ids.as_str(), emu, "-I", "1000000", "-i", EMU_TARGET]);
            //     match numactl.set_workdir(workload.to_str()) {
            //         Ok(_) => {}, // do nothing
            //         Err(emu_err) => {
            //             log::error!("{}, thread {} exit", emu_err.as_str(), thread_id::get());
            //             // TODO: specify exit code
            //             exit(-1);
            //         }
            //     }
            //     // set env `NOOP_HOME` to currnet workload
            //     numactl.exe.env("NOOP_HOME", workload.to_path_buf());
            //     numactl.exe.env("NEMU_HOME", NEMU_HOME);
            //     numactl.exe.env("AM_HOME", AM_HOME);
            //     match numactl.excute(stdout_f.to_str(), stderr_f.to_str()) {
            //         Ok(exit_code) => {
            //             log::info!("make with args: {:?} excute return {}", numactl.get_args(), exit_code);
            //             if exit_code != 0 {
            //                 log::error!("exit not zero, thread {} exit.", thread_id::get());
            //                 exit(-1);
            //             }
            //         },
            //         Err(emu_err) => {
            //             log::error!("{}, thread {} exit.", emu_err.as_str(), thread_id::get());
            //             // TODO: specify exit code
            //             exit(-1);
            //         }
            //     }
            // }
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