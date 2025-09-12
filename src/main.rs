pub mod search;

use std::env;
use std::fs;
use colored::Colorize;

fn main() {
    let arg:Vec<String> = env::args().collect();
    search::search(&arg);


}