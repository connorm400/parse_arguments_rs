//! How I'd really recommend using parse_argument with a match statement rather than combinators

use parse_argument::parse_argument;

fn main() {
    let argument: String = match parse_argument("string") {
        None => String::from("default"),
        Some(Ok(s)) => s,
        Some(Err(e)) => {
            eprintln!("error parsing argument: {e:?}");
            std::process::exit(1);
        }
    }; 

    println!("{argument}");
}
