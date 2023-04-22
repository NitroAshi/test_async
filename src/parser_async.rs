use tokio::fs::File;
use tokio::io::{AsyncBufRead, AsyncReadExt};
use tokio::io::{self, AsyncBufReadExt, BufReader};
use tokio::sync::mpsc;
use flate2::read::GzDecoder;
use std::pin::Pin;
use async_compression::tokio::bufread::GzipDecoder;


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
