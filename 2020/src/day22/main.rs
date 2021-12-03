use eyre::Result;
use std::collections::{HashSet, VecDeque};
use std::fs::read_to_string;
use std::{thread, time};

const DEBUG: bool = false;

enum CombatType {
    Standard,
    Recursive,
}

struct Combat {
    game_id: usize,
    game_type: CombatType,
    player1: PlayerDeck,
    player2: PlayerDeck,
    previous_states: HashSet<(PlayerDeck, PlayerDeck)>,
}

impl Combat {
    fn new(
        game_id: usize,
        game_type: CombatType,
        player1: PlayerDeck,
        player2: PlayerDeck,
    ) -> Self {
        Combat {
            game_id,
            game_type,
            player1,
            player2,
            previous_states: HashSet::new(),
        }
    }

    fn play_standard(&mut self) -> PlayerDeck {
        let deck_len = self.player1.deck.len() + self.player2.deck.len();
        while self.player1.deck.len() < deck_len && self.player2.deck.len() < deck_len {
            let (card1, card2) = self
                .player1
                .deck
                .iter()
                .zip(self.player2.deck.iter())
                .map(|(o, t)| (*o, *t))
                .next()
                .unwrap();
            self.player1.deck.pop_front();
            self.player2.deck.pop_front();
            if card1 > card2 {
                self.player1.deck.push_back(card1);
                self.player1.deck.push_back(card2);
            } else {
                self.player2.deck.push_back(card2);
                self.player2.deck.push_back(card1);
            }
        }
        if self.player1.deck.is_empty() {
            self.player2.clone()
        } else {
            self.player1.clone()
        }
    }

    fn should_recurse(&self, card1: usize, card2: usize) -> bool {
        self.player1.deck.len() > card1 && self.player2.deck.len() > card2
    }

    fn get_next_cards(&self) -> (usize, usize) {
        self.player1
            .deck
            .iter()
            .zip(self.player2.deck.iter())
            .map(|(o, t)| (*o, *t))
            .next()
            .unwrap()
    }

    fn print_game_state(&self, round_num: usize, card1: usize, card2: usize) {
        println!();
        println!("-- round {} game {} --", round_num, self.game_id);
        println!("player 1 deck! {:?}", self.player1.deck);
        println!("player 2 deck! {:?}", self.player2.deck);
        println!("player 1 plays {:?}", card1);
        println!("player 2 plays {:?}", card2);
    }

    fn play_recursive(&mut self) -> PlayerDeck {
        let deck_len = self.player1.deck.len() + self.player2.deck.len();
        let mut round_winner: PlayerDeck;
        let mut round_num = 1;
        while self.player1.deck.len() < deck_len && self.player2.deck.len() < deck_len {
            if DEBUG {
                let _sleep_time = time::Duration::from_millis(1000);
                thread::sleep(_sleep_time);
            }

            if self
                .previous_states
                .contains(&(self.player1.clone(), self.player2.clone()))
            {
                if DEBUG {
                    println!("seen this state before! Player 1 wins!");
                }
                return self.player1.clone();
            }
            self.previous_states
                .insert((self.player1.clone(), self.player2.clone()));
            let (card1, card2) = self.get_next_cards();
            if DEBUG {
                self.print_game_state(round_num, card1, card2);
            }
            round_num += 1;
            if self.should_recurse(card1, card2) {
                if DEBUG {
                    println!("going to play a sub game");
                }
                // recursive
                let recursive_player1 = PlayerDeck::new(
                    &self.player1.name,
                    self.player1
                        .deck
                        .iter()
                        .skip(1)
                        .take(card1)
                        .copied()
                        .collect(),
                );
                let recursive_player2 = PlayerDeck::new(
                    &self.player2.name,
                    self.player2
                        .deck
                        .iter()
                        .skip(1)
                        .take(card2)
                        .copied()
                        .collect(),
                );
                let mut recursive_game = Combat::new(
                    self.game_id + 1,
                    CombatType::Recursive,
                    recursive_player1,
                    recursive_player2,
                );
                round_winner = recursive_game.play();
            } else {
                // non-recursive
                round_winner = if card1 > card2 {
                    self.player1.clone()
                } else {
                    self.player2.clone()
                }
            }
            self.player1.deck.pop_front();
            self.player2.deck.pop_front();
            if DEBUG {
                println!("round winner: {:?}", round_winner);
            }
            match round_winner.name.as_str() {
                "Player 1:" => {
                    self.player1.deck.push_back(card1);
                    self.player1.deck.push_back(card2);
                }
                "Player 2:" => {
                    self.player2.deck.push_back(card2);
                    self.player2.deck.push_back(card1);
                }
                &_ => panic!(),
            }
        }
        if self.player1.deck.is_empty() {
            if DEBUG {
                println!("game winner! {:?}", self.player2);
            }
            self.player2.clone()
        } else {
            if DEBUG {
                println!("game winner! {:?}", self.player1);
            }
            self.player1.clone()
        }
    }

    fn play(&mut self) -> PlayerDeck {
        match self.game_type {
            CombatType::Standard => self.play_standard(),
            CombatType::Recursive => self.play_recursive(),
        }
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
struct PlayerDeck {
    name: String,
    deck: VecDeque<usize>,
}

impl PlayerDeck {
    fn from_raw(raw_player_str: &str) -> Self {
        let mut lines = raw_player_str.lines();
        let name = lines.next().unwrap();
        let deck = lines.map(|s| s.parse::<usize>().unwrap()).collect();
        Self::new(name, deck)
    }

    fn new(name: &str, deck: VecDeque<usize>) -> Self {
        PlayerDeck {
            name: name.to_string(),
            deck,
        }
    }
}

fn main() -> Result<()> {
    //let input = read_to_string("src/day22/input-sample2.txt")?;
    let input = read_to_string("src/day22/input.txt")?;
    let mut players = input.split("\n\n");
    let player1 = PlayerDeck::from_raw(players.next().unwrap());
    let player2 = PlayerDeck::from_raw(players.next().unwrap());
    let mut game = Combat::new(1, CombatType::Standard, player1.clone(), player2.clone());

    let winner = game.play();
    println!("winner: {:?}", winner);
    let score: usize = winner
        .deck
        .iter()
        .rev()
        .enumerate()
        // add one for scoring
        .map(|(i, &n)| (i + 1) * n)
        .sum();
    println!("score: {:?}", score);

    // part2
    let mut game = Combat::new(1, CombatType::Recursive, player1, player2);
    let winner = game.play();
    println!("winner: {:?}", winner);
    let score: usize = winner
        .deck
        .iter()
        .rev()
        .enumerate()
        // add one for scoring
        .map(|(i, &n)| (i + 1) * n)
        .sum();
    println!("score: {:?}", score);
    Ok(())
}
