# parse_arguments_rs
Easy way to deal with parsing commandline arguments

Crate on [crates.io](https://crates.io/crates/parse_argument). 
Documentation on [docs.rs](https://docs.rs/parse_argument/0.1.3/parse_argument/) or by running `cargo doc --open`.

## TODO
* the key nor the value can have any spaces because of the nature of `std::env::args`. I will try to fix it at some point.
* the hashmap function clones strings which could be more efficient - maybe with string slices
