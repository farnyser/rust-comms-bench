use std::time::Duration;

pub(crate) async fn tokio_consumer(mut rx: tokio::sync::broadcast::Receiver<()>) {
    loop {
        match rx.recv().await {
            Ok(_) => {}
            Err(_) => break,
        };
    }
}

pub(crate) async fn tokio_bench(length: u32) {
    let (tx, rx) = tokio::sync::broadcast::channel(100);
    tokio::spawn(tokio_consumer(rx));

    for _ in 0..length {
        while !tx.is_empty() {
            tokio::time::sleep(Duration::from_micros(1)).await;
        }

        tx.send(()).unwrap();
    }
}
