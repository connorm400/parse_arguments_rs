//!# parse_arguments_rs
//!Easy way to deal with parsing commandline arguments
//!
//!Can be used with any type that implements FromStr (for parsing).
//!Use `parse_argument()` function to find a value for a specified key (flag). Look at the examples for an idea of how to use it. 
//!It works for arguments that look like this: 
//!
//!```bash
//!./rust_code --setting=foo --num=42 --hello="world"
//!```
//!And to retrieve those values you would write:
//!```rust
// assuming you made an Setting enum that implemented FromStr trait
//!let _ = parse_argument::<Setting>("option").unwrap().unwrap();
//!let _ = parse_argument::<i32>("num").unwrap().unwrap();
//!let _ = parse_argument::<String>("hello").unwrap().unwrap();
//!```
//!
//!Run `cargo doc --open` to see the documentation.
//! 
//!_Will Hopefully be on [crates.io](https://crates.io) soon  | [Github repo](https://github.com/connorm400/parse_arguments_rs/)_
//!
//!## TODO
//!* the key nor the value can have any spaces because of the nature of `std::env::args`. I will try to fix it at some point.
//!* Make a function that returns a hashmap of all the arguments with their key and value

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
            if split_vec.len() != 2 {
                Some(Err(ParseArgumentError::BadLen))
            } else {
                Some(split_vec.get(1).unwrap().parse::<T>().map_err(|e| ParseArgumentError::ParseError(e)))
            }
        })
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