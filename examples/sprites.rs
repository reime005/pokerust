extern crate pokerust;

use pokerust::*;

fn main() -> () {
    let pokemon = Pokemon::from_name("pikachu").unwrap();
    println!("{:?}", pokemon);
}
