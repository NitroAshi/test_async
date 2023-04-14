use flate2::read::GzDecoder;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
//use std::sync::{Arc, Mutex};
//use tokio::task::JoinHandle;
//use serde::{Serialize, Deserialize};
//use serde_json::{Result, Value};
//use std::io::Write;
//use encoding_rs::UTF_8;
//use encoding_rs_io::DecodeReaderBytesBuilder;

pub fn read_file(filename: &str) -> Result<Box<dyn BufRead>, std::io::Error> {
    let file = File::open(filename)?;
    let reader: Box<dyn BufRead> = match filename.ends_with(".gz") {
        false => Box::new(BufReader::new(file)),
        true => Box::new(BufReader::new(GzDecoder::new(file))),
    };
    //BufReader::new(file);
    //let reader = BufReader::new(DecodeReaderBytesBuilder::new().encoding(Some(UTF_8)).build(file));
    //return Ok(Box::new(reader));
    return Ok(reader);
    // Ok(Box::new(BufReader::new(File::open(filename)?)))
}

/*
pub fn find_keyword(filename: &str, keyword: &str) -> Result<Vec<String>, std::io::Error> {
    let reader = read_file(filename).unwrap();
    let mut lines_with_keyword = Vec::new();
    for line in reader.lines() {
        let line = line?;
        if line.contains(keyword) {
            lines_with_keyword.push(line);
        }
    }
    return Ok(lines_with_keyword);
}
*/

pub struct LogSubject {
    observers: Vec<Box<dyn LogObserver>>,
}

impl LogSubject {
    pub fn new() -> LogSubject {
        LogSubject {
            observers: Vec::new(),
        }
    }

    pub fn attach(&mut self, observer: Box<dyn LogObserver>) {
        self.observers.push(observer);
    }

    fn notify(&self, line: &str) {
        for observer in self.observers.iter() {
            observer.update(line);
        }
    }

    pub fn process_log_buf(&self, filename: &str) {
        let reader = read_file(filename).unwrap();

        for line in reader.lines() {
            if let Ok(line) = line {
                self.notify(&line);
            }
        }
    }
}

pub trait LogObserver {
    fn update(&self, line: &str);
}

pub struct RunningInfo {
    //max_memory: u32,
    //elapsed_time: u32,
}

impl LogObserver for RunningInfo {
    fn update(&self, line: &str) {
        if line.starts_with("Maximum memory usage for this session including child processes:") {
            if let Some(result) = line.split_whitespace().rev().nth(1) {
                let result = result.parse::<f32>().unwrap() as u32;
                println!("max_memory -> {}", result);
            }
        }
        if line.starts_with("Elapsed time for this session:") {
            if let Some(result) = line.split_whitespace().rev().nth(4) {
                let result = result.parse::<u32>().unwrap();
                println!("elapsed_time -> {}", result);
            }
        }
    }
}

pub struct UserInfo {
    //siteid: &'a str,
    //user: &'a str,
}

/*
impl UserInfo {
    pub fn new(siteid: &str, user: &str) -> Self {
        Self {
            siteid: siteid,
            user: user,
        }
    }
}
*/

impl LogObserver for UserInfo {
    fn update(&self, line: &str) {
        if line.starts_with("PV-INFO: siteid") {
            if let Some(result) = line.split_whitespace().last() {
                println!("siteid -> {}", result);
            }
        }
        if line.starts_with("PV-INFO: user") {
            if let Some(result) = line.split_whitespace().last() {
                println!("user -> {}", result);
            }
        }
    }
}
