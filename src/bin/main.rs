use pokerust::{Berry, Endpoint, List};

fn main() {
    let berry_list = Berry::list(4, 5).unwrap();

    println!("{:?}", berry_list);

    println!("{:?}", berry_list.previous_list());
}
