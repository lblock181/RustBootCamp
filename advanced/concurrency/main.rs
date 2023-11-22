use std::{thread, time::Duration};

fn main() {

    // CREATING THREADS

    let handle = thread::spawn(|| {
        for i in 0..20 {
            println!("Spawned thread {}", {i});
            // forcing thread to sleep
            thread::sleep(Duration::from_millis(1));
        }
    });
    
    // w/o join handle, main thread will complete and exit before spawned threads are done
    for i in 0..10 {
        println!("Main thread {i}");
        thread::sleep(Duration::from_millis(1));
    }

    // by joining, force main thread to wait until all handle threads are complete
    handle.join().unwrap();
}