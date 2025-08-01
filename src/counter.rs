use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

pub struct BroadcastCounter {
    counter: Arc<AtomicUsize>,
}

impl BroadcastCounter {
    /// Creates a new `BroadcastCounter` with an initial value of 0.
    pub(crate) fn new() -> Self {
        BroadcastCounter {
            counter: Arc::new(AtomicUsize::new(0)),
        }
    }

    /// Increments the counter by a specified value.
    /// This is used by the producer to indicate how many consumers are expected to
    /// process a broadcast message.
    pub(crate) fn increment_by(&self, val: usize) {
        self.counter.fetch_add(val, Ordering::SeqCst);
    }

    /// Decrements the counter by 1.
    /// This is used by each consumer after processing a message.
    pub(crate) fn decrement(&self) {
        self.counter.fetch_sub(1, Ordering::SeqCst);
    }

    /// Checks if the counter is zero, indicating all outstanding messages have been processed.
    pub(crate) fn is_empty(&self) -> bool {
        self.counter.load(Ordering::SeqCst) == 0
    }
}
