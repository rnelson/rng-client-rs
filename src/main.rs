#[macro_use]
extern crate clap;
extern crate curl;

use std::io::{stdout, Write};
use std::process;
use curl::easy::Easy;

fn main() {
    let matches = clap_app!(app =>
                            (version: "0.1.0")
                            (author: "Ross Nelson <ross.nelson@gmail.com>")
                            (about: "Client for RNG Service")
                            (@arg SERVER: -s --server +takes_value "Sets the server hostname/IP")
                            (@arg TYPE: -t --type +takes_value +required "Sets the data type")
                            ).get_matches();

    let server = matches.value_of("SERVER").unwrap_or("127.0.0.1");
    let requested_type = matches.value_of("TYPE").unwrap();

    let mut url = String::from("http://");
    url.push_str(server);
    url.push_str(":10713/generate/");

    match requested_type {
        "integer" | "int" | "i" => url.push_str("int"),
        "floatingPoint" | "float" | "f" => url.push_str("float"),
        "doublePrecision" | "double" | "d" => url.push_str("double"),
        _ => error(String::from("unacceptable type")),
    }

    let mut easy = Easy::new();
    easy.url(&url).unwrap();
    easy.write_function(|data| {
        stdout().write_all(data).unwrap();
        Ok(data.len())
    }).unwrap();

    easy.perform().unwrap();
}

fn error(message: String) {
    println!("error: {}", message);
    process::exit(1)
}
