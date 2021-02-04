//! Auto Background Test Routine for XiangShan Processor
//! XiangShan: https://github.com/RISCVERS/XiangShan
//! Never return unless some untreatable errors occur

extern crate xscommand;
extern crate simple_logger;
extern crate threadpool;
extern crate chrono;
extern crate psutil;
extern crate toml;
extern crate serde;
extern crate serde_derive;

mod config;
mod tasks;
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
    busybox::BusyBox,
};
use threadpool::ThreadPool;
use chrono::prelude::*;

const WORKERS_NUM: usize = 3;
const WORK_ROOT: &str = "/home/ccc/rust_xs_test_workload";
const SLEEP_TIME: u64 = 120;
const IMG_LIST: &str = "/bigdata/zyy/checkpoints_profiles/betapoint_profile_06/";
const THREAD_NUM: usize = 8;
const NEMU_HOME: &str = "/home/ccc/NEMU";
const AM_HOME: &str = "/home/ccc/nexus-am";
const MAX_INSTR: usize = 1000000;

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
        pool.execute(move || { // TODO: return code when thread exit
            let time = get_workload_name();
            let work_root = if let Some(dir) =  config.work_root() { dir } else { WORK_ROOT };
            let work_root = Path::new(work_root);
            let workload = work_root.join(time);
            if !workload.exists() {
                match fs::create_dir_all(workload.as_path()) {
                    Ok(_) => {}, // do nothing
                    Err(msg) => {
                        log::error!("Failed in creating workload {:?} with msg
                        {} , thread exit.", workload, msg);
                        // TODO: specify exit code
                        return;
                    },
                }
            }
            let xs_home;
            if let Some(noop_home) = config.noop_home() {
                // use existing XiangShan proj
                xs_home = workload.join(noop_home);
            } else {
                let repo_url = "https://github.com/RISCVERS/XiangShan.git";
                // git clone XiangShan
                handle!(Git::clone(repo_url, workload.to_str()), false, workload);
                xs_home = workload.join(url2path(repo_url));
            }
            // enter XiangShan and make init
            handle!(Make::init(xs_home.to_str()), false, workload);

            // if use existing XiangShan proj, git pull
            handle!(Git::pull(xs_home.to_str()), false, workload);

            // change the ram size
            let ram_h = xs_home.join("src/test/csrc/ram.h");
            let ram_h_contents = match fs::read_to_string(&ram_h) {
                Ok(content) => content,
                Err(_) => {
                    log::error!("Failed to read ram.h, thread exit.");
                    return;
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
                        log::error!("Failed to open ram.h, thread exit");
                        return;
                    }
                };
                let mut buf_writer = BufWriter::new(f);
                for line in new_contents {
                    match buf_writer.write(line.as_bytes()) {
                        Ok(_) => {},
                        Err(_) => {
                            log::error!("BufWriter write line error, thread exit.");
                            return;
                        }
                    }
                    match buf_writer.write(b"\n") {
                        Ok(_) => {},
                        Err(_) => {
                            log::error!("BufWriter write \\n error, thread exit.");
                            return;
                        }
                    }
                }
            }

            // numatcl -C 0-255 make build/emu EMU_TRACE=1 SIM_ARGS="--disable-log" EMU_THREADS=thread_num -j256
            let thread_num = if let Some(num) = config.thread_num() { num } else { THREAD_NUM };
            let nemu_home = if let Some(path) = config.nemu_home() { path } else { NEMU_HOME };
            let am_home = if let Some(path) = config.am_home() { path } else { AM_HOME };
            handle!(Numactl::make_emu(xs_home.to_str(), nemu_home, am_home, thread_num), false, workload);
            
            // create ./emu_res dir && 
            let res_dir = workload.join("./emu_res");
            if !res_dir.exists() {
                match fs::create_dir_all(res_dir.as_path()) {
                    Ok(_) => {}, // do nothing
                    Err(msg) => {
                        log::error!("Failed in creating res_dir {:?} with msg
                        {} , thread exit.", res_dir, msg);
                        return;
                    },
                }
            }
            let stdout_f = res_dir.join("stdout.txt");
            let stderr_f = res_dir.join("stderr.txt");
            let emu_path = xs_home.join("./build/emu");
            let emu = if let Some(path) = emu_path.to_str() {
                log::info!("Found emu in {}", path);
                path
            } else {
                log::error!("No path in emu_path, thread exit");
                return;
            };

            // git log > workload/git_log.txt
            let git_log_f = workload.join("git_log.txt");
            handle!(Git::log(git_log_f.to_str(), None, xs_home.to_str()), false, workload);
           
            // cp XSSimTop.v && emu workload/
            let src_v = xs_home.join("build/XSSimTop.v");
            if let Some(dir) = workload.to_str() {
                if let Some(v) = src_v.to_str() {
                    handle!(BusyBox::cp(v, dir, workload.to_str()), false, workload);
                } else {
                    log::error!("no XSSimTop.v in {:?}/build, thread exit.", xs_home);
                    return;
                }
                handle!(BusyBox::cp(emu, dir, workload.to_str()), false, workload);
            }

            // numactl -C [] emu -I 1000000 -i test_img
            let img_list = if let Some(dir) = config.img_list() { dir } else { IMG_LIST };
            let tasks = tasks::tasks_list(img_list);
            use rand::Rng;
            let mut task_id = rand::thread_rng();
            let img = tasks[task_id.gen_range(0..tasks.len())].as_str();
            let max_instr = if let Some(max) = config.max_instr() { max } else { MAX_INSTR };
            handle!(
                Numactl::run_emu(
                    xs_home.to_str(),
                    stdout_f.to_str(),
                    stderr_f.to_str(),
                    emu,
                    img,
                    nemu_home,
                    am_home,
                    thread_num,
                    max_instr
                ),
                true,
                workload
            );
            log::info!("thread return 0");
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

#[macro_export]
macro_rules! handle {
    // handle!(Result<i32, i32>, Bool);
    ($e:expr, $b:expr, $w:expr) => {
        match $e {
            Ok(code) => {
                match code {
                    0 => {}, // no error occur, do nothing
                    1 => {
                        // TODO
                        return;
                    },
                    2 => {
                        // If handling emu simulation,  
                        // returning exit code 3 means emu abort at somewhere.  
                        // So the thread should return remaining the output  
                        // for the debuging
                        if $b {
                            log::error!("An EMU Simulation Abort!");
                        } else {
                            todo!()
                        }
                        return;
                    },
                    3 => {
                        // If handling emu simulation,    
                        // returning exit code 3  
                        // means that emu run at limited instruction,    
                        // and all is well.  
                        // So we should no nothing
                        // If not handling emu simualtion, the thread should end.  
                        if !$b {
                            return;
                        } else {
                            log::error!("An EMU Simulation Finished!");
                            if matches!(fs::remove_dir_all(&$w), Err(_)) {
                                log::error!("Remove Workload Failed!");
                            }
                        }
                    }, 
                    _ => {
                        panic!("Unhandle Exit Code");
                    }
                }
            },
            // Error code:
            // + no error -> 0
            // + set workdir error -> 1
            // + create stdout file error -> 2
            // + create stderr file error -> 3
            // + execute without exit code -> 4
            // + execute return error -> 5
            // + unallowed error -> -1
            // + (simulation only)no cpus avaiable -> -2
            Err(code) => {
                match code {
                    -1 => {
                        if matches!(fs::remove_dir_all(&$w), Err(_)) {
                            log::error!("Remove Workload Failed!");
                        }
                        panic!("Unallowed Error Occur!");
                    },
                    -2 => {
                        log::info!("Wait 10 minutes for avaiable cpus");
                        thread::sleep(Duration::from_secs(600));
                        if matches!(fs::remove_dir_all(&$w), Err(_)) {
                            log::error!("Remove Workload Failed!");
                        }
                        return;
                    },
                    0 => {}, // no error, do nothing
                    1 => {
                        log::error!("Set WorkDir Error");
                        if matches!(fs::remove_dir_all(&$w), Err(_)) {
                            log::error!("Remove Workload Failed!");
                        }
                        return;
                    },
                    2 => {
                        log::error!("Create Stdout File Error");
                        if matches!(fs::remove_dir_all(&$w), Err(_)) {
                            log::error!("Remove Workload Failed!");
                        }
                        return;
                    },
                    3 => {
                        log::error!("Create Stderr File Error");
                        if matches!(fs::remove_dir_all(&$w), Err(_)) {
                            log::error!("Remove Workload Failed!");
                        }
                        return;
                    },
                    4 => {
                        log::error!("Execute Without Exit Code");
                        return;
                    },
                    5 => {
                        log::error!("Execute Return Error");
                        return;
                    },
                    _ => {
                        if matches!(fs::remove_dir_all(&$w), Err(_)) {
                            log::error!("Remove Workload Failed!");
                        }
                        panic!("Unhandle Error Code");
                    }
                }
            }
        }
    };
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
    assert_eq!(conf.img_list(), Some("/bigdata/zyy/checkpoints_profiles/betapoint_profile_06"));
    assert_eq!(conf.thread_num(), Some(8));
    assert_eq!(conf.max_instr(), Some(1000000));
    assert_eq!(conf.nemu_home(), Some("/home/ccc/NEMU"));
    assert_eq!(conf.am_home(), Some("/home/ccc/nexus-am"));
}