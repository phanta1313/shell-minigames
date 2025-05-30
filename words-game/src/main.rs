use std::fs::File;
use std::io::{self, BufRead, BufReader};
use rand::Rng; 
use std::process::Command;

fn indexes(s: &str, c: char) -> Vec<usize> {
    s.char_indices()
        .filter_map(|(i, ch)| if ch == c { Some(i) } else { None })
        .collect()
}

fn insert_ins(pattern: &mut Vec<char>, ins: Vec<usize>, c: char) {
    for i in ins {
        pattern[i] = c;
    }
}

fn main() {
    let file = File::open("resources/usa.txt").expect("Не удалось открыть файл");
    let reader = BufReader::new(file);
    let lst: Vec<String> = reader
        .lines()
        .map(|line| line.expect("Ошибка чтения строки"))
        .collect();

    let mut char_len = 0;
    while !(3..=12).contains(&char_len) {
        println!("Enter length of word (3-12): ");
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        char_len = input.trim().parse().unwrap_or(0);
    }

    let mut rng = rand::rng();
    let mut word = lst[rng.random_range(0..lst.len())].clone();
    while word.len() != char_len {
        word = lst[rng.random_range(0..lst.len())].clone();
    }

    let mut pattern: Vec<char> = "-".repeat(char_len).chars().collect();

    let mut tries_count = 0;

    while pattern.iter().collect::<String>() != word {
        Command::new("cmd").args(["/C", "cls"]).status().unwrap();

        println!("{}", pattern.iter().collect::<String>());
        println!("Enter char to guess: ");

        let mut char_input = String::new();
        io::stdin().read_line(&mut char_input).unwrap();
        let mut char_input = char_input.trim().to_string();

        while char_input.len() != 1 {
            println!("Please enter 1 valid character: ");
            char_input.clear();
            io::stdin().read_line(&mut char_input).unwrap();
            char_input = char_input.trim().to_string();
        }

        let c = char_input.chars().next().unwrap();
        let ins = indexes(&word, c);
        insert_ins(&mut pattern, ins, c);

        tries_count += 1;
    }

    Command::new("cmd").args(["/C", "cls"]).status().unwrap();

    println!("Congrats, you did this at {}'s try", tries_count);
    println!("{}", word);
}
