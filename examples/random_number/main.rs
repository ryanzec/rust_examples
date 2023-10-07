use rand::Rng;

fn random_number() -> u32 {
    let mut rng = rand::thread_rng();

    rng.gen_range(0..100000)
}

fn main() -> () {
    println!("{}", random_number());
}
