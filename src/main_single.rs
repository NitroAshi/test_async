mod args;
mod log_path;
//mod parser;

use std::fs;
use tokio::fs::File;
use tokio::io::{self, AsyncBufReadExt};
use tokio::sync::mpsc;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    
    let paths = args::input();
    let logs_pool = log_path::find_logs(paths);

    let (tx, mut rx) = mpsc::channel(1000);
    for log in logs_pool {
        let tx = tx.clone();
        tokio::spawn(async move {
            let file = File::open(log).await?;
            let reader = io::BufReader::new(file);
            let mut lines = reader.lines();
            while let Some(line) = lines.next_line().await? {
                if line.contains("cpu-info") {
                    tx.send(format!("{}", line)).await.unwrap();
                    break;
                }
            }
            //Ok(())
            Ok::<(), io::Error>(())
        });
    }

    drop(tx);

    while let Some(result) = rx.recv().await {
        println!("{}", result);
    }

    Ok(())



    
}
