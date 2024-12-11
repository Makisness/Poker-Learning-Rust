use crate::utils;
use crate::utils::{draw_community_monte};

pub fn monte_carlo_simulation(mut p1_hand: Vec<u8>, mut p2_hand: Vec<u8>, community_cards:Vec<u8>, mut deck:Vec<u8>, turn:u8, number_of_simulations:u32) -> Vec<f32>{
    // Get the cards that have been revealed
    let revealed_cards = &community_cards[0..turn as usize];
    let unrevealed_cards = &community_cards[turn as usize..=4];
    // Add these cards to the players hands
    p1_hand.extend_from_slice(revealed_cards);
    p2_hand.extend_from_slice(revealed_cards);
    deck.extend_from_slice(unrevealed_cards);

    let mut p1:u32 = 0;
    let mut p2:u32 = 0;

    for _i in 0..number_of_simulations {
        let mut deck = deck.clone();
        utils::fisher_yates_shuffle(&mut deck);
        let mut monte_test_community = Vec::new();
        draw_community_monte(&mut monte_test_community,&mut deck, turn);
        let winning_player = utils::compare_hands("",p1_hand.clone(),"",p2_hand.clone(),monte_test_community).winning_player;

        if winning_player == 1{
            p1 += 1;
        } else if winning_player == 2 {
            p2 += 1;
        }

    }
    let win_probability_p1:f32 = p1 as f32 / number_of_simulations as f32;
    let win_probability_p2:f32 = p2 as f32 / number_of_simulations as f32;

    let mut result: Vec<f32> = Vec::new();
    result.push(win_probability_p1);
    result.push(win_probability_p2);

    println!("\np1 win probability: {}%\np2 win probability: {}%\n",win_probability_p1*100.0,win_probability_p2*100.0);

    result
}