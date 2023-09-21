use std::thread;
use std::time::Duration;

fn main() -> () {
    let handle = thread::spawn(|| {
        println!("starting thread");

        thread::sleep(Duration::from_secs(1));

        println!("exiting thread");
    });

    // .join() stops the execute of the main thread until this thread related to the handle
    // completes
    handle.join().expect("thread failed to finish");
}
