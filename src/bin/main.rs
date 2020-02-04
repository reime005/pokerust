use pokerust::{Berry, FromId};

use std::ops::Rem;

fn main() {
    for id in 0..100 {
        let id: i64 = id;
        let berry = Berry::from_id(id.rem(10) + 1).unwrap();

        println!("{}", id);
        println!("{:?}", berry);
    }
}
