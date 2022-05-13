use std::io::{stdin, stdout, Write};

extern crate termion;
use termion::{clear, cursor};

use hilo::{Command, Deck, Table};

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
        let input = read_input();
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

// TODO select row using arrow keys
fn game_loop(mut deck: Deck, mut table: Table) {
    let mut row_num: usize = 0;
    let input_row = (table.rows.len() * 2 + 1) as u16;
    let mut input: String;
    table.print(&deck, row_num);
    loop {
        print!("{}", clear::All);
        table.print(&deck, row_num);
        loop {
            print!(
                "{}{}{}",
                cursor::Goto(1, input_row),
                clear::CurrentLine,
                "Row? "
            );
            stdout().flush().unwrap();
            input = read_input();
            row_num = match input.parse::<usize>() {
                Ok(n) => n,
                Err(_) => {
                    print!("\nInvalid input!");
                    continue;
                }
            };
            row_num = row_num - 1;
            if !table.has_row(row_num) {
                print!("\nRow does not exist!");
                continue;
            }
            // TODO unit tests for table.print
            // TODO does the whole table need to be reprinted?
            table.print(&deck, row_num);
            print!("{}{}", cursor::Goto(1, input_row + 1), clear::CurrentLine,);
            break;
        }
        let row = table.rows.get_mut(row_num).unwrap();
        let command: Command;
        loop {
            print!(
                "{}{}{}",
                cursor::Goto(1, input_row + 2),
                clear::CurrentLine,
                "Command? [c|al|ar|dl|dr]? "
            );
            stdout().flush().unwrap();
            input = read_input();
            command = match input.as_str() {
                "c" => Command::Collapse,
                "al" => Command::AddLeft,
                "ar" => Command::AddRight,
                "dl" => {
                    if row.len() < 2 {
                        print!("\nCannot remove last card in row!");
                        continue;
                    }
                    Command::RemoveLeft
                }
                "dr" => {
                    if row.len() < 2 {
                        print!("\nCannot remove last card in row!");
                        continue;
                    }
                    Command::RemoveRight
                }
                _ => {
                    print!("\nInvalid command!");
                    continue;
                }
            };
            print!("{}{}", cursor::Goto(1, input_row + 3), clear::CurrentLine,);
            break;
        }
        match command {
            Command::RemoveLeft => {
                row.remove_left(&mut deck);
                continue;
            }
            Command::RemoveRight => {
                row.remove_right(&mut deck);
                continue;
            }
            _ => (),
        }
        let mut card: String;
        loop {
            print!(
                "{}{}{}",
                cursor::Goto(1, input_row + 4),
                clear::CurrentLine,
                "Card? "
            );
            stdout().flush().unwrap();
            card = read_input();
            if !deck.is_card(&card) {
                print!("\nInvalid card!");
                continue;
            }
            if !deck.has_card(&card) {
                print!("\n Card not in deck!");
                continue;
            }
            print!("{}{}", cursor::Goto(1, input_row + 5), clear::CurrentLine,);
            break;
        }
        match command {
            Command::Collapse => row.collapse(card, &mut deck),
            Command::AddLeft => row.add_left(card),
            Command::AddRight => row.add_right(card),
            _ => (),
        };
    }
}

fn read_input() -> String {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    input.trim().to_string().to_lowercase()
}

fn main() {
    let (deck, table) = init();
    game_loop(deck, table);
}
