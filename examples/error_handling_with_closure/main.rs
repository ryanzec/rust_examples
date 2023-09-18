use std::fs;
use std::fs::File;
use std::io::{Error, ErrorKind};
use rand::prelude::*;

const FILE_PATH: &str = "error_handling_with_closure.txt";

fn open_or_create_file() -> () {
    let _file = File::open(FILE_PATH).unwrap_or_else(|error| {
        if error.kind() != ErrorKind::NotFound {
            panic!("unable to open file: {error:?}");
        }

        return File::create(FILE_PATH).unwrap_or_else(|error| {
            panic!("unable to create file: {error:?}")
        })
    });
}

fn remove_file_error_handler(error: Error) -> Result<(), Error> {
    // any error other than not found should kill the application
    if error.kind() != ErrorKind::NotFound {
        return Err(error);
    }

    return Ok(());
}

fn main() -> Result<(), Error> {
    fs::remove_file(FILE_PATH).or_else(remove_file_error_handler)?;

    open_or_create_file();

    fs::remove_file(FILE_PATH).or_else(remove_file_error_handler)?;

    // this should randomly cause the application to panic
    if random() {
        fs::remove_file(FILE_PATH)?;
    }

    return Ok(());
}