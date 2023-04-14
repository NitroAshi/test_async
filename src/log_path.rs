use std::fs::{self, DirEntry};
use std::path::{Path, PathBuf};

//add struct LogPaths ???   need to be optimized

pub fn find_logs(paths: Vec<PathBuf>) -> Vec<String> {
    let mut logs = Vec::new();

    for path in paths {
        if path.is_file() {
            if let Some(file) = path.to_str() {
                if file.ends_with(".out") || file.ends_with(".out.gz") {
                    logs.push(file.to_string());
                }
            }
        } else if path.is_dir() {
            for entry in fs::read_dir(path).expect("read_dir failed") {
                if let Ok(entry) = entry {
                    if let Some(file) = entry.path().to_str() {
                        if file.ends_with(".out") || file.ends_with(".out.gz") {
                            logs.push(file.to_string());
                        }
                    }
                }
            }
        }
    }
    return logs;
}
