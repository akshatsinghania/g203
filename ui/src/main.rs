use lib::hello_from_lib;

extern crate lib;

fn main() {
    hello_from_lib();
    println!("Hello, world!");
}
