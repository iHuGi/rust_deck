use rand::{thread_rng, seq::SliceRandom};

/// A collection of playing cards stored as a growable array of Strings in the Heap.
#[derive(Debug)]
struct Deck {
    cards: Vec<String>,
}

impl Deck {
    /// Associated function to generate a standard 24-card deck.
    /// Allocates a new block of memory on the Heap for the cards vector.
    fn new() -> Self {
        let suits = ["Hearts", "Spades", "Diamonds", "Clubs"];
        let values = ["Ace", "Two", "Three", "Four", "Five", "Six"];

        let mut cards = vec![];

        for suit in suits {
            for value in values {
                // format! allocates a new String on the Heap for every card.
                let card = format!("{} of {}", value, suit);
                cards.push(card);
            }
        }

        // Returns ownership of the Heap-allocated Deck back to the caller.
        Deck { cards }
    }

    /// Shuffles the cards in-place.
    /// Uses a mutable borrow (&mut) to rearrange memory without new allocations.
    /// High-performance: No data is copied, just the internal pointers are swapped.
    fn shuffle(&mut self) {
        let mut rng = thread_rng();
        self.cards.shuffle(&mut rng);
    }

    /// Deals cards by splitting the RAM block at a specific index.
    /// Moves ownership of the 'tail' portion of the vector to a new variable.
    /// The original 'cards' vector is shrinked in-place. 
    fn deal(&mut self, num_cards: usize) -> Vec<String> {
        // split_off is a zero-copy move of the elements' pointers.
        // WARNING: Panic-prone if num_cards > cards.len()!
        self.cards.split_off(self.cards.len() - num_cards) 
    }
}

fn main() {
    // 1. Initialize a mutable deck (Owner of the Heap memory block).
    let mut deck = Deck::new();

    // 2. Randomize the pointers within the existing memory block.
    deck.shuffle();
    
    // 3. Move a slice of the memory block into 'hand'.
    // Deck 'shrinks' and Hand 'grows' with the transferred card pointers.
    let hand = deck.deal(3);
    
    // 4. Output results using the Debug formatter.
    println!("--- Your Hand (Moved Memory) ---");
    println!("{:#?}", hand);

    println!("\n--- Remaining Deck ({} cards left in block) ---", deck.cards.len());
    println!("{:#?}", deck); 
}