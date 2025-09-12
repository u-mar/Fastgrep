use std::fs;
use colored::Colorize;

use crate::args::Arguments;


pub fn search(arg:&Vec<String>,a:Arguments) {
    if arg.len() < 3 {
        panic!("Not enough Arguments");
    }
    let word:String = arg[2].clone();
    let file:String = arg[4].clone();
    println!("Searching {:} in {:}",word,file);
    let texts:String = fs::read_to_string(&file).unwrap();
    let mut found = false;
    for (i,line) in texts.lines().enumerate(){
        if a.i {
            let w = word.to_lowercase();
            if line.to_lowercase().contains(&w){
                found = true;
                let highlighted = line.replace(&w, &w.green().bold().to_string());
                println!("{}: {}", (i + 1).to_string().blue(), highlighted);
            }
        }
        else {
            if line.contains(&word){
                found = true;
                let highlighted = line.replace(&word, &word.green().bold().to_string());
                println!("{}: {}", (i + 1).to_string().blue(), highlighted);
            }
        }
    }
    if !found {
        println!("{}", "No matches found".red());
    }
    
}