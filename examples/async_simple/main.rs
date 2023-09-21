use tokio::io::AsyncReadExt;
use tokio::{fs::File, io, task};

async fn read_file(path: &str) -> io::Result<String> {
    let mut file = File::open(path).await?;
    let mut contents = String::new();

    file.read_to_string(&mut contents).await?;

    return Ok(contents);
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    let task = task::spawn(async {
        let results = read_file("simple.txt").await;

        match results {
            Ok(contents) => println!("file contents: {contents}"),
            Err(error) => println!("error reading file: {error}"),
        }
    });

    // once the task is created, it automatically starts
    println!("task1 has started");

    // other code can execute however if you get to a point when you need to wait for the task
    // to finish, you use task::block_on();
    task.await?;

    println!("task1 has finished");

    return Ok(());
}
