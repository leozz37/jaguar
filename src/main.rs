extern crate clap;
use clap::{Arg, App};
use std::net::TcpStream;


fn ping(port: &str) {
    println!("PING {}", port);

    if let Ok(_stream) = TcpStream::connect("127.0.0.1:".to_owned() + port) {
        println!("{} alive!", port);
    } else {
        println!("Couldn't connect to server");
    }
}

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

    let port = matches.value_of("port").unwrap_or("test");
    ping(port)
}