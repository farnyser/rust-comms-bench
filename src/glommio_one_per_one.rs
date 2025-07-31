use glommio::channels::shared_channel::SharedReceiver;
use glommio::spawn_local;
use glommio::timer::sleep;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::time::Duration;

pub(crate) async fn glommio_consumer(rx: SharedReceiver<()>, counter: Arc<AtomicUsize>) {
    let rx = rx.connect().await;
    while rx.recv().await.is_some() {
        counter.fetch_sub(1, std::sync::atomic::Ordering::SeqCst);
    }
}

pub(crate) async fn glommio_bench(length: u32) {
    let (tx, rx) = glommio::channels::shared_channel::new_bounded(100);
    let counter = Arc::new(AtomicUsize::new(0));

    let consumer_counter = counter.clone();
    spawn_local(async move {
        glommio_consumer(rx, consumer_counter).await;
    })
    .detach();

    let tx = tx.connect().await;

    for _ in 0..length {
        while counter.load(Ordering::SeqCst) > 0 {
            sleep(Duration::from_micros(1)).await;
        }

        counter.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        tx.send(()).await.unwrap();
    }
}
