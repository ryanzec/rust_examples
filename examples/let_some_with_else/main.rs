fn return_none_if_10(number: i32) -> Option<i32> {
    if number == 10 {
        return None;
    }

    Some(number)
}
fn main() -> () {
    let Some(value1) = return_none_if_10(100) else {
        panic!("this should never happen");
    };

    println!("{}", value1);

    // else block must diverge, which I think means it must end the current function call with
    // a return, panic, continue, etc., as the rest of the code following this statement can not
    // be assumed to run properly with the let Some(...) making the assignment
    let Some(value2) = return_none_if_10(10) else {
        println!("this should print last, None returned");

        return;
    };
}
