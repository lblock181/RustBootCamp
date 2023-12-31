# Notes on Concurrency & Async/Await in Rust

## Intro

Multi-processing 
- many processes running at same time (e.g. chrome extensions)
- useful when want multiple processes of same code running w/o much interaction w/ each other

Threads & Multi-threading
- threads are spawn w/in process
- spawning multiple threads == multi threading
    - same heap but own stack

Concurrency
- parts of program executes independently
- common models of concurrency
    - Time Slicing
        - execution of parts interleaved on single core
    - Parallel Execution
        - execution of parts happens at same time using multiple cores
    - OS Threads
        - basic primative provided by OS
    - Coroutines
        - hide low level details
    - Async programming
        - exposes low level details
    - Event driven
        - uses callbacks
    - Actor model
        - divide compuation into actors that can communicate through messages

### OS Threads vs Async Programming
OS Threads
- managed by OS
- preemptive schedule
- higher performance overhead **
- harded to reason about
- ideal for small amounts of CPU bound workloads

Async Task
- managed by runtime or library
- cooperatively scheduled
- lower performance overhead
- easier to reason about
- ideal for I/O bound workloads

Comes down to CPU or I/O bound workload

## Working with threads
see [main.rs](./concurrency/main.rs) for code snippets
- spanwning threads will return handle
- to ensure all threads complete before main thread terminates
    - use `handle.join().unwrap()`
    - set equal to value if thread returns values
- threads take ownership of variables
    - to use variable within thread scope, use `move` keyword

## Send & Sync
- traits for async programming
- marker traits (properties enforced by dev) e.g. do nothing
- both are `unsafe`
- all rust primatives (except Rc) implement Send
    - if custom type made up of types that all impl send => custom type is Send
- types are Sync if they impl Send

## Async Await
- returns Future<Output>
- top level async (usually fn main) needs to use runtime to call/poll async
- Rust does not have runtime built into language
- most popular async runtime is Tokio
- to use Tokio, annotate fn main w/ `#[tokio::main]`
    - allows for main to be set as async
    ```rust
    #[tokio::main]
    async main() {}
    ```

## Streams
tokio async can only return one value.
to circumvent this, use tokio_stream
common to use on servers for TcpStream