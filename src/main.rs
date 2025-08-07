use crate::crossbeam_one_per_one::crossbeam_broadcast_bench;
use crate::flume_one_per_one::flume_broadcast_bench;
use crate::glommio_broadcast_one_per_one::glommio_broadcast_bench;
use crate::glommio_one_per_one::glommio_bench;
use crate::glommio_shared_channels::glommio_shared_channels_bench;
use crate::kanal_one_per_one::kanal_broadcast_bench;
use crate::kanal_two_shared_channels::kanal_two_shared_channels_bench;
use crate::tokio_mpsc_one_per_one::tokio_mpsc_bench;
use crate::tokio_one_per_one::tokio_bench;
use glommio::LocalExecutor;

mod counter;
mod crossbeam_one_per_one;
mod flume_one_per_one;
mod glommio_broadcast_one_per_one;
mod glommio_one_per_one;
mod glommio_shared_channels;
mod glommio_two_shared_channels;
mod kanal_one_per_one;
mod kanal_two_shared_channels;
mod tokio_mpsc_one_per_one;
mod tokio_one_per_one;

macro_rules! time_it {
    ($name:expr, $expr:expr, $iterations:expr) => {{
        let start = std::time::Instant::now();
        let result = $expr;
        let elapsed = start.elapsed();
        println!(
            "{} - Time elapsed: {:?}, per item (avg): {:?}",
            $name,
            elapsed,
            elapsed / $iterations
        );
        result
    }};
}

#[tokio::main]
async fn main() {
    time_it!("Tokio Broadcast", tokio_bench(1000).await, 1000);
    time_it!("Tokio MPSC", tokio_mpsc_bench(1000).await, 1000);
    time_it!("Crossbeam", crossbeam_broadcast_bench(1000, 1), 1000);
    time_it!("Flume", flume_broadcast_bench(1000, 1), 1000);
    time_it!("Kanal", kanal_broadcast_bench(1000, 1), 1000);
    time_it!("Kanal 2 RX", kanal_two_shared_channels_bench(1000), 1000);

    let ex = LocalExecutor::default();
    ex.run(async {
        time_it!("Glommio", glommio_bench(1000).await, 1000);
    });

    let ex = LocalExecutor::default();
    ex.run(async {
        time_it!(
            "Glommio broadcast",
            glommio_broadcast_bench(1000, 1).await,
            1000
        );
    });
    let ex = LocalExecutor::default();
    ex.run(async {
        time_it!(
            "Glommio shared channels",
            glommio_shared_channels_bench(1000).await,
            1000
        );
    });

    // let ex = LocalExecutor::default();
    // ex.run(async {
    //     time_it!(
    //         "Glommio two shared channels",
    //         glommio_two_shared_channels_bench(100000).await,
    //         1000
    //     );
    // });
}
