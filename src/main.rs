#[macro_use]
extern crate clap;
use clap::App;
use std::net::TcpStream;


fn ping(hostname: &str, port: &str) {
    let address = hostname.to_owned() + ":" + port;
    println!("PING {}", &address);

    if let Ok(_stream) = TcpStream::connect(&address) {
        println!("{} alive!", &address);
    } else {
        println!("Couldn't connect to server");
    }
}

fn main() {
    let yaml = load_yaml!("../resources/cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    let hostname = matches.value_of("hostname").unwrap_or("127.0.0.1");
    let port = matches.value_of("port").unwrap_or("");

    ping(hostname, port)
}