extern crate rand;
use rand::prelude::*;

fn main() {
    let mut rng = rand::thread_rng();

    println!("Random die roll: {}", rng.gen_range(1..=30));
    println!("Random die roll: {}", rng.gen_range(1..=12));
    println!("Random die roll: {}", rng.gen_range(1990..=2000));
    println!("Random UUID: 0x{:X}", rng.gen::<u128>());

    if rng.gen() {
        println!("You got lucky!");
    }
}
