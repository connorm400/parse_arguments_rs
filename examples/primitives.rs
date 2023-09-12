use parse_argument::parse_argument;
use std::process::exit;

fn main() { 
    let num = parse_argument::<u8>("num")
        // use unwrap_or() to set a default value
        .unwrap_or(Ok(2))
        // use .map_err() to handle any errors (I like to close the program with these errors)
        .map_err(|e| {
            println!("Argument didn't parse: {e:?}");
            exit(1);
        })
        .unwrap()
    ;

    println!("{num}");

    exit(0);
}
