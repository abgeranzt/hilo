use std::io::{stdin, stdout, Write};

extern crate termion;
use termion::{clear, cursor};

use hilo::{Deck, Table};

fn init() -> (Deck, Table) {
    // TODO print usage
    print!("{}{}", clear::All, cursor::Goto(1, 1,));
    let mut deck: Deck;
    loop {
        print!("Deck size? ");
        stdout().flush().unwrap();
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        input = input.trim().to_string();
        let size = match input.parse::<usize>() {
            Ok(size) => size,
            Err(_) => {
                println!("\nInvalid input!");
                continue;
            }
        };
        deck = match Deck::new(size) {
            Ok(deck) => deck,
            Err(e) => {
                println!("\n{}", e.to_string());
                continue;
            }
        };
        break;
    }
    let rows: usize;
    loop {
        print!("\n\nRows? ");
        stdout().flush().unwrap();
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        input = input.trim().to_string();
        rows = match input.parse::<usize>() {
            Ok(rows) => rows,
            Err(_) => {
                println!("\nInvalid input!");
                continue;
            }
        };
        break;
    }
    let mut cards: Vec<String>;
    let table: Table;
    loop {
        print!("Inital cards? ");
        stdout().flush().unwrap();
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        input = input.trim().to_string();
        cards = input.split(",").map(|c| String::from(c)).collect();
        // TODO more verbose user information
        if cards.len() != rows {
            println!("\nCard amount must match row count");
            continue;
        }
        if !cards.iter().all(|c| deck.is_card(c) && deck.has_card(c)) {
            println!("\nInvalid card(s)");
            continue;
        }
        for c in cards.iter() {
            deck.remove(c).unwrap();
        }
        table = match Table::new(rows, cards) {
            Ok(table) => table,
            Err(_) => {
                println!("\nInvalid input!");
                continue;
            }
        };
        break;
    }

    (deck, table)
}

fn main() {}

#[cfg(test)]
mod test {
    use super::*;
}
