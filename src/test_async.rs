use tokio::fs::File;
use tokio::io::{self, AsyncBufReadExt, BufReader};
use tokio::io::AsyncBufRead;
use tokio::sync::mpsc;
//use flate2::read::GzDecoder;
use std::pin::Pin;
use async_compression::tokio::bufread::GzipDecoder;
// use async_compression::futures::bufread::GzipDecoder;

pub async fn read_file(filename: &str) -> Result<Pin<Box<dyn AsyncBufRead + Send + 'static>>, std::io::Error> {
    let file = File::open(filename).await?;
    let reader: Pin<Box<dyn AsyncBufRead + Send + 'static>> = match filename.ends_with(".out.gz") {
        false => Box::pin(BufReader::new(file)),
        true => Box::pin(BufReader::new(GzipDecoder::new(BufReader::new(file)))),
    };
    return Ok(reader);
}

pub async fn read_plain_file(filename: &str) -> Result<BufReader<File>, io::Error> {
    let file = File::open(filename).await?;
    let reader = tokio::io::BufReader::new(file);
    Ok(reader)
}

pub async fn read_gz_file(filename: &str) -> Result<BufReader<GzipDecoder<BufReader<File>>>, io::Error> {
    let file = File::open(filename).await?;
    let reader = BufReader::new(GzipDecoder::new(BufReader::new(file)));
    Ok(reader)
}


pub async fn process_logs(paths: Vec<String>) -> Vec<String> {
    let (tx, mut rx) = mpsc::channel(100);
    for path in paths {
        let tx = tx.clone();
        tokio::spawn(async move {
            let reader = read_file(&path);

            let mut lines = &reader.lines();
            while let Some(line) = lines.next_line().await.unwrap() {
                if line.starts_with("PV-INFO: siteid") {
                    tx.send(format!("{}", line)).await.unwrap();
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
