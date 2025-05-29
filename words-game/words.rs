use rand::seq::SliceRandom;
use std::fs;
use std::io::{self, Write};
use std::process::Command;

fn main() {
    let contents = fs::read_to_string("resources/usa.txt").expect("Cannot read file");
    let lst: Vec<&str> = contents.lines().map(str::trim).collect();

    let mut char_len = input("Enter length of word (3-12): ")
        .parse::<usize>().unwrap_or(0);

    while !(3..=12).contains(&char_len) {
        char_len = input("Enter length of word (3-12): ")
            .parse::<usize>().unwrap_or(0);
    }

    let mut rng = rand::thread_rng();
    let mut word = lst.choose(&mut rng).unwrap_or(&"").to_string();

    while word.len() != char_len {
        word = lst.choose(&mut rng).unwrap_or(&"").to_string();
    }

    let mut pattern: Vec<char> = "-".repeat(char_len).chars().collect();

    let mut tries_count = 0;
    while pattern.iter().collect::<String>() != word {
        clear_screen();
        println!("{}", pattern.iter().collect::<String>());
        let mut char = input("Enter char to guess: ");
        while char.len() != 1 {
            char = input("Pleaser enter 1 valid character: ");
        }

        let c = char.chars().next().unwrap();
        let ins = indexes(&word, c);
        pattern = insert_ins(pattern, &ins, c);

        tries_count += 1;
    }

    clear_screen();
    println!("Congrats, you did this at {}'s try", tries_count);
    println!("{}", word);
}

fn indexes(s: &str, c: char) -> Vec<usize> {
    s.chars()
        .enumerate()
        .filter_map(|(i, ch)| if ch == c { Some(i) } else { None })
        .collect()
}

fn insert_ins(mut p: Vec<char>, ins: &[usize], c: char) -> Vec<char> {
    for &i in ins {
        p[i] = c;
    }
    p
}

fn input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.trim().to_string()
}

fn clear_screen() {
    Command::new("clear").status().unwrap();
}
