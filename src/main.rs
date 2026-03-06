// Required for the compiler to format the custom struct for printing
#[derive(Debug)]
struct Deck {
    // A vector (growable list) of strings representing playing cards
    cards: Vec<String>,
}

fn main() {

    // List of suits
    let suits = ["Hearts", "Spades", "Diamonds", "Clubs"];
    
    // List of values
    let values = ["Ace", "Two", "Three", "Four", "Five", "Six"];

    // Needs to be 'mut' (mutable) to allow adding cards later in the loop
    let mut cards = vec![];

    // Double nested for loop to iterate through every combination
    for suit in suits {
        for value in values {
            // The format! macro automatically builds the string
            let card = format!("{} of {}", value, suit);
            
            // Add the new card to the vector
            cards.push(card);
        }
    }

    // Create a new instance of Deck and pass in the populated vector
    // (Shortcut: Just write `Deck { cards };` here)
    let deck = Deck { cards: cards }; 
    
    // Print the deck to the terminal using the pretty-print debug formatter {:#?}
    println!("Here is the deck: {:#?}", deck); 
}