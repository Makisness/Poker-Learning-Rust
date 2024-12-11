use std::collections::HashMap;
use rand::*;
use std::fmt;
use std::str;


pub struct Card{
    suit: Suit,
    rank: Rank,
}
pub struct GameResult {
    pub(crate) winner: Option<String>,  // None if it's a tie
    winning_rank: u8,        // Rank of the winning hand
    pub(crate) win_condition: String, // Description of the win condition
    pub(crate)winning_player: u8,
}
pub struct player{
    name: String,
    hand: Vec<u8>,
    chips: u8,
    folded: bool,
}

impl fmt::Display for GameResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.winner {
            Some(winner) => write!(
                f,
                "Winner: {}\nWinning Rank: {}\nWin Condition: {}",
                winner, self.winning_rank, self.win_condition,
            ),
            None => write!(
                f,
                "It's a Tie!\nWinning Rank: {}\nWin Condition: {}",
                self.winning_rank, self.win_condition,
            ),
        }
    }
}
impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} of {}", self.rank, self.suit)
    }
}
impl fmt::Display for Suit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let suit_str = match self {
            Suit::Hearts => "Hearts",
            Suit::Diamonds => "Diamonds",
            Suit::Clubs => "Clubs",
            Suit::Spades => "Spades",
        };
        write!(f, "{}", suit_str)
    }
}

impl fmt::Display for Rank {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let rank_str = match self {
            Rank::Ace => "Ace",
            Rank::Two => "Two",
            Rank::Three => "Three",
            Rank::Four => "Four",
            Rank::Five => "Five",
            Rank::Six => "Six",
            Rank::Seven => "Seven",
            Rank::Eight => "Eight",
            Rank::Nine => "Nine",
            Rank::Ten => "Ten",
            Rank::Jack => "Jack",
            Rank::Queen => "Queen",
            Rank::King => "King",
        };
        write!(f, "{}", rank_str)
    }
}

enum Suit {
    Clubs,
    Diamonds,
    Hearts,
    Spades,
}

enum Rank {
    Ace,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
}
fn get_suit(suit_index:u8) -> Suit {
    match suit_index {
        0 => Suit::Spades,
        1 => Suit::Hearts,
        2 => Suit::Diamonds,
        3 => Suit::Clubs,
        _ => panic!("invalid suit index"),
    }
}
fn get_rank(rank_index:u8) -> Rank {
    match rank_index {
        0 => Rank::Ace,
        1 => Rank::Two,
        2 => Rank::Three,
        3 => Rank::Four,
        4 => Rank::Five,
        5 => Rank::Six,
        6 => Rank::Seven,
        7 => Rank::Eight,
        8 => Rank::Nine,
        9 => Rank::Ten,
        10 => Rank::Jack,
        11 => Rank::Queen,
        12 => Rank::King,
        _ => panic!("invalid rank index"),
    }
}

fn get_rank_int(index:u8) -> u8{
    index%13
}
fn get_suit_int(index:u8) -> u8{
    index/13
}


//gets a card from an int. this finds where the card would be in a sorted deck by parsing the int
fn get_card(index: u8) -> Card{
    let c_suit = get_suit(get_suit_int(index));
    let c_rank = get_rank(get_rank_int(index));
    Card{suit: c_suit, rank: c_rank}
}

//takes a vec of random ints and converts it to a vec of cards.
pub fn convert_to_cards(card_indices:&Vec<u8>) -> Vec<Card>{
    let mut new_deck: Vec<Card> = Vec::new();
    for card in card_indices{
        let t_card = get_card(*card);
        new_deck.push(t_card);
    }
    new_deck
}

pub(crate) fn create_shuffled_deck_int() -> Vec<u8>{
    let mut deck: Vec<u8> = (0..52).collect();
    fisher_yates_shuffle(&mut deck);
    deck
}

//Shuffle algorithm. uses random numbers to swap 2 elements randomly. This happens once for every index.
pub fn fisher_yates_shuffle(deck: &mut [u8]){
    let mut rng = rand::thread_rng();
    let len = deck.len();
    for i in 0..len {
        let j = rng.gen_range(i..len);
        deck.swap(i, j);
    }
}

pub(crate) fn draw_hand(player_hand: &mut Vec<u8>, deck: &mut Vec<u8>){
    if let Some(card) = deck.pop(){
        player_hand.push(card);
    }
    if let Some(card) = deck.pop(){
        player_hand.push(card);
    }
}
pub(crate) fn draw_community(community_hand: &mut Vec<u8>, deck: &mut Vec<u8>){
    if let Some(card) = deck.pop(){
        community_hand.push(card);
    }
    if let Some(card) = deck.pop(){
        community_hand.push(card);
    }
    if let Some(card) = deck.pop(){
        community_hand.push(card);
    }
    if let Some(card) = deck.pop(){
        community_hand.push(card);
    }
    if let Some(card) = deck.pop(){
        community_hand.push(card);
    }
}

