use pokerust::{Berry, FromId};

fn main() {
    let berry = Berry::from_id(1).unwrap();
    let foo = berry.firmness.get().unwrap();

    println!("{:?}", foo);
}
