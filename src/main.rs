use regex::Regex;
use std::collections::HashMap;
use std::fmt;
use std::io::{stdin, stdout, Error, ErrorKind, Read, Write};
use std::thread::sleep;
use std::time::Duration;

extern crate termion;
use termion::{clear, color, cursor, input::TermRead};

use hilo::{Deck, Table};

fn init() -> (Deck, Table) {
    let deck: Deck;
    let table: Table;
    loop {
        print!("{}{}Deck size? ", clear::All, cursor::Goto(1, 1));
        stdout().flush().unwrap();
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        let size = match input[..input.len() - 1].parse::<usize>() {
            Ok(size) => size,
            Err(_) => {
                println!("\nInvalid input!");
                sleep(Duration::new(1, 0));
                continue;
            }
        };
        deck = match Deck::new(size) {
            Ok(deck) => deck,
            Err(e) => {
                println!("\n{}", e.to_string());
                sleep(Duration::new(1, 0));
                continue;
            }
        };
        break;
    }
    (deck, Table::new(1, &vec![String::from("a2")]).unwrap())
}

fn main() {}

#[cfg(test)]
mod test {
    use super::*;
}