pub(crate) fn draw_community_monte(community_hand: &mut Vec<u8>, deck: &mut Vec<u8>, turns:u8) {
    for _i in 0..5-turns {
        if let Some(card) = deck.pop() {
            community_hand.push(card);
        }
    }
}
    fn count_occurrences<T: std::cmp::Eq + std::hash::Hash + Clone>(items: &[T]) -> HashMap<T, usize> {
    let mut counts = HashMap::new();
    for item in items.iter() {
        *counts.entry(item.clone()).or_insert(0) += 1;
    }
    counts
}
fn is_consecutive(cards: &[u8]) -> bool {
    let mut ranks: Vec<u8> = cards.iter().map(|&card| get_rank_int(card)).collect();

    // Check for standard consecutive ranks
    ranks.sort_unstable();
    if ranks.windows(5).any(|window| window.windows(2).all(|pair| pair[1] == pair[0] + 1)) {
        return true;
    }

    // Special case: Aces as low
    if ranks.contains(&0) {
        let mut ace_low = ranks.clone();
        ace_low.retain(|&rank| rank != 0); // Remove Ace
        ace_low.push(13);                 // Treat Ace as high
        ace_low.sort_unstable();

        return ace_low.windows(5).any(|window| window.windows(2).all(|pair| pair[1] == pair[0] + 1));
    }

    false
}

pub fn combine_hand_and_community(hand: &Vec<u8>, community: &Vec<u8>) -> Vec<u8> {
    let mut combined = hand.clone();
    combined.extend(community);
    combined
}
fn evaluate_hand(cards: &Vec<u8>) -> u8 {
    let ranks = cards.iter().map(|&card| get_rank_int(card)).collect::<Vec<_>>();
    let suit_counts = count_occurrences(&cards.iter().map(|&card| get_suit_int(card)).collect::<Vec<_>>());
    let rank_counts = count_occurrences(&ranks);

    // Check for Royal Flush
    if suit_counts.values().any(|&count| count >= 5) {
        let mut royal_flush_ranks = vec![10, 11, 12, 13, 0]; // 0 represents Ace
        royal_flush_ranks.sort_unstable();
        for (&suit, &count) in suit_counts.iter() {
            if count >= 5 {
                let flush_ranks: Vec<u8> = cards
                    .iter()
                    .filter(|&&card| get_suit_int(card) == suit)
                    .map(|&card| get_rank_int(card))
                    .collect();
                if royal_flush_ranks.iter().all(|rank| flush_ranks.contains(rank)) {
                    return 10; // Royal Flush has the highest rank
                }
            }
        }
    }

    // Straight Flush
    if suit_counts.values().any(|&count| count >= 5) && is_consecutive(cards) {
        return 9;
    }

    // Four of a Kind
    if rank_counts.values().any(|&count| count == 4) {
        return 8;
    }

    // Full House
    if rank_counts.values().any(|&count| count == 3)
        && rank_counts.values().any(|&count| count == 2)
    {
        return 7;
    }

    // Flush
    if suit_counts.values().any(|&count| count >= 5) {
        return 6;
    }

    // Straight
    if is_consecutive(cards) {
        return 5;
    }

    // Three of a Kind
    if rank_counts.values().any(|&count| count == 3) {
        return 4;
    }

    // Two Pair
    if rank_counts.values().filter(|&&count| count == 2).count() == 2 {
        return 3;
    }

    // One Pair
    if rank_counts.values().any(|&count| count == 2) {
        return 2;
    }

    // High Card
    1
}

pub fn compare_hands(p1_name: &str, p1_hand: Vec<u8>, p2_name: &str, p2_hand: Vec<u8> , community_cards: Vec<u8>) -> GameResult {
    let p1_combined = combine_hand_and_community(&p1_hand, &community_cards);
    let p2_combined = combine_hand_and_community(&p2_hand, &community_cards);

    let p1_rank = evaluate_hand(&p1_combined);
    let p2_rank = evaluate_hand(&p2_combined);

    match p1_rank.cmp(&p2_rank) {
        std::cmp::Ordering::Greater => GameResult {
            winner: Some(p1_name.to_string()),
            winning_rank: p1_rank,
            win_condition: hand_rank_to_string(p1_rank),
            winning_player: 1,
        },
        std::cmp::Ordering::Less => GameResult {
            winner: Some(p2_name.to_string()),
            winning_rank: p2_rank,
            win_condition: hand_rank_to_string(p2_rank),
            winning_player: 2,
        },
        std::cmp::Ordering::Equal => tie_breaker(p1_name, p1_hand, p2_name, p2_hand, p1_rank, community_cards),
    }
}

