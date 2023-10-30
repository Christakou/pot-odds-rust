mod deck;
mod handcomp;

use deck::Deck;
use handcomp::HandCounter;

fn main() {

    println!("Hello, world!");
    let mut deck:Deck = Deck::new();
    deck.print();
    deck.shuffle();
    deck.print();
   
    let mut hand_a:Deck = deck.deal(5);
    let mut hand_b:Deck = deck.deal(5);

    hand_a.print();

    
    deck.print();
    let mut comparer = HandCounter::new(hand_a);
    comparer.print();
}

