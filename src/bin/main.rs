use pokerust::{Berry, Endpoint};

fn main() {
    let berry_list = Berry::list(4, 5).unwrap();

    println!("{:?}", berry_list);
}
