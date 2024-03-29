use clap::App;
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::io::Read;
use std::io::Write;
use std::str;

fn ping(hostname: &str, port: &str) -> String {
    let address = hostname.to_owned() + ":" + port;
    println!("PING {}", &address);

    if let Ok(_stream) = TcpStream::connect(&address) {
        return "Alive".to_string();
    } else {
        return "Couldn't connect to server".to_string();
    }
}

// TODO: handle return and connection error
fn send(hostname: &str, port: &str, data: &str) -> std::io::Result<()> {
    let address = hostname.to_owned() + ":" + port;
    let mut stream = TcpStream::connect(address)?;

    stream.write(data.as_bytes())?;
    Ok(())
}

fn handle_client(mut stream: TcpStream) {
    loop {
        let mut read = [0; 1028];
        match stream.read(&mut read) {
            Ok(n) => {
                if n == 0 { 
                    break;
                }
                stream.write(&read[0..n]).unwrap();

                let s = match str::from_utf8(&read[0..n]) {
                    Ok(v) => v,
                    Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
                };
                println!("Received: {}", s);
            }
            Err(err) => {
                panic!(err);
            }
        }
    }
}

fn listen(hostname: &str, port: &str) {
    let address = hostname.to_owned() + ":" + port;
    let listener = TcpListener::bind(address).unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(move || {
                    handle_client(stream);
                });
            }
            Err(_) => {
                println!("Error");
            }
        }
    }
}

pub fn parse_arguments() {
    let yaml = load_yaml!("../resources/cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    let hostname = matches.value_of("hostname").unwrap_or("127.0.0.1");
    let port = matches.value_of("port").unwrap_or("");

    if matches.is_present("listen") {
        listen(hostname, port);
    }
    else if matches.is_present("send") {
        let data = matches.value_of("data").unwrap_or("test");
        let _ = send(hostname, port, data);
    }
    else {
        println!("{}", ping(hostname, port));
    }
}
