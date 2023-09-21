use bevy::prelude::error;

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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let results = pass_me_32(1);

    if let Err(erro) = results {
        println!("{}", erro);
    }

    Ok(())
}