fn tie_breaker(p1_name: &str, p1_hand: Vec<u8>, p2_name: &str, p2_hand: Vec<u8>, hand_rank: u8,community_cards:Vec<u8>) -> GameResult {
    let p1_combined = combine_hand_and_community(&p1_hand, &community_cards);
    let p2_combined = combine_hand_and_community(&p2_hand, &community_cards);

    let winner = match hand_rank {
        9 | 5 => compare_high_card(p1_name, &p1_combined, p2_name, &p2_combined), // Straight Flush, Straight
        6 => compare_high_card(p1_name, &p1_combined, p2_name, &p2_combined), // Flush
        7 | 8 => compare_by_rank_counts(p1_name, &p1_combined, p2_name, &p2_combined, vec![3, 2]), // Full House, Four of a Kind
        4 => compare_by_rank_counts(p1_name, &p1_combined, p2_name, &p2_combined, vec![3]), // Three of a Kind
        3 => compare_by_rank_counts(p1_name, &p1_combined, p2_name, &p2_combined, vec![2, 2]), // Two Pair
        2 => compare_by_rank_counts(p1_name, &p1_combined, p2_name, &p2_combined, vec![2]), // One Pair
        1 => compare_high_card(p1_name, &p1_combined, p2_name, &p2_combined), // High Card
        _ => None, // Should not happen
    };

    match winner {
        Some(winner_name) => GameResult {
            winner: Some(winner_name.clone()),
            winning_rank: hand_rank,
            win_condition: hand_rank_to_string(hand_rank),
            winning_player: get_winning_player_num(Option::from(winner_name),p1_name),
        },
        None => GameResult {
            winner: None,
            winning_rank: hand_rank,
            win_condition: "Tie".to_string(),
            winning_player: 0,
        },
    }
}

fn hand_rank_to_string(rank: u8) -> String {
    match rank {
        9 => "Straight Flush".to_string(),
        8 => "Four of a Kind".to_string(),
        7 => "Full House".to_string(),
        6 => "Flush".to_string(),
        5 => "Straight".to_string(),
        4 => "Three of a Kind".to_string(),
        3 => "Two Pair".to_string(),
        2 => "One Pair".to_string(),
        1 => "High Card".to_string(),
        _ => "Unknown".to_string(),
    }
}
fn compare_high_card(p1_name: &str, p1: &Vec<u8>, p2_name: &str, p2: &Vec<u8>) -> Option<String> {
    let mut p1_ranks: Vec<u8> = p1.iter().map(|&card| get_rank_int(card)).collect();
    let mut p2_ranks: Vec<u8> = p2.iter().map(|&card| get_rank_int(card)).collect();

    p1_ranks.sort_unstable_by(|a, b| b.cmp(a)); // Descending order
    p2_ranks.sort_unstable_by(|a, b| b.cmp(a)); // Descending order

    for (r1, r2) in p1_ranks.iter().zip(p2_ranks.iter()) {
        match r1.cmp(r2) {
            std::cmp::Ordering::Greater => return Some(p1_name.to_string()),
            std::cmp::Ordering::Less => return Some(p2_name.to_string()),
            std::cmp::Ordering::Equal => continue,
        }
    }
    None // It's a tie
}
fn get_winning_player_num(winner_name: Option<String>,p1_name:&str) -> u8{
    if winner_name == Some(p1_name.parse().unwrap()) {
        1
    }else{
        2
    }
}
fn compare_by_rank_counts(p1_name: &str, p1: &Vec<u8>, p2_name: &str, p2: &Vec<u8>, group_priorities: Vec<usize>) -> Option<String> {
    let p1_counts = count_occurrences(&p1.iter().map(|&card| get_rank_int(card)).collect::<Vec<_>>());
    let p2_counts = count_occurrences(&p2.iter().map(|&card| get_rank_int(card)).collect::<Vec<_>>());

    let mut p1_groups: Vec<u8> = p1_counts.iter()
        .filter(|(_, &count)| group_priorities.contains(&count))
        .map(|(&rank, _)| rank)
        .collect();

    let mut p2_groups: Vec<u8> = p2_counts.iter()
        .filter(|(_, &count)| group_priorities.contains(&count))
        .map(|(&rank, _)| rank)
        .collect();

    p1_groups.sort_unstable_by(|a, b| b.cmp(a)); // Descending
    p2_groups.sort_unstable_by(|a, b| b.cmp(a)); // Descending

    for (g1, g2) in p1_groups.iter().zip(p2_groups.iter()) {
        match g1.cmp(g2) {
            std::cmp::Ordering::Greater => return Some(p1_name.to_string()),
            std::cmp::Ordering::Less => return Some(p2_name.to_string()),
            std::cmp::Ordering::Equal => continue,
        }
    }

    // If groups are equal, fall back to high card
    compare_high_card(p1_name, p1, p2_name, p2)
}


