extern crate clap;
use clap::{Arg, App};

fn main() {
    let matches = App::new("Jaguar")
        .version("0.1")
        .author("Leonardo Lima <leonardoaugusto287@gmail.com>")
        .about("Test socket connetions")
        .arg(Arg::with_name("port")
            .short("p")
            .long("port")
            .value_name("PORT")
            .help("Port to be interacted with")
            .takes_value(true))
        .get_matches();

    let port = matches.value_of("port");
    println!("PORT: {:?}", port);
}