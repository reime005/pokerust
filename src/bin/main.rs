use pokerust::Berry;

use std::ops::Rem;

fn main() {
    for id in 0..100 {
        let id: u64 = id;
        let berry = Berry::from_id(id.rem(10) + 1).unwrap();

        println!("{}", id);
        println!("{:?}", berry);
    }
}
