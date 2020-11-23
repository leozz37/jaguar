#[macro_use]
extern crate clap;
mod jaguar;

fn main() {
    jaguar::parse_arguments();
}