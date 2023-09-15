//!# parse_arguments_rs
//!Easy way to deal with parsing commandline arguments
//!
//!Can be used with any type that implements FromStr (for parsing).
//!Use `parse_argument()` function to find a value for a specified key (flag). 
//! Look at the [examples](https://github.com/connorm400/parse_arguments_rs/tree/main/examples) for an idea of how to use it. 
//!It works for arguments that look like this: 
//!
//!```bash
//!./rust_code --setting=foo --num=42 --hello="world"
//!```
//!And to retrieve those values you would write:
//!```rust
//! // assuming you made a Setting enum that implemented FromStr trait
//!let _ = parse_argument::<Setting>("setting").unwrap().unwrap();
//!let _ = parse_argument::<i32>("num").unwrap().unwrap();
//!let _ = parse_argument::<String>("hello").unwrap().unwrap();
//!```
//!
//! Alternativelly, you can convert the arguments into a hashmap:
//! ```
//! let _ = args_to_hashmap();
//! // which would, in this example, look like {"num": "42", "hello": "world", "setting": "foo"}
//! ```
//!Run `cargo doc --open` to see the documentation.
//! 
//! _[crates.io link](https://crates.io/crates/parse_argument) | [Github repo](https://github.com/connorm400/parse_arguments_rs/)_
//!

use std::env::args;
use std::str::FromStr;

/// Errors for parse_argument(). Takes type `<<T as FromStr>::Err>`
#[derive(Debug, PartialEq)]
pub enum ParseArgumentError<T> {
    BadLen,
    ParseError(T)
}
type Err<T> = ParseArgumentError<<T as FromStr>::Err>;

/// Takes an argument flag, and a type (generally with turbofish syntax).
/// Will look for any arguments in `std::env::args` that contains `--{flag}=`.
/// Then it will split it and try to parse it.
/// 
/// Can be used with any type that implements `std::str::FromStr`. You can 
/// implement it for your own custom types.
/// 
/// Usefull for adding extra runtime settings to a cli application.
/// 
///  **_Make sure your argument / value does not have spaces. At least currently, it will not work._**
/// 
/// # Errors
/// * None if the argument isn't used by the end user
/// 
/// * `Some(Err(ParseArgumentError::BadLen))` if there the vector from splitting the argument by 
///   the `=` sign's length isn't 2 (as in key and value). That also means you can't have equal signs in 
///   your arguments.
/// 
/// * `Some(Err(ParseArgumentError::ParseError(ParseError)))` if parsing the value doesn't work out
/// 
/// # Example
/// 
/// ```
/// // assuming you run this as "cargo r -- --num=2"
/// println!("{:?}", 
///     parse_argument::<u8>("num")
/// );
/// ```
pub fn parse_argument<T>(flag: &str) -> Option<Result<T, Err<T>>> 
where T: FromStr {
    args().find(|x| x.contains(format!("--{flag}=").as_str()))
        .and_then(|x| {
            let split_vec: Vec<&str> = x.split("=").collect();
            match split_vec.len() {
                2 => Some(split_vec[1].parse::<T>().map_err(|e| ParseArgumentError::ParseError(e))),
                _ => Some(Err(ParseArgumentError::BadLen))
            }
        })
}

use std::collections::HashMap;
/// Returns commandline arguments in a hashmap (T and E are Strings). 
/// 
/// To be a part of the hashmap, the argument must have this formatting:
/// 
/// ```bash
/// ./rust_code --argument=true --hello=foo --num=42
/// ```
/// And you could access these by doing this in rust: 
/// ```
/// let arguments = args_to_hashmap();
/// println!("{arguments:?}");
/// //will return {"argument": "true", "hello": "foo", "num": "42"}
/// ```
pub fn args_to_hashmap() -> HashMap<String, String> {
    let mut map = HashMap::new();
    for arg in args() {
        arg.strip_prefix("--").and_then(|a| {
            let split_a: Vec<&str> = a.split("=").collect();
            match split_a.len() {
                2 => Some(map.entry(split_a[0].to_owned()).or_insert(split_a[1].to_owned())),
                _ => None
            }
        });
    }
    map
}

// its not very possible to test parse_argument<T> since its basically just a procedure
// that analyzes a side effect of sorts.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_test() {
        assert_eq!(parse_argument::<String>("hello"), None);
    } 
}