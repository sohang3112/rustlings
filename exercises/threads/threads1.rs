// threads1.rs
// Make this compile! Execute `rustlings hint threads1` for hints :)
// The idea is the thread spawned on line 21 is completing jobs while the main thread is
// monitoring progress until 10 jobs are completed. If you see 6 lines
// of "waiting..." and the program ends without timing out when running,
// you've got it :)

use std::sync::{Arc, RwLock};
use std::thread;
use std::time::Duration;

struct JobStatus {
    jobs_completed: u32,
}

fn main() {
    // Arc (Atomic Reference Count) is needed for shared ownership across threads
    // RwLock is needed so that we can have multiple readers, 1 writer
    let status = Arc::new(RwLock::new(JobStatus { jobs_completed: 0 }));
    
    // if we didn't clone and used status directly, 
    // then status would be moved into the thread closure
    // so main thread would not be able to access status.
    let status_shared = status.clone();

    thread::spawn(move || {
        // get a mutable write lock to status
        let mut status = status_shared.write().unwrap();
        for _ in 0..10 {
            thread::sleep(Duration::from_millis(250));
            status.jobs_completed += 1;
        }
    });

    // in each iteration, acquire read lock to status
    // and check its value is < 10
    // NOTE: we CAN'T do status.read().unwrap() outside loop
    // because status is being updated in another thread, 
    // so we need to keep re-checking its value.
    while status.read().unwrap().jobs_completed < 10 {
        println!("waiting... ");
        thread::sleep(Duration::from_millis(500));
    }
}
