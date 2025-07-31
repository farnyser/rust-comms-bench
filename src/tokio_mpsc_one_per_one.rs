use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::time::Duration;

pub(crate) async fn tokio_mpsc_consumer(
    mut rx: tokio::sync::mpsc::Receiver<()>,
    counter: Arc<AtomicUsize>,
) {
    loop {
        match rx.recv().await {
            Some(_) => {
                counter.fetch_sub(1, std::sync::atomic::Ordering::SeqCst);
            }
            None => break,
        };
    }
}

pub(crate) async fn tokio_mpsc_bench(length: u32) {
    let (tx, rx) = tokio::sync::mpsc::channel(100);
    let counter = Arc::new(AtomicUsize::new(0));

    let consumer_counter = counter.clone();
    tokio::spawn(tokio_mpsc_consumer(rx, consumer_counter));

    for _ in 0..length {
        while counter.load(Ordering::SeqCst) > 0 {
            tokio::time::sleep(Duration::from_micros(1)).await;
        }

        tx.send(()).await.unwrap();
        counter.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    }
}
