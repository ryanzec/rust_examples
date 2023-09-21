use std::fs;
use std::fs::File;
use std::io::{Error, ErrorKind};

const FILE_PATH: &str = "error_handling_with_closure.txt";

fn open_or_create_file() -> Result<File, Error> {
    // this is a closure to handle the error that might happen, in this case we are creating
    // the file only if the error we got when trying to open it was that is does not exist
    File::open(FILE_PATH).or_else(|error| {
        if error.kind() != ErrorKind::NotFound {
            panic!("unable to open file: {error:?}");
        }

        File::create(FILE_PATH)
    })
}

fn main() -> Result<(), Error> {
    let _ = open_or_create_file();

    fs::remove_file(FILE_PATH)?;

    Ok(())
}
