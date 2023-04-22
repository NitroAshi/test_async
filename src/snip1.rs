pub async fn process_file(path: &str, tx: mpsc::Sender<String>) {
    let reader = read_file(path).await.unwrap();

    // iterator
    let mut lines = reader.lines();
    while let Some(line) = lines.next_line().await.unwrap() {
        if line.starts_with("cpu-info") {
            tx.send(format!("{}", &line)).await.unwrap();
            break;
        }
    }
}

for path in paths {
    let tx = tx.clone();
    tokio::spawn(async move {
        process_file(&path, tx).await;
    });
}

