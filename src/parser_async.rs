// use std::fs;
// use std::fs::File;
// use std::io::BufRead;
// use std::io::BufReader;
use tokio::fs::File;
use tokio::io::{self, AsyncBufReadExt, BufReader};
// use tokio::io::{self, AsyncBufReadExt};
// use futures::{
//     io::{self, BufReader, ErrorKind},
//     prelude::*,
// };
use tokio::sync::mpsc;
use flate2::read::GzDecoder;
use std::pin::Pin;
use async_compression::tokio::bufread::GzipDecoder;
// use async_compression::futures::bufread::GzipDecoder;


// pub async fn read_file(filename: &str) -> Result<Pin<Box<dyn tokio::io::AsyncBufRead + Send>>, std::io::Error> {
//     let file = File::open(filename).await?;
//     let reader: Pin<Box<dyn tokio::io::AsyncBufRead + Send>> = match filename.ends_with(".out.gz") {
//         false => Box::pin(BufReader::new(file)),
//         true => Box::pin(BufReader::new(GzipDecoder::new(file))),
//     };
//     return Ok(reader);
// }

// pub fn read_file(filename: &str) -> Result<Box<dyn std::io::BufRead + Send>, std::io::Error> {
//     let file = std::fs::File::open(filename)?;
//     let reader: Box<dyn std::io::BufRead> = match filename.ends_with(".out.gz") {
//         false => Box::new(std::io::BufReader::new(file)),
//         true => Box::new(std::io::BufReader::new(GzDecoder::new(file))),
//     };
//     return Ok(reader);
// }


// pub async fn read_file(filename: &str) -> Result<BufReader<File>, io::Error> {
//     let file = File::open(filename).await?;
//     let reader = tokio::io::BufReader::new(file);
//     Ok(reader)
// }

pub async fn read_gz_file(filename: &str) -> Result<BufReader<GzipDecoder<BufReader<File>>>, io::Error> {
    let file = File::open(filename).await?;
    // let reader = GzipDecoder::new(file);
    let reader = BufReader::new(GzipDecoder::new(BufReader::new(file)));
    // let reader = GzipDecoder::new(file);
    Ok(reader)
}



pub async fn process_logs(paths: Vec<String>) -> Vec<String> {
    let (tx, mut rx) = mpsc::channel(100);
    for path in paths {
        let tx = tx.clone();
        tokio::spawn(async move {
            // let file = File::open(path).await.unwrap();
            // let reader = io::BufReader::new(file);
            let reader = read_gz_file(&path).await.unwrap();
            // for line in reader.lines() {
            //     if let Ok(line) = line {
            //         if line.starts_with("cpu-info") {
            //             tx.send(format!("{}", line)).await.unwrap();
            //             break;
            //         }
            //     }
            // }


            let mut lines = reader.lines();
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
