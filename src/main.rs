use std::error::Error;

use hearthstone::{Card, CardClass, Game};

fn main() -> Result<(), Box<dyn Error>> {
    let mut reader = csv::Reader::from_path("data/cards.csv")?;
    let cards = reader
        .deserialize::<Option<Card>>()
        .filter_map(|c| {
            c.unwrap()
                .filter(|c| c.card_class == CardClass::Neutral && c.text.is_empty())
        })
        .collect();
    let game = Game::new(cards);
    let match_ = game.create_match();
    let player = match_.current_player();
    println!("Current player: {:?}", player);
    player.end_turn();
    println!("End turn");
    println!("Current player: {:?}", match_.current_player());
    println!("Opponent player: {:?}", match_.current_player().opponent());
    println!("Original player: {:?}", player);
    println!("Opponent of original player: {:?}", player.opponent());
    Ok(())
}
