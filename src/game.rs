

#[derive(PartialEq, Eq)]
enum Card {
    Value(u8, Colour),
    Draw(u8, Colour),
    Reverse(Colour),
    Skip(Colour),
    Wild(Colour),
    Blank(Colour),
    None
}

impl Card {
    fn get_colour(&self) -> &Colour {
        match self {
            Card::Value(_, c) => c,
            Card::Draw(_, c) => c,
            Card::Reverse(c) => c,
            Card::Skip(c) => c,
            Card::Wild(c) => c,
            Card::Blank(c) => c,
            Card::None => &Colour::None
        }
    }

    fn get_value(&self) -> Option<u8> {
        match self {
            Card::Value(n, _) => {
                Some(*n)
            }
            Card::Draw(n, _) => {
                Some(*n)
            }
            _ => None
        }
    }
}

#[derive(PartialEq, Eq, Clone, Copy)]
enum Colour {
    Green,
    Purple,
    Red,
    Yellow,
    None
}


#[derive(PartialEq, Eq)]
enum Direction {
    Clockwise,
    CounterClockwise
}

struct Whono<'a> {
    current_player_index: usize,
    direction: Direction,
    players: Vec<Player<'a>>,
    deck: Vec<Card>,
    discard: Vec<Card>,
}

struct Player<'a> {
    id: u16,
    name: &'a str,
    cards: Vec<Card>,
}

impl<'a> Player<'a> {
    pub fn new(id: u16, username: &'a str, deck_size: usize) -> Self {
        let cards = Vec::with_capacity(deck_size);
        Self {
            id,
            name: username,
            cards,
        }
    }
}

impl<'a> Whono<'a> {
    pub fn new(first_player_id: u16, first_player_name: &'a str, amount_of_players: usize, deck_size: usize) -> Self {
        let mut players = Vec::with_capacity(amount_of_players);
        players[0] = Player::new(first_player_id, first_player_name, deck_size);
        Self {
            current_player_index: 0,
            direction: Direction::Clockwise,
            players: players,
            deck: Vec::with_capacity(deck_size),
            discard: Vec::with_capacity(deck_size),
        }
    }

    // Game runs clockwise by default, from index 0 to the amount of players and back
    fn increment_turn(&mut self) {
        if self.direction == Direction::Clockwise {
            if self.current_player_index == (self.players.len() - 1) {
                self.current_player_index = 0;
            } else {
                self.current_player_index += 1;
            }
        } else {
            if self.current_player_index == 0 {
                self.current_player_index = self.players.len() - 1;
            } else {
                self.current_player_index -= 1;
            }
        }
    }

    fn draw_cards(&mut self, amount: u16) {
        for i in 0..amount {
            self.players[self.current_player_index].cards.push(self.deck.pop().unwrap());
        }
    }

    fn discard_card(&mut self, card_index: usize, wild_colour: Option<Colour>) {
        let card = &self.players[self.current_player_index].cards[card_index];
        let top_discard_card = &self.discard[self.discard.len() -1];
        let matched = match top_discard_card {
            Card::Value(num, c) => {
                card.get_colour() == c || (card.get_value().is_some() && (card.get_value().unwrap() == *num))
            },
            Card::Draw(num, c) => {
                card.get_colour() == c || (card.get_value().is_some() && (card.get_value().unwrap() == *num))
            },
            Card::Reverse(c) => card.get_colour() == c,
            Card::Skip(c) => card.get_colour() == c,
            Card::Wild(c) => card.get_colour() == c,
            Card::Blank(c) => card.get_colour() == c,
            _ => false
        };
        if let Some(colour) = wild_colour {
            self.players[self.current_player_index].cards[card_index] = Card::Blank(colour);
            self.discard.push(self.players[self.current_player_index].cards.remove(card_index));
        } else if matched {
            if card == &Card::Reverse(*card.get_colour()) {
                self.reverse_direction();
            }
            self.discard.push(self.players[self.current_player_index].cards.remove(card_index));
        }
    }

    fn reverse_direction(&mut self) {
        match self.direction {
            Direction::Clockwise => {
                self.direction = Direction::CounterClockwise;
            }
            Direction::CounterClockwise => {
                self.direction = Direction::Clockwise;
            }
        }
    }
}