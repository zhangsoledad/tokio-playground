use std::{
    thread,
    time::{self, Instant},
};
use tokio::runtime::Builder;
use tokio::task::spawn_blocking;

fn main() {
    let mut rt = Builder::new()
        .threaded_scheduler()
        .enable_all()
        .core_threads(4)
        .build()
        .unwrap();

    rt.block_on(async {
        let start = Instant::now();
        let mut tasks = Vec::new();

        for _ in 0..40 {
            // Spawn a task fetching the list.
            tasks.push(spawn_blocking(|| {
                thread::sleep(time::Duration::from_secs(1));
            }))
        }

        // Wait for all tasks to complete.
        for t in tasks {
            t.await;
        }

        dbg!(start.elapsed());
    });
}
