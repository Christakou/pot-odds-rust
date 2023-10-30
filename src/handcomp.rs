

mod handcomp {
    use deck::Deck;
    use std::collections::hash_map::RandomState;
    use std::collections::HashMap;

    pub struct HandComparer {
        pub first_hand: Deck,
        pub second_hand: Deck,
    }

    impl HandComparer {}

    pub struct HandCounter {
        pub hand: Deck,
        pub suit_map: HashMap<String, u8>,
        pub rank_map: HashMap<u8, u8>,
    }

    impl HandCounter {
        fn new(deck: Deck) -> Self {
            let _s1 = RandomState::new();
            let _s2 = RandomState::new();
            let mut suit_map: HashMap<String, u8> = HashMap::with_hasher(_s1);
            let mut rank_map: HashMap<u8, u8> = HashMap::with_hasher(_s2);
            for card in &deck.cards {
                let s: String = card.suit.clone();
                *suit_map.entry(s).or_insert(0) += 1;
                *rank_map.entry(card.rank).or_insert(0) += 1;
            }

            return HandCounter {
                hand: deck,
                suit_map,
                rank_map,
            };
        }
        fn print(self) {
            println!("Counting Hand");
            for (suit, count) in self.suit_map {
                println!("Suit: {} Count: {}", suit, count);
            }
            for (rank, count) in self.rank_map {
                println!("Rank: {} Count: {}", rank, count);
            }
        }
    }
}
