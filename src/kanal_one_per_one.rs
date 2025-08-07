use kanal::{Receiver, Sender};
use std::thread;

fn consumer_task(rx: Receiver<u64>) {
    loop {
        match rx.recv() {
            Ok(_) => {}
            Err(_) => {
                break;
            }
        }
    }
}

fn producer_task(tx_channels: Vec<Sender<u64>>, length: u32) {
    for i in 0..length {
        for tx in &tx_channels {
            tx.send(i as u64).unwrap();
        }
    }
}

pub(crate) fn kanal_broadcast_bench(length: u32, num_consumers: usize) {
    let mut producer_senders: Vec<Sender<u64>> = Vec::with_capacity(num_consumers);
    let mut consumer_join_handles = Vec::new();

    let available_cores = num_cpus::get();
    let actual_consumer_threads = std::cmp::min(num_consumers, available_cores.saturating_sub(1));

    for _ in 0..actual_consumer_threads {
        let (tx, rx) = kanal::bounded(1);
        producer_senders.push(tx);

        let join_handle = thread::spawn(move || {
            consumer_task(rx);
        });

        consumer_join_handles.push(join_handle);
    }

    producer_task(producer_senders, length);

    for handle in consumer_join_handles {
        handle.join().expect("Failed to join consumer thread");
    }
}
