pub mod deck {
    use card::Card;
    use rand::prelude::SliceRandom;
    use rand::thread_rng;

    pub struct Deck {
        pub cards: Vec<Card>,
    }

    impl Deck {
        fn new() -> Self {
            let mut cards: Vec<Card> = Vec::new();
            for suit in ["♦", "♠", "♣", "♥"] {
                for rank in 1..14 {
                    cards.push(Card {
                        suit: suit.to_string(),
                        rank,
                    });
                }
            }
            return Deck { cards };
        }

        fn shuffle(&mut self) {
            self.cards.shuffle(&mut thread_rng());
        }

        fn print(&self) {
            println!("Printing Deck!");
            for card in &self.cards {
                card.print();
                print!(" ");
            }
            println!();
        }

        fn deal(&mut self, n: u8) -> Deck {
            let mut resultant_vec: Vec<Card> = Vec::new();
            for _ in 0..n {
                let result = self.cards.pop().unwrap();
                resultant_vec.push(result);
            }
            return Deck {
                cards: resultant_vec,
            };
        }
    }
}
