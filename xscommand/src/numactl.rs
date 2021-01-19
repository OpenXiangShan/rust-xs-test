/// Numactl XSCommand Implementation
/// 
extern crate psutil;

use super::{XSCommand, XSCommandErr, DefaultErr};
use std::{fs::File,
    os::unix::io::{FromRawFd, IntoRawFd},
    process::{Command, Stdio, exit},
    thread,
    time::Duration,
    vec,
};
use xscommand_macros::XSCommand;

#[derive(XSCommand)]
pub struct Numactl<'a> {
    pub exe: Command,
    args: Vec<&'a str>,
    work_dir: Option<&'a str>,
}

impl<'a> Numactl<'a> {
    pub fn make_emu(workload: Option<&str>, nemu_home: &str, am_home: &str, thread_num: usize) -> Result<i32, i32> {
        let mut numactl = Numactl::new("numactl");
        let thread_num = thread_num.to_string();
        let mut thread_num_arg =  "EMU_THREADS=".to_string();
        thread_num_arg.push_str(thread_num.as_str());
        numactl.set_args(vec!["-C", "0-255", "make", "build/emu", "EMU_TARCE=1", "SIM_ARGS=\"--disable-all\"", thread_num_arg.as_str(), "-j256"]);
        if let Err(err) = numactl.set_workdir(workload) {
            return Err(err.err_code());
        };
        numactl.exe.env("NEMU_HOME", nemu_home);
        numactl.exe.env("AM_HOME", am_home);
        match numactl.excute(None, None) {
            Ok(exit_code) => Ok(exit_code),
            Err(err) => Err(err.err_code()),
        }
    }
    pub fn run_emu(
        workload: Option<&str>,
        stdout: Option<&str>,
        stderr: Option<&str>,
        emu_path: &str,
        img_path: &str,
        nemu_home: &str,
        am_home: &str,
        thread_num: usize) -> Result<i32, i32> {
        let mut numactl = Numactl::new("numactl");
        let mut cpu_percent_collector = match psutil::cpu::CpuPercentCollector::new() {
            Ok(collector) => collector,
            Err(_) => {
                // TODO: specify exit code
                let exit_code = -1;
                log::error!("new CpuPercentCollector error, exit with {}", exit_code);
                exit(exit_code);
            }
        };
        let cpu_percents = match cpu_percent_collector.cpu_percent_percpu() {
            Ok(percents) => percents,
            Err(_) => {
                // TODO: specify exit code
                let exit_code = -1;
                log::error!("get cpu_percent_percpu error, exit with {}.", exit_code);
                exit(exit_code);
            }
        };
        let (mut begin, mut end) = (0, 0);
        let mut count = 0;
        for _ in 0..3 {
            begin = 0;
            end = 0;
            for i in 0..cpu_percents.len() {
                if cpu_percents[i] > 50.0 {
                    begin += 1;
                    end = begin;
                } else {
                    end += 1;
                }
                if (end - begin) >= (thread_num - 1) {
                    // found 8 free cpus, break
                    break;
                }
            }
            if (end - begin) >= (thread_num - 1) {
                log::info!("found avaiable 8 cpus, ready to run emu.");
                break;
            }
            count += 1;
            thread::sleep(Duration::from_secs(120));
        }
        if count >= 3 {
            // TODO: specify exit code
            let exit_code = -1;
            log::error!("no {} cpus avaiable, exit with {}.", thread_num, exit_code);
            exit(exit_code);
        }
        let mut cpu_ids = begin.to_string();
        cpu_ids.push_str("-");
        cpu_ids.push_str(end.to_string().as_str());
        log::info!("avaiable cpus: {}", cpu_ids);
        numactl.set_args(vec!["-C", cpu_ids.as_str(), emu_path, "-I", "1000000", "-i", img_path]);
        if let Err(err) = numactl.set_workdir(workload) {
            return Err(err.err_code());
        };
        if let Some(noop_home) = workload {
            numactl.exe.env("NOOP_HOME", noop_home);    
        } else {
            // TODO: specify exit code
            let exit_code = -1;
            log::error!("workload.to_str() return None, exit with {}.", exit_code);
            exit(exit_code);
        }
        numactl.exe.env("NEMU_HOME", nemu_home);
        numactl.exe.env("AM_HOME", am_home);
        match numactl.excute(stdout, stderr) {
            Ok(exit_code) => Ok(exit_code),
            Err(err) => Err(err.err_code()),
        }
    }
}
