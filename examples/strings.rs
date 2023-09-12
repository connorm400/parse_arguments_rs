//! parse_argument can also be used for Strings, as it does have the FromStr trait, but having spaces in them isn't supported yet.
use parse_argument::parse_argument;

fn main() {
    let argument = parse_argument::<String>("string")
        .unwrap_or(Ok(String::from("default")))
        // shouldn't ever be an error so we can safely unwrap it.
        .unwrap()
    ;

    println!("{argument:?}");
}