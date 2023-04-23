mod args;
mod log_path;
mod parser_async;

//use std::fs;
use tokio::fs::File;
use tokio::io::{self, AsyncBufReadExt};
use tokio::sync::mpsc;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    
    let paths = args::input();
    let logs_pool = log_path::find_logs(paths);

    let mut subject = parser_async::LogSubject::new();
    let running_observer = parser_async::RunningInfo {};
    subject.attach(Box::new(running_observer));

    let (tx, mut rx) = mpsc::channel(100);

    for log in logs_pool {
        let tx = tx.clone();
        tokio::spawn( async move {
            subject.process_log(&log);
        });
    }

    //let results = parser_async::process_logs(logs_pool).await;
    //for result in results {
    //    println!("{}", result);
    //}

    Ok(())
    
}
