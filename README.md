# parse_arguments_rs
Easy way to deal with parsing commandline arguments

Can be used with any type that implements FromStr (for parsing).
Use `parse_argument()` function to find a value for a specified key (flag). Look at the examples for an idea of how to use it. 
It works for arguments that look like this: 

```bash
./rust_code --option=foo --num=42 --hello="world"
```

Run `cargo doc --open` to see the documentation.
Will Hopefully be on [crates.io](https://crates.io) soon

## TODO
* the key nor the value can have any spaces because of the nature of `std::env::args`. I will try to fix it at some point.
* Make a function that returns a hashmap of all the arguments with their key and value
