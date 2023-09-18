fn pass_me_32(number: isize) -> Result<(), String> {
    if number != 32 {
        return Err("you can only pass 32 to me".into());
    }

    return Ok(());
}

fn main() -> Result<(), String> {
    pass_me_32(1)?;

    println!("done");

    return Ok(());
}