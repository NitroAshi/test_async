mod args;
mod log_path;
mod parser_async;

use std::fs;
use tokio::fs::File;
use tokio::io::{self, AsyncBufReadExt};
use tokio::sync::mpsc;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    
    let paths = args::input();
    let logs_pool = log_path::find_logs(paths);

    let results = parser_async::process_logs(logs_pool).await;
    for result in results {
        println!("{}", result);
    }

    Ok(())
    
}
