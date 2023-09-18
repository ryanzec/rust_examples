fn panic() -> () {
    back();
}

fn back() -> () {
    trace();
}

fn trace() -> () {
    // we your application gets into a state that is can't recover from it should use the panic!
    // marco, it should be an extreme edge case where this should be used
    // panic! should be favored over assert! as generally it would be easier to read
    panic!("can't recover, help!!!");
}

fn main() -> () {
    panic();
}