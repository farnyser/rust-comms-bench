use futures::{select, FutureExt, StreamExt};
use glommio::channels::shared_channel::{ConnectedReceiver, ConnectedSender};
use glommio::{LocalExecutorBuilder, Placement};

/*

async fn consumer_task(mut rx1: ConnectedReceiver<u64>, mut rx2: ConnectedReceiver<u64>) {
    loop {
        let msg = poll_fn(|cx| {
            match rx1.poll_next_unpin(cx) {
                Poll::Ready(Some(val)) => return Poll::Ready(Some(("rx1", val))),
                Poll::Ready(None) => return Poll::Ready(None),
                Poll::Pending => {}
            }
            match rx2.poll_next_unpin(cx) {
                Poll::Ready(Some(val)) => return Poll::Ready(Some(("rx2", val))),
                Poll::Ready(None) => return Poll::Ready(None),
                Poll::Pending => {}
            }
            Poll::Pending
        }).await;

        match msg {
            Some((src, val)) => {
                println!("Got {} from {}", val, src);
            }
            None => {
                break;
            }
        }
    }
}
*/

async fn consumer_task(rx1: ConnectedReceiver<u64>, rx2: ConnectedReceiver<u64>) {
    let mut rx1_fut = Box::pin(rx1.recv()).fuse();
    let mut rx2_fut = Box::pin(rx2.recv()).fuse();

    loop {
        let d = select! {
            e = rx1_fut => {
                rx1_fut = Box::pin(rx1.recv()).fuse();
                e
            },
            e = rx2_fut => {
                rx2_fut = Box::pin(rx2.recv()).fuse();
                e
            },
        };

        match d {
            Some(_) => {}
            None => {
                break;
            }
        }
    }
}

async fn producer_task(tx1: ConnectedSender<u64>, tx2: ConnectedSender<u64>, length: u32) {
    for msg_i in 0..length {
        if msg_i % 4 == 0 {
            tx1.send(msg_i as u64).await.unwrap();
        } else {
            tx2.send(msg_i as u64).await.unwrap();
        }
    }
}

pub(crate) async fn glommio_two_shared_channels_bench(length: u32) {
    let mut consumer_join_handles = Vec::new();
    let (tx1, rx1) = glommio::channels::shared_channel::new_bounded(1);
    let (tx2, rx2) = glommio::channels::shared_channel::new_bounded(1);

    let join_handle = std::thread::spawn(move || {
        let ex = LocalExecutorBuilder::new(Placement::Unbound)
            .spawn(move || async move {
                let rx1 = rx1.connect().await;
                let rx2 = rx2.connect().await;
                consumer_task(rx1, rx2).await
            })
            .unwrap();

        ex.join().unwrap();
    });

    consumer_join_handles.push(join_handle);

    let tx1 = tx1.connect().await;
    let tx2 = tx2.connect().await;
    producer_task(tx1, tx2, length).await;

    for handle in consumer_join_handles {
        handle.join().expect("Failed to join consumer thread");
    }
}
