use crate::counter::BroadcastCounter;
use glommio::channels::channel_mesh::{Full, MeshBuilder, Receivers};
use glommio::timer::sleep;
use glommio::{LocalExecutorBuilder, Placement};
use num_cpus;
use std::sync::Arc;
use std::time::Duration;

async fn consumer_task(rx: Receivers<u64>, counter: Arc<BroadcastCounter>, producer_id: usize) {
    loop {
        match rx.recv_from(producer_id).await {
            Ok(Some(_)) => {
                counter.decrement();
            }
            Ok(None) => {
                break;
            }
            Err(_) => {
                break;
            }
        }
    }
}

async fn producer_task(mesh: MeshBuilder<u64, Full>, counter: Arc<BroadcastCounter>, length: u32) {
    let (tx, _) = mesh.join().await.unwrap();

    for msg_i in 0..length {
        while !counter.is_empty() {
            sleep(Duration::from_micros(1)).await;
        }

        counter.increment_by(tx.nr_consumers() - 1);

        for consumer_id in 1..tx.nr_consumers() {
            tx.send_to(consumer_id, msg_i as u64).await.unwrap();
        }
    }

    while !counter.is_empty() {
        sleep(Duration::from_micros(1)).await;
    }
}

pub(crate) async fn glommio_broadcast_bench(length: u32, num_consumers: usize) {
    let total_participants = 1 + num_consumers;
    let mesh = MeshBuilder::full(total_participants, 100);

    let available_cores = num_cpus::get();
    let actual_consumer_cores = std::cmp::min(num_consumers, available_cores.saturating_sub(1));

    let counter = Arc::new(BroadcastCounter::new());

    let mut consumer_join_handles = Vec::new();

    for consumer_id_idx in 0..actual_consumer_cores {
        let consumer_counter = counter.clone();
        let mesh_clone_for_consumer = mesh.clone();
        let current_consumer_expected_id = 1 + consumer_id_idx;

        let join_handle = std::thread::spawn(move || {
            let ex = LocalExecutorBuilder::new(Placement::Unbound)
                .name(&format!(
                    "consumer-executor-{}",
                    current_consumer_expected_id
                )) // Name for easier debugging
                .spawn(move || async move {
                    let (_, consumer_rx) = mesh_clone_for_consumer.join().await.unwrap();
                    consumer_task(consumer_rx, consumer_counter, 0).await
                })
                .unwrap();

            ex.join().unwrap();
        });

        consumer_join_handles.push(join_handle);
    }

    let producer_counter = counter.clone();
    let mesh_clone_for_producer = mesh.clone();
    producer_task(mesh_clone_for_producer, producer_counter, length).await;

    for handle in consumer_join_handles {
        handle.join().expect("Failed to join consumer thread");
    }
}
