use regex::Regex;
use std::collections::HashMap;
use std::io::{stdin, stdout, Error, ErrorKind, Read, Write};
use std::thread::sleep;
use std::time::Duration;

extern crate termion;
use termion::{clear, cursor, input::TermRead};

struct Deck {
    size: usize,
    cards: HashMap<String, bool>,
    values: HashMap<usize, usize>,
}

impl Deck {
    fn new(size: usize) -> Result<Deck, Error> {
        if size % 4 != 0 {
            return Err(Error::new(
                ErrorKind::Other,
                "Deck size must divisible by 4",
            ));
        }
        let mut cards = HashMap::new();
        let mut values = HashMap::new();
        let mut val = 14;
        for _ in 0..(size / 4) {
            values.insert(val, 4);
            for c in ['a', 'b', 'c', 'd'] {
                cards.insert(format!("{}{}", c, val), true);
            }
            val = val - 1;
        }
        Ok(Deck {
            size,
            cards,
            values,
        })
    }

    // TODO handle cards that are not part of the deck
    fn add(&mut self, card: String) -> Result<(), Error> {
        let value = match Deck::parse_value(&card) {
            Ok(value) => value,
            Err(e) => return Err(e),
        };
        self.cards.insert(card, true);
        let count = match self.values.get_mut(&value) {
            Some(count) => count,
            None => return Err(Error::new(ErrorKind::Other, "Card not in deck")),
        };
        *count = *count + 1;
        self.size = self.size + 1;
        Ok(())
    }

    fn remove(&mut self, card: String) -> Result<(), Error> {
        let value = match Deck::parse_value(&card) {
            Ok(value) => value,
            Err(e) => return Err(e),
        };
        self.cards.insert(card, false);
        let count = match self.values.get_mut(&value) {
            Some(count) => count,
            None => return Err(Error::new(ErrorKind::Other, "Card not in deck")),
        };
        *count = *count - 1;
        self.size = self.size - 1;
        Ok(())
    }

    fn calc(&self, card: &String) -> Result<(f32, f32, f32), Error> {
        let comp_value = match Deck::parse_value(&card) {
            Ok(comp_value) => comp_value,
            Err(e) => return Err(e),
        };
        let mut higher = 0;
        let mut equal = 0;
        let mut lower = 0;
        for (value, count) in self.values.iter() {
            if *value > comp_value {
                higher = higher + *count;
            } else if *value == comp_value {
                equal = equal + *count;
            } else {
                lower = lower + *count;
            }
        }
        let chance = |n| -> f32 { n as f32 / self.size as f32 };
        Ok((chance(higher), chance(equal), chance(lower)))
    }

    fn parse_value(card: &String) -> Result<usize, Error> {
        match card[1..].parse::<usize>() {
            Ok(card) => Ok(card),
            Err(_) => return Err(Error::new(ErrorKind::Other, "Invalid card value")),
        }
    }

    fn is_card(card: &String) -> bool {
        let re: Regex = Regex::new(r"^[abc]\d{1,2}$").unwrap();
        if !re.is_match(&card) {
            return false;
        }
        match Deck::parse_value(card) {
            Ok(_) => return true,
            Err(_) => return false,
        };
    }
}

struct Table {
    rows: Vec<Vec<String>>,
}

impl Table {
    fn new(row_count: usize, cards: &Vec<String>) -> Table {
        let mut rows = Vec::new();
        for i in 0..row_count {
            let mut row = Vec::new();
            let card = cards.get(i).unwrap().clone();
            row.push(card);
            rows.push(row);
        }
        Table { rows }
    }

    fn add_left(&mut self, row_number: usize, card: String) {
        let row = self.rows.get_mut(row_number).unwrap();
        row.insert(0, card);
    }

    fn add_right(&mut self, row_number: usize, card: String) {
        let row = self.rows.get_mut(row_number).unwrap();
        row.push(card)
    }

    fn collapse(&mut self, row_number: usize, card: String, deck: &mut Deck) {
        let row = self.rows.get_mut(row_number).unwrap();
        for c in row.drain(..) {
            deck.add(c).unwrap();
        }
        row.push(card);
    }
}

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
    (deck, Table::new(1, &vec![String::from("a2")]))
}

