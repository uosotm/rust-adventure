extern crate clap;
use clap::{Arg, App};

fn main() {
    let matches = App::new("Hello")
        .version("0.1.0")
        .author("User Name <user@example.com>")
        .about("The program that greets with a name")
        .arg(Arg::with_name("NAME")
             .required(true)
             .takes_value(true)
             .help("Name to greet with"))
        .get_matches();

    let name = matches.value_of("NAME").unwrap();
    println!("Hello, {}", name);
}
