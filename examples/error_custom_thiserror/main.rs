#[derive(thiserror::Error, Debug, Clone)]
enum CustomError {
    #[error("you must pass 32")]
    MustPass32,
}

fn pass_me_32(number: isize) -> Result<(), CustomError> {
    if number != 32 {
        return Err(CustomError::MustPass32);
    }

    return Ok(());
}

fn main() -> () {
    let result = pass_me_32(1);

    match result {
        Ok(_) => {},
        Err(error) => match error {
            CustomError::MustPass32 => println!("{}", error),
        },
    }
}