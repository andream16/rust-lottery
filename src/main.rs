extern crate rand;

use rand::Rng;

fn main() {

    let entries: [&str; 5] = ["Batman", "Superman", "Spiderman", "PolPot", "Bunneh"];

    println!("{:?}", rand::thread_rng().choose(&entries));
}
