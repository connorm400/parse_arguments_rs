//! The hash map function is very simple and easy to use. Simply call 
//! the function and .get() whatever you want. 

use parse_argument::args_to_hashmap;

fn main() {
    let map = args_to_hashmap();
    println!("{map:#?}");
}