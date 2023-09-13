use parse_argument::args_to_hashmap;

fn main() {
    let map = args_to_hashmap();
    println!("{map:#?}");
}