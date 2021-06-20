use std::{fs, path::PathBuf};

use crate::util::string;

pub fn read_dir(path: &str) -> (Vec<String>, Vec<String>) {
    let read_dir_result = fs::read_dir(path);

    let mut paths: Vec<PathBuf> = Vec::new();
    let mut dirs: Vec<String> = Vec::new();
    let mut files: Vec<String> = Vec::new();

    for e in read_dir_result {
        for ex in e {
            for dir_entry in ex {
                paths.push(dir_entry.path());
            }
        }
    }

    for path in paths {
        let str = string::cleanup_str(path.clone());
        if path.is_dir() {
            dirs.push(str);
        } else if path.is_file() {
            files.push(str);
        }
    }

    (dirs, files)
}
