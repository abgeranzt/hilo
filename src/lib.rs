use regex::Regex;
use std::collections::HashMap;
use std::fmt;
use std::io::{Error, ErrorKind};

extern crate termion;
use termion::{clear, color, cursor};

pub enum Command {
    Collapse,
    AddLeft,
    AddRight,
    RemoveLeft,
    RemoveRight,
}

pub struct Deck {
    size: usize,
    cards: HashMap<String, bool>,
    values: HashMap<usize, usize>,
}

impl Deck {
    pub fn new(size: usize) -> Result<Deck, Error> {
        if size > 52 || size % 4 != 0 {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                "Deck size must no larger than 52 and divisible by 4",
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

    pub fn is_card(&self, card: &String) -> bool {
        let re: Regex = Regex::new(r"^[abcd]\d{1,2}$").unwrap();
        if !re.is_match(card) {
            return false;
        }
        let value = match Deck::parse_value(card) {
            Ok(value) => value,
            Err(_) => return false,
        };
        if value > 14 || value < (14 - self.cards.len() / 4) + 1 {
            return false;
        }
        true
    }

    pub fn has_card(&self, card: &String) -> bool {
        match self.cards.get(card) {
            Some(card) => *card,
            None => false,
        }
    }

    pub fn add(&mut self, card: String) -> Result<(), Error> {
        if !self.is_card(&card) {
            return Err(Error::new(ErrorKind::InvalidInput, "Card not in deck"));
        }
        let value = match Deck::parse_value(&card) {
            Ok(value) => value,
            Err(e) => return Err(e),
        };
        self.cards.insert(card, true);
        let count = match self.values.get_mut(&value) {
            Some(count) => count,
            None => {
                return Err(Error::new(
                    ErrorKind::InvalidInput,
                    "Card value not in deck",
                ))
            }
        };
        *count = *count + 1;
        self.size = self.size + 1;
        Ok(())
    }

    pub fn remove(&mut self, card: &String) -> Result<(), Error> {
        let value = match Deck::parse_value(card) {
            Ok(value) => value,
            Err(e) => return Err(e),
        };
        self.cards.insert(card.clone(), false);
        let count = match self.values.get_mut(&value) {
            Some(count) => count,
            None => return Err(Error::new(ErrorKind::InvalidInput, "Card not in deck")),
        };
        *count = *count - 1;
        self.size = self.size - 1;
        Ok(())
    }

    pub fn calc(&self, card: &String) -> Result<(f32, f32, f32), Error> {
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

    pub fn format_card_chance(&self, card: &String) -> String {
        let (higher, equal, lower) = self.calc(card).unwrap();
        format!(
            "{}▲ {:.2} {}◀▶ {:.2} {}▼ {:.2}{}",
            color::Fg(color::Green),
            higher,
            color::Fg(color::Reset),
            equal,
            color::Fg(color::Blue),
            lower,
            color::Fg(color::Reset)
        )
    }

    fn parse_value(card: &String) -> Result<usize, Error> {
        match card[1..].parse::<usize>() {
            Ok(card) => Ok(card),
            Err(_) => return Err(Error::new(ErrorKind::InvalidInput, "Invalid card value")),
        }
    }
}

pub struct Table {
    pub rows: Vec<Row>,
}

impl Table {
    pub fn new(row_count: usize, cards: Vec<String>) -> Result<Table, Error> {
        if row_count != cards.len() {
            return Err(Error::new(ErrorKind::InvalidInput, "Invalid card amount"));
        }
        let mut rows = Vec::new();
        for i in 0..row_count {
            let card = cards.get(i).unwrap().clone();
            rows.push(Row::new(card));
        }
        Ok(Table { rows })
    }

    // TODO unit test for this
    pub fn has_row(&self, row_num: usize) -> bool {
        match self.rows.get(row_num) {
            Some(_) => true,
            None => false,
        }
    }

    pub fn print(&self, deck: &Deck) {
        print!("{}", cursor::Goto(1, 1));
        for row in self.rows.iter() {
            println!(
                "{}{}\t---\t{}\t---\t{}\n{}",
                clear::CurrentLine,
                deck.format_card_chance(row.get_left()),
                row,
                deck.format_card_chance(row.get_right()),
                clear::CurrentLine,
            )
        }
    }
}

pub struct Row {
    cards: Vec<String>,
}

impl Row {
    fn new(card: String) -> Row {
        Row { cards: vec![card] }
    }

    pub fn len(&self) -> usize {
        self.cards.len()
    }

    pub fn get_left(&self) -> &String {
        return self.cards.get(0).unwrap();
    }

    pub fn add_left(&mut self, card: String) {
        self.cards.insert(0, card);
    }

    // TODO unit test
    pub fn remove_left(&mut self, deck: &mut Deck) {
        let card = self.cards.remove(0);
        deck.add(card).unwrap();
    }

    pub fn get_right(&self) -> &String {
        return self.cards.get(self.cards.len() - 1).unwrap();
    }

    pub fn add_right(&mut self, card: String) {
        self.cards.push(card)
    }

    // TODO unit test
    pub fn remove_right(&mut self, deck: &mut Deck) {
        let card = self.cards.pop().unwrap();
        deck.add(card).unwrap();
    }

    pub fn collapse(&mut self, card: String, deck: &mut Deck) {
        for c in self.cards.drain(..) {
            deck.add(c).unwrap();
        }
        self.cards.push(card);
    }

    fn format_card(card: &String) -> String {
        let mut red = false;
        let (suit, value) = card.split_at(1);
        let suit = match suit {
            "a" => "♣",
            "b" => "♠",
            "c" => {
                red = true;
                "♥"
            }
            "d" => {
                red = true;
                "♦"
            }
            _ => "",
        };
        let value = match value {
            "14" => "A",
            "13" => "K",
            "12" => "Q",
            "11" => "J",
            value => value,
        };
        let padding = match value.len() {
            1 => " ",
            _ => "",
        };
        if !red {
            return format!("[{} {}{}]", suit, padding, value);
        }
        format!(
            "{}[{} {}{}]{}",
            color::Fg(color::Red),
            suit,
            padding,
            value,
            color::Fg(color::Reset)
        )
    }
}

impl fmt::Display for Row {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut cards = self.cards.iter();
        let card = cards.next();
        let mut fmt_string = Row::format_card(card.unwrap());
        loop {
            match cards.next() {
                Some(c) => fmt_string = format!("{} {}", fmt_string, Row::format_card(c)),
                None => break,
            };
        }
        write!(f, "{}", fmt_string)
    }
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
        deck.remove(&card).unwrap();
        assert_eq!(deck.size, 35);
        let card = deck.cards.get(&card).unwrap();
        assert!(!card);
    }

    #[test]
    fn deck_can_calculate_chance() {
        let mut deck = Deck::new(8).unwrap();
        let card = String::from("a14");
        deck.remove(&card).unwrap();
        let (higher, equal, lower) = deck.calc(&card).unwrap();
        assert_eq!(higher, 0.0);
        assert_eq!(equal, 3.0 / 7.0);
        assert_eq!(lower, 4.0 / 7.0);
    }

    #[test]
    fn deck_can_format_card_chance() {
        let mut deck = Deck::new(8).unwrap();
        let card = String::from("a14");
        deck.remove(&card).unwrap();
        let (higher, equal, lower) = deck.calc(&card).unwrap();
        assert_eq!(
            deck.format_card_chance(&card),
            format!(
                "{}▲ {:.2} {}◀▶ {:.2} {}▼ {:.2}{}",
                color::Fg(color::Green),
                higher,
                color::Fg(color::Reset),
                equal,
                color::Fg(color::Blue),
                lower,
                color::Fg(color::Reset)
            )
        )
    }

    #[test]
    fn deck_can_parse_card_values() {
        let cards = [String::from("a1"), String::from("b14"), String::from("bb")];
        assert_eq!(Deck::parse_value(&cards[0]).unwrap(), 1);
        assert_eq!(Deck::parse_value(&cards[1]).unwrap(), 14);
        let c3 = match Deck::parse_value(&cards[2]) {
            Ok(_) => false,
            Err(_) => true,
        };
        assert!(c3);
    }

    #[test]
    fn deck_can_check_card_validity() {
        let deck = Deck::new(52).unwrap();
        let cards = [
            String::from("a10"),
            String::from("b20"),
            String::from("14"),
            String::from("!14"),
            String::from("f10"),
        ];
        assert!(deck.is_card(&cards[0]));
        for card in cards[1..].iter() {
            assert!(!deck.is_card(&card));
        }
    }

    #[test]
    fn deck_can_check_card_presence() {
        let mut deck = Deck::new(8).unwrap();
        let cards = [
            String::from("a14"),
            String::from("b10"),
            String::from("c13"),
        ];
        deck.remove(&cards[0]).unwrap();
        assert!(!deck.has_card(&cards[0]));
        assert!(!deck.has_card(&cards[1]));
        assert!(deck.has_card(&cards[2]));
    }

    #[test]
    fn table_can_be_created() {
        let cards = vec![String::from("a1"), String::from("b2"), String::from("c3")];
        let table = Table::new(3, cards.clone()).unwrap();
        for i in 0..3 {
            assert_eq!(
                table.rows.get(i).unwrap().cards.get(0).unwrap(),
                cards.get(i).unwrap()
            )
        }
    }

    #[test]
    fn row_can_add_cards() {
        let card1 = String::from("a1");
        let mut row = Row::new(card1.clone());
        let card_left = String::from("c4");
        let card_right = String::from("d10");
        row.add_left(card_left.clone());
        row.add_right(card_right.clone());
        assert_eq!(row.cards, vec![card_left, card1, card_right]);
    }

    #[test]
    fn row_can_collapse_into_deck() {
        let mut deck = Deck::new(16).unwrap();
        let cards = [
            String::from("a14"),
            String::from("b13"),
            String::from("c13"),
        ];
        deck.remove(&cards[0]).unwrap();
        deck.remove(&cards[1]).unwrap();
        let mut row = Row::new(cards[0].clone());
        row.add_right(cards[1].clone());
        assert_eq!(deck.size, 14);
        assert!(!deck.has_card(&cards[0]));
        assert!(!deck.has_card(&cards[1]));
        assert_eq!(&row.cards, &vec![cards[0].clone(), cards[1].clone()]);
        deck.remove(&cards[2]).unwrap();
        row.collapse(cards[2].clone(), &mut deck);
        assert_eq!(deck.size, 15);
        assert!(deck.cards.get(&cards[0]).unwrap());
        assert!(deck.cards.get(&cards[1]).unwrap());
    }

    #[test]
    fn row_formats_cards_correctly() {
        let cards = [
            String::from("a4"),
            String::from("b10"),
            String::from("c12"),
            String::from("d14"),
        ];
        assert_eq!(Row::format_card(&cards[0]), String::from("[♣  4]"));
        assert_eq!(Row::format_card(&cards[1]), String::from("[♠ 10]"));
        assert_eq!(
            Row::format_card(&cards[2]),
            format!("{}[♥  Q]{}", color::Fg(color::Red), color::Fg(color::Reset))
        );
        assert_eq!(
            Row::format_card(&cards[3]),
            format!("{}[♦  A]{}", color::Fg(color::Red), color::Fg(color::Reset))
        );
    }

    #[test]
    fn row_formats_correctly() {
        let cards = [String::from("a4"), String::from("b3"), String::from("c12")];
        let mut row = Row::new(cards[1].clone());
        row.add_left(cards[0].clone());
        row.add_right(cards[2].clone());
        assert_eq!(
            format!("{}", row),
            format!(
                "{} {} {}",
                Row::format_card(row.cards.get(0).unwrap()),
                Row::format_card(row.cards.get(1).unwrap()),
                Row::format_card(row.cards.get(2).unwrap())
            )
        );
    }
}
