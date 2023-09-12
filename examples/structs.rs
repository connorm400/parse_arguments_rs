//! You can, of course, also make a struct for the arguments. 
//! Just make sure that the argument does not contain a space or else it 
//! will split the arg.
//! 
//! most of this is taken from the rust std::str::FromStr documentation

use parse_argument::parse_argument;
use std::str::FromStr;
use std::process::exit;

#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32
}

#[derive(Debug, PartialEq, Eq)]
struct ParsePointError;

impl FromStr for Point {
    type Err = ParsePointError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y) = s
            .strip_prefix('(')
            .and_then(|s| s.strip_suffix(')'))
            .and_then(|s| s.split_once(','))
            .ok_or(ParsePointError)?;

        let x_fromstr = x.parse::<i32>().map_err(|_| ParsePointError)?;
        let y_fromstr = y.parse::<i32>().map_err(|_| ParsePointError)?;

        Ok(Point { x: x_fromstr, y: y_fromstr })
    }
}

fn main() {
    let argument = parse_argument::<Point>("point")
        .unwrap_or(Ok(Point {x: 0, y: 0 }))
        .map_err(|e| {
            println!("Error parsing: {e:?}");
            exit(1)
        })
        .unwrap()
    ;

    println!("argument: {argument:?}")
}