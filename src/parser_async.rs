use tokio::fs::File;
use tokio::io::{self, AsyncBufRead, AsyncBufReadExt, AsyncReadExt, BufReader};
use tokio::sync::mpsc;
//use flate2::read::GzDecoder;
use async_compression::tokio::bufread::GzipDecoder;
use std::pin::Pin;
use async_trait::async_trait;

pub async fn read_file(filename: &str) -> Result<Pin<Box<dyn AsyncBufRead + Send>>, std::io::Error> {
    let file = File::open(filename).await?;
    let reader: Pin<Box<dyn tokio::io::AsyncBufRead + Send>> = match filename.ends_with(".out.gz") {
        false => Box::pin(BufReader::new(file)),
        true => Box::pin(BufReader::new(GzipDecoder::new(BufReader::new(file)))),
    };
    return Ok(reader);
}

pub async fn process_logs(paths: Vec<String>) -> Vec<String> {
    let (tx, mut rx) = mpsc::channel(100);
    for path in paths {
        let tx = tx.clone();
        tokio::spawn(async move {
            let reader = read_file(&path).await.unwrap();

            // iterator
            let mut lines = reader.lines();
            while let Some(line) = lines.next_line().await.unwrap() {
                //tx.send(format!("{}", line)).await.unwrap();
                if line.starts_with("cpu-info") {
                    tx.send(format!("{}", &line)).await.unwrap();
                    break;
                }
            }

        });
    }

    drop(tx);

    let mut results = vec![];
    while let Some(result) = rx.recv().await {
        results.push(result);
    }

    results

}

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

    async fn notify(&self, line: &str, tx: mpsc::Sender<String>) {
        for observer in self.observers.iter() {
            observer.update(line, tx);
        }
    }

    pub async fn process_log(&self, filename: &str, tx: mpsc::Sender<String>) {
        let reader = read_file(filename).await.unwrap();
        // iterator
        let mut lines = reader.lines();
        while let Some(line) = lines.next_line().await.unwrap() {
            self.notify(&line, tx);            
            //tx.send(format!("{}", &line)).await.unwrap();                
            //if line.starts_with("cpu-info") {
            //    tx.send(format!("{}", &line)).await.unwrap();
            //    break;
            //}
        }
    }
}



#[async_trait]
pub trait LogObserver {
    async fn update(&self, line: &str, tx: mpsc::Sender<String>);
}

// Observers
//
// RunningInfo
pub struct RunningInfo {
    //max_memory: u32,
    //elapsed_time: u32,
}

#[async_trait]
impl LogObserver for RunningInfo {
    async fn update(&self, line: &str, tx: mpsc::Sender<String>) {
        if line.starts_with("Maximum memory usage for this session including child processes:") {
            if let Some(result) = line.split_whitespace().rev().nth(1) {
                let result = result.parse::<f32>().unwrap() as u32;
                //println!("max_memory -> {}", result);
                let message = format!("max_memory -> {}", result);
                tx.send(message).await.unwrap();
            }
        }
        if line.starts_with("Elapsed time for this session:") {
            if let Some(result) = line.split_whitespace().rev().nth(4) {
                let result = result.parse::<u32>().unwrap();
                //println!("elapsed_time -> {}", result);
                let message = format!("elapsed_time -> {}", result);
                tx.send(message).await.unwrap();
            }
        }
    }
}



