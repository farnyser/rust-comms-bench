use crate::counter::BroadcastCounter;
use crossbeam_channel::{unbounded, Receiver, Sender};
use num_cpus;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

fn consumer_task(rx: Receiver<u64>, counter: Arc<BroadcastCounter>) {
    loop {
        match rx.recv() {
            Ok(_) => {
                counter.decrement();
            }
            Err(_) => {
                break;
            }
        }
    }
}

fn producer_task(
    tx_channels: Vec<Sender<u64>>,
    counter: Arc<BroadcastCounter>,
    length: u32,
    num_consumers: usize,
) {
    for i in 0..length {
        while !counter.is_empty() {
            thread::sleep(Duration::from_micros(1));
        }

        counter.increment_by(num_consumers);

        for tx in &tx_channels {
            tx.send(i as u64).unwrap();
        }
    }

    while !counter.is_empty() {
        thread::sleep(Duration::from_micros(1));
    }
}

pub(crate) fn crossbeam_broadcast_bench(length: u32, num_consumers: usize) {
    let counter = Arc::new(BroadcastCounter::new());

    let mut producer_senders: Vec<Sender<u64>> = Vec::with_capacity(num_consumers);
    let mut consumer_join_handles = Vec::new();

    let available_cores = num_cpus::get();
    let actual_consumer_threads = std::cmp::min(num_consumers, available_cores.saturating_sub(1));

    for _ in 0..actual_consumer_threads {
        let (tx, rx) = unbounded();
        producer_senders.push(tx);

        let consumer_counter = counter.clone();

        let join_handle = thread::spawn(move || {
            consumer_task(rx, consumer_counter);
        });

        consumer_join_handles.push(join_handle);
    }

    producer_task(producer_senders, counter, length, actual_consumer_threads);

    for handle in consumer_join_handles {
        handle.join().expect("Failed to join consumer thread");
    }
}
