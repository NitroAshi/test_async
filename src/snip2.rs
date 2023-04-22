use tokio::sync::mpsc;

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

    pub async fn async_process_log_buf(&self, filename: &str) {
        let reader = read_file(filename).await.unwrap();

        let (tx, mut rx) = mpsc::channel::<String>(1024);

        let mut buf = String::new();
        loop {
            let n = reader.read_line(&mut buf).await.unwrap();
            if n == 0 {
                break;
            }
            let line = buf.clone();
            buf.clear();
            tx.send(line).await.unwrap();
        }
        drop(tx);

        while let Some(line) = rx.recv().await {
            self.notify(&line);
        }
    }
}

