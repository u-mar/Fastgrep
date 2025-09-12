use std::env;
use std::fs;
fn main() {
    let arg:Vec<String> = env::args().collect();
    let word:String = arg[1].clone();
    let file:String = arg[3].clone();
    println!("Searching {:} in {:}",word,file);
    let texts:String = fs::read_to_string(&file).unwrap();
    let mut total = 0;
    for text in texts.split_whitespace() {
        if text == word {
            total += 1;
        }
    }
    if total == 0 {
        println!("{:} Not Found in {:}",word,file);
    }
    else {
        println!("{:} Found {} times",word,total);
    }

}