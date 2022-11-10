use colored::Colorize;
use rand::Rng;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::io::{stdin, BufReader};
fn main() {
    let paths_dir = fs::read_dir("./tests").unwrap();
    println!("{}", "Choose test:".cyan());
    let mut paths: Vec<String> = Vec::new();
    for (i, path) in paths_dir.enumerate() {
        let test_path = path.unwrap().path().display().to_string();
        println!("{}: {}", i.to_string().green(), test_path.clone().magenta());
        paths.push(test_path);
    }
    let mut test_num = String::new();
    stdin().read_line(&mut test_num).expect("Error");
    let test_num = test_num.trim().parse::<usize>().unwrap();
    let test = paths.iter().nth(test_num).unwrap();
    println!("{} {}", "You choose".yellow(), test.bright_purple());
    let file = File::open(test).expect("Error while opening test file");
    let lines = BufReader::new(file).lines();

    let mut words: Vec<String> = Vec::new();
    let mut translations: Vec<String> = Vec::new();

    for line in lines {
        match line {
            Ok(text) => {
                let splitted = text.split(":").collect::<Vec<&str>>();
                words.push(splitted.iter().nth(0).unwrap().to_string());
                translations.push(splitted.iter().nth(1).unwrap().to_string());
            }
            Err(e) => println!("ERROR: {}", e.to_string()),
        }
    }
    for (word, translation) in words.iter().zip(translations.iter()) {
        println!("{} - {}", word.green(), translation.purple());
    }

    println!(
        "{} ",
        "Choose type of training(0 - org, 1 -trs):"
            .on_white()
            .black()
    );
    let mut type_str = String::new();
    stdin().read_line(&mut type_str).expect("error");
    let type_n: usize = type_str.trim().parse().unwrap();

    if type_n == 1 {
        std::mem::swap(&mut words, &mut translations);
    }

    let mut rng = rand::thread_rng();
    let mut used: Vec<usize> = Vec::new();
    loop {
        if used.len() == words.len() {
            println!("{}", "Finished!".on_white().black());
            break;
        }
        let mut key;
        loop {
            key = rng.gen_range(0..words.len());
            if !used.contains(&key) {
                break;
            }
        }
        println!(
            "{} '{}' ({}/{}): ",
            "Enter translation of word".yellow(),
            words.iter().nth(key).unwrap().green(),
            used.len().to_string().green(),
            words.len().to_string().purple()
        );
        let mut answer = String::new();
        stdin().read_line(&mut answer).unwrap();
        if *answer.trim() == *translations.iter().nth(key).unwrap().trim() {
            println!("{}", "Right!".green());
            used.push(key);
        } else {
            println!(
                "{} '{}'",
                "Not right! Answer is".red(),
                translations.iter().nth(key).unwrap().red().bold()
            );
        }
    }
}