fn main() {
    let (deck, table) = init();
    println!("{:#?}", deck.cards);

    // let val = Deck::parse_value(String::from("a1499123"));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn deck_can_be_created() {
        let deck = Deck::new(52).unwrap();
        assert_eq!(deck.size, 52);
        let card = String::from("a5");
        let card = deck.cards.get(&card).unwrap();
        assert!(card);
        assert_eq!(*deck.values.get(&7).unwrap(), 4);
    }

    #[test]
    fn deck_can_add_cards() {
        let mut deck = Deck::new(36).unwrap();
        let card = String::from("a10");
        deck.add(card.clone()).unwrap();
        assert_eq!(deck.size, 37);
        let card = deck.cards.get(&card).unwrap();
        assert!(card);
    }

    #[test]
    fn deck_can_remove_cards() {
        let mut deck = Deck::new(36).unwrap();
        let card = String::from("a10");
        deck.remove(card.clone()).unwrap();
        assert_eq!(deck.size, 35);
        let card = deck.cards.get(&card).unwrap();
        assert!(!card);
    }

    #[test]
    fn deck_can_calculate_chance() {
        let mut deck = Deck::new(8).unwrap();
        let card = String::from("a14");
        deck.remove(card.clone()).unwrap();
        let (higher, equal, lower) = deck.calc(&card).unwrap();
        assert_eq!(higher, 0.0);
        assert_eq!(equal, 3.0 / 7.0);
        assert_eq!(lower, 4.0 / 7.0);
    }

    #[test]
    fn deck_can_parse_card_values() {
        let card1 = String::from("a1");
        let card2 = String::from("b14");
        let card3 = String::from("bb");
        let card4 = String::from("");
        assert_eq!(Deck::parse_value(&card1).unwrap(), 1);
        assert_eq!(Deck::parse_value(&card2).unwrap(), 14);
        let c3 = match Deck::parse_value(&card3) {
            Ok(_) => false,
            Err(_) => true,
        };
        assert!(c3);
        let c4 = match Deck::parse_value(&card4) {
            Ok(_) => false,
            Err(_) => true,
        };
        assert!(c4);
    }

    #[test]
    fn table_can_be_created() {
        let cards = vec![String::from("a1"), String::from("b2"), String::from("c3")];
        let table = Table::new(3, &cards);
        for i in 0..3 {
            assert_eq!(
                table.rows.get(i).unwrap().get(0).unwrap(),
                cards.get(i).unwrap()
            )
        }
    }

    #[test]
    fn table_can_add_cards() {
        let card1 = String::from("a1");
        let card2 = String::from("b2");
        let cards = vec![card1.clone(), card2.clone()];
        let mut table = Table::new(2, &cards);
        let card_left = String::from("c4");
        let card_right = String::from("d10");
        table.add_left(0, card_left.clone());
        table.add_right(1, card_right.clone());
        assert_eq!(*table.rows.get(0).unwrap(), vec![card_left, card1]);
        assert_eq!(*table.rows.get(1).unwrap(), vec![card2, card_right])
    }

    #[test]
    fn table_can_collapse_into_deck() {
        let mut deck = Deck::new(16).unwrap();
        let card1 = String::from("a14");
        let card2 = String::from("b13");
        deck.remove(card1.clone()).unwrap();
        deck.remove(card2.clone()).unwrap();
        let mut table = Table::new(1, &vec![card1.clone()]);
        table.add_right(0, card2.clone());
        assert_eq!(deck.size, 14);
        assert!(!deck.cards.get(&card1).unwrap());
        assert!(!deck.cards.get(&card2).unwrap());
        assert_eq!(
            table.rows.get(0).unwrap(),
            &vec![card1.clone(), card2.clone()]
        );
        table.collapse(0, String::from("a1"), &mut deck);
        assert_eq!(deck.size, 16);
        assert!(deck.cards.get(&card1).unwrap());
        assert!(deck.cards.get(&card2).unwrap());
    }
}
