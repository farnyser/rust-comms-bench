use glommio::channels::shared_channel::{ConnectedReceiver, ConnectedSender};
use glommio::{LocalExecutorBuilder, Placement};

async fn consumer_task(rx: ConnectedReceiver<u64>) {
    loop {
        match rx.recv().await {
            Some(_) => {}
            None => {
                break;
            }
        }
    }
}

async fn producer_task(tx: ConnectedSender<u64>, length: u32) {
    for msg_i in 0..length {
        tx.send(msg_i as u64).await.unwrap();
    }
}

pub(crate) async fn glommio_shared_channels_bench(length: u32) {
    let mut consumer_join_handles = Vec::new();
    let (tx, rx) = glommio::channels::shared_channel::new_bounded(1);

    let join_handle = std::thread::spawn(move || {
        let ex = LocalExecutorBuilder::new(Placement::Unbound)
            .spawn(move || async move {
                let rx = rx.connect().await;
                consumer_task(rx).await
            })
            .unwrap();

        ex.join().unwrap();
    });

    consumer_join_handles.push(join_handle);

    let tx = tx.connect().await;
    producer_task(tx, length).await;

    for handle in consumer_join_handles {
        handle.join().expect("Failed to join consumer thread");
    }
}
