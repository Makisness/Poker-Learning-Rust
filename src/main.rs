mod utils;
mod probability;
use std::io;
use crate::utils::*;
use crate::probability::*;

fn main() {

    //Initialize the shuffled deck, hands, and community
    let mut deck_int = create_shuffled_deck_int();
    let mut p1_hand: Vec<u8> = Vec::new();
    let mut p2_hand: Vec<u8> = Vec::new();
    let mut community_cards: Vec<u8> = Vec::new();

    //draw the starting hands for each player
    draw_hand(&mut p1_hand, &mut deck_int);
    draw_hand(&mut p2_hand, &mut deck_int);

    draw_community(&mut community_cards, &mut deck_int);

    println!("--Welcome to Simplified Poker!--\nPlease enter the name of player 1: ");

    let mut name_p1 = String::new();
    let _ = io::stdin().read_line(&mut name_p1);
    name_p1 = name_p1.trim().to_string();
    println!("\nPlease enter the name of player 2: ");
    let mut name_p2 = String::new();
    let _ = io::stdin().read_line(&mut name_p2);
    name_p2 = name_p2.trim().to_string();

    println!("\n{} has ",name_p1);
    for card in convert_to_cards(&p1_hand){
        println!("{}", card);
    }
    println!("\n{} has ",name_p2);
    for card in convert_to_cards(&p2_hand){
        println!("{}", card);
    }

    // Reveal community cards one by one

    let mut revealed_cards = Vec::new();
    let mut turn = 0;
    for card in convert_to_cards(&community_cards) {

        //Compute win probability
        monte_carlo_simulation(p1_hand.clone(), p2_hand.clone(), community_cards.clone(), deck_int.clone(), turn,7500);
        turn += 1;


        println!("\nPress Enter to reveal the next community card...");
        let _ = io::stdin().read_line(&mut String::new());
        revealed_cards.push(card);
        println!(
            "Community Cards: {}",
            revealed_cards
                .iter()
                .map(|card| card.to_string())
                .collect::<Vec<String>>()
                .join(", ")
        );
    }
    //Best Hand Wins - return GameResult Struct, includes info about the winner and cards
    let result = compare_hands(&*name_p1, p1_hand, &*name_p2, p2_hand, community_cards);

    println!("Winner: {}\nHand: {}",result.winner.unwrap().to_string(), result.win_condition.to_string());

}
