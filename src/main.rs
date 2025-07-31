use crate::glommio_one_per_one::glommio_bench;
use crate::tokio_mpsc_one_per_one::tokio_mpsc_bench;
use crate::tokio_one_per_one::tokio_bench;
use glommio::LocalExecutor;

mod glommio_one_per_one;
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

    let ex = LocalExecutor::default();

    ex.run(async {
        time_it!("Glommio", glommio_bench(1000).await, 1000);
    });
}
