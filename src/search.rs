use std::fs;
use colored::Colorize;


pub fn search(arg:&Vec<String>) {
    if arg.len() < 3 {
        panic!("Not enough Arguments");
    }
    let word:String = arg[1].clone();
    let file:String = arg[3].clone();
    println!("Searching {:} in {:}",word,file);
    let texts:String = fs::read_to_string(&file).unwrap();
    let mut found = false;
    for (i,line) in texts.lines().enumerate(){
        if line.to_lowercase().contains(&word.to_lowercase()){
            found = true;
            let highlighted = line.replace(&word, &word.green().bold().to_string());
            println!("{}: {}", (i + 1).to_string().blue(), highlighted);
        }
    }
    if !found {
        println!("{}", "No matches found".red());
    }
    
}