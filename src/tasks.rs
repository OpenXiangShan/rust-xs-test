//! Get Tasks List from Specified Directory

use std::path::Path;
use walkdir::{WalkDir};

pub fn tasks_list<P: AsRef<Path>>(dir: P) -> Vec<String> {
    let mut tasks = Vec::new();
    for entry in WalkDir::new(dir) {
        let entry = entry.unwrap();
        if entry.file_name().to_str().map(|s| s.ends_with(".gz")).unwrap_or(false) {
            if let Some(str) = entry.file_name().to_str() {
                tasks.push(String::from(str));
            }
        }
    }
    tasks
}

#[test]
fn test_tasks_list() {
    let emu_tasks_dir = Path::new("/bigdata/zyy/checkpoints_profiles/betapoint_profile_06");
    let tasks = tasks_list(emu_tasks_dir);
    for task in tasks {
        println!("{}", task);
    }
}