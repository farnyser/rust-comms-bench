use kanal::{Receiver, Sender};

fn consumer_task(rx1: Receiver<u64>, rx2: Receiver<u64>) {
    loop {
        match rx1.try_recv() {
            Ok(_) => {}
            Err(_) => {
                break;
            }
        }
        match rx2.try_recv() {
            Ok(_) => {}
            Err(_) => {
                break;
            }
        }
    }
}

fn producer_task(tx1: Sender<u64>, tx2: Sender<u64>, length: u32) {
    for msg_i in 0..length {
        if msg_i % 4 == 0 {
            tx1.send(msg_i as u64).unwrap();
        } else {
            tx2.send(msg_i as u64).unwrap();
        }
    }
}

pub(crate) fn kanal_two_shared_channels_bench(length: u32) {
    let mut consumer_join_handles = Vec::new();
    let (tx1, rx1) = kanal::bounded(1);
    let (tx2, rx2) = kanal::bounded(1);

    let join_handle = std::thread::spawn(move || consumer_task(rx1, rx2));

    consumer_join_handles.push(join_handle);

    producer_task(tx1, tx2, length);

    for handle in consumer_join_handles {
        handle.join().expect("Failed to join consumer thread");
    }
}
