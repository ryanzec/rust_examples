use rand::Rng;
use std::thread;
use std::time::Duration;

fn main() -> () {
    let numbers = vec![1, 2, 3];
    let mut threads = Vec::new();

    for number in numbers {
        threads.push(thread::spawn(move || {
            let mut rng = rand::thread_rng();

            thread::sleep(Duration::from_secs_f64(rng.gen_range(0.0..0.5)));

            println!("{number:?}");
        }));
    }

    println!("main thread");

    // this lets us wait until all the thread have finished
    for thread in threads {
        thread.join().expect("thread failed to finish");
    }

    println!("main thread finished");
}
