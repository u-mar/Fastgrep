#![warn(dead_code)]

pub mod search;
pub mod args;
use std::env::args;
fn main() {
    let arg:Vec<String> = args().collect();
    let a: args::Arguments = args::Arguments::build(&arg);
    println!("{:?}",a);
    search::search(&arg);


}