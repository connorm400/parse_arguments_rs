//! How to use your own custom enums or structs with parse_argument()
//! You really only need to implement FromStr for your data type.

use parse_argument::parse_argument;
use std::str::FromStr;
use std::process::exit;

#[derive(Debug)]
enum Settings {
    Foo, 
    Bar,
    Baz,
    Default // Default is only used for the sake of an example
}

#[derive(Debug)]
struct ParseSettingsErr;

impl FromStr for Settings {
    type Err = ParseSettingsErr;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "foo" | "Foo" => Ok(Self::Foo),
            "bar" | "Bar" => Ok(Self::Bar),
            "baz" | "Baz" => Ok(Self::Baz),
            _ => Err(ParseSettingsErr)
        }
    }
}

fn main() {
    let setting = parse_argument::<Settings>("setting")
        // use unwrap_or() to set a default value
        .unwrap_or(Ok(Settings::Default))
        // use .map_err() to handle any errors (I like to close the program with these errors)
        .map_err(|_| {
            println!("Error parsing setting");
            exit(1);
        })
        .unwrap()
    ;

    println!("--setting argument: {:?}", setting);
}