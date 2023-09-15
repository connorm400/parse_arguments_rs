# parse_arguments_rs
Easy way to deal with parsing commandline arguments

Can be used with any type that implements FromStr (for parsing).
Use `parse_argument()` function to find a value for a specified key (flag). 
Look at the [examples](https://github.com/connorm400/parse_arguments_rs/tree/main/examples) for an idea of how to use it. 
It works for arguments that look like this: 

```bash
./rust_code --setting=foo --num=42 --hello="world"
```
And to retrieve those values you would write:
```rust
 // assuming you made a Setting enum that implemented FromStr trait
let _ = parse_argument::<Setting>("setting").unwrap().unwrap();
let _ = parse_argument::<i32>("num").unwrap().unwrap();
let _ = parse_argument::<String>("hello").unwrap().unwrap();
```

Alternativelly, you can convert the arguments into a hashmap:
```
let _ = args_to_hashmap();
// which would, in this example, look like {"num": "42", "hello": "world", "setting": "foo"}
```
Run `cargo doc --open`  or go to [docs.rs](https://docs.rs/parse_argument/latest/parse_argument/index.html) to see the documentation.
 
_[crates.io link](https://crates.io/crates/parse_argument) | [Github repo](https://github.com/connorm400/parse_arguments_rs/)_
