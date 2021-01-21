/// Config for RUST-XS-TEST

use serde_derive::Deserialize;
use std::fs;
use std::path::Path;
/// Global Config
#[derive(Debug, Deserialize)]
pub struct Config {
    hook: Option<HookConfig>,
    emu: Option<EmuConfig>,
}

#[derive(Debug, Deserialize)]
pub struct HookConfig {
    workers_num: Option<usize>,
    work_root: Option<String>,
    sleep_time: Option<u64>,
}

#[derive(Debug, Deserialize)]
pub struct EmuConfig {
    img: Option<String>,
    thread_num: Option<usize>,
    noop_home: Option<String>,
    nemu_home: Option<String>,
    am_home: Option<String>,
}


impl Config {
    pub fn new<P: AsRef<Path>>(f: P) -> Self {
        let toml_f = fs::read_to_string(f).unwrap();
        let config: Config = toml::from_str(toml_f.as_str()).unwrap();
        config
    }

    pub fn workers_num(&self) -> Option<usize> {
        if let Some(config) = &self.hook {
            config.workers_num
        } else {
            None
        }
    }

    pub fn work_root(&self) -> Option<&str> {
        if let Some(config) = &self.hook {
            if let Some(string) = &config.work_root {
                Some(string.as_str())
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn sleep_time(&self) -> Option<u64> {
        if let Some(config) = &self.hook {
            config.sleep_time
        } else {
            None
        }
    }

    pub fn img(&self) -> Option<&str> {
        if let Some(config) = &self.emu {
            if let Some(string) = &config.img {
                Some(string.as_str())
            } else {
                None
            }
        } else {
            None
        }
    }
    
    pub fn thread_num(&self) -> Option<usize> {
        if let Some(config) = &self.emu {
            config.thread_num
        } else {
            None
        }
    }

    pub fn noop_home(&self) -> Option<&str> {
        if let Some(config) = &self.emu {
            if let Some(string) = &config.noop_home {
                Some(string.as_str())
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn nemu_home(&self) -> Option<&str> {
        if let Some(config) = &self.emu {
            if let Some(string) = &config.nemu_home {
                Some(string.as_str())
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn am_home(&self) -> Option<&str> {
        if let Some(config) = &self.emu {
            if let Some(string) = &config.am_home {
                Some(string.as_str())
            } else {
                None
            }
        } else {
            None
        }
    }
}

