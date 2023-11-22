use std::{thread, time::Duration};
use std::sync::{mpsc, Mutex};
use::std::sync::Arc;

use tokio_stream::StreamExt;


fn main_async() {
    // typically used for I/O bound
    
    // to drive/call async functions, need runtime or executor
    //      eg Tokio
    #[tokio::main]
    async fn mf(i) {
        println!("async fn");
        let s = read_from_db().await
        println!("s {s}");
        let s2 = read_from_db().await;
        println!("s2 {s2}");
        println!("{}");
        println!("{}");
        println!("{}");
        println!("{}");
    }

    async fn read_from_db() -> String {
        "DB res".to_owned()
    }

    // TOKIO TASKS
    // tokio tasks -> green thread
    #[tokio::main]
    async fn main() {
        let mut handles = vec![];
        for i in 0..2 {
            let h = tokio::spawn(async move {
                mf(i).await;
            });
            handles.push(h);
        }

        for h in handles {
            h.await.unwrap();
        }
    }

    // TOKIO STREAMS
    // stream is async way to pass multiple values
    #[tokio::main]
    async fn main() {
        // because tokio_stream takes iter, can use map on input iter
        let mut stream = tokio_stream::iter(["This","is", "an", "iter"]);

        // streams don't support for loops yet but can access using while
        while let Some(s) = stream.next().await {
            println!("{s}");
        }
    }


}

fn main_threads() {

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

    // MOVING VALUES INTO THREADS

    let s = "Value".to_owned();
    let handle = thread::spawn(move || {
        println!("printing {s}")
    })

    // MESSAGE PASSING
    // import ::sync::mpsc;
    
    let (tx, rx) = mpsc::channel();

    let st = [
        "?ehs si dlo woh".to_owned();
        "ruof egap ta kool".to_owned();
        "xis si owt sulp ruof".to_owned();
        "ruof tsap flah s'ti".to_owned();
    ];

    for s in st {
        let tx_clone = tx.clone();
        thread::spawn(move || {
            // turn sentence in to iterator, reverse iterator, then collect
            // only works for ascii chars
            let rev = s.chars().rev().collect();
            tx_clone.send(rev).unwrap();
        })
    }

    // this code will continue to run because original tx is still live
        // use below to drop tx before receiving to ensure main thread exits
        // must be done before recv
    drop(tx);

    // two options for receiving messages
        // recv blocks main thread until all transmitters are dropped
        // iterator will block for iteration
    rx.recv();
    for s in rx {
        println!("{s}");
    }

    // SHARING STATE BETWEEN THREADS

    // need to use mutex (import using ::sync::Mutex)
    #[derive(Debug)]
    struct Database {
        connections: Vec<u32>
    }

    impl Database {
        fn new() -> Database {
            Database {connections: vec![]}
        }
        fn connect(&mut self, id: u32) {
            self.connections.push(id);
        }
    }

    // imagine in main

    // set database instance within mutex
    let db: Mutex<Database> = Mutex::new(Database::new());
    
    {
    let mut db_lock: MutexGuard<Database> = db.lock().unwrap();
    db_lock.connect(1);
    // lock will be dropped because MutexGuard implements drop & db_lock goes out of scope
    }

     // using db in multiple threads
    let db: Arc<Mutex<Database>> = Arc::new(Mutex::new(Database::new()));
    let mut handles = vec![];
    for i in 0..10 {
        let db = Arc::clone(&db);
        let handle = thread::spawn(move || {
            let mut db_lock = db.lock().unwrap();
            db_lock.push(i);
        });
        handles.push(handle)
    }
    for h in handles {
        h.join().unwrap();
    }
    let db_lock = db.lock().unwrap();
    println!("{db_lock}");

}