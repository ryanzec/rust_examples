pub fn with_none() -> Option<i32> {
    return None;
}

fn main() -> () {
    let value = with_none().unwrap_or_else(|| {
        println!("None value");

        std::process::exit(exitcode::OK);
    });

    println!("has value {}", value);
}