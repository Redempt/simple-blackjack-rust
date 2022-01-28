use rand::thread_rng;
use rand::seq::SliceRandom;
use std::io::{stdin, BufRead, BufReader};

type Deck<'a> = Vec<&'a str>;

fn calculate_score(hand: &Deck) -> i32 {
    let mut score = 0;
    let mut ace = false;
    hand.iter().for_each(|card| {
        match *card {
            "A" => {
                score += 11;
                ace = true;
            }
            "J" | "Q" | "K" => score += 10,
            _ => score += card.parse::<i32>().unwrap()
        }
    });
    if score > 21 && ace {
        score -= 10;
    }
    score
}

fn draw<'a>(hand: &mut Deck<'a>, deck: &mut Deck<'a>) {
    hand.push(deck.remove(0));
}

fn print_player(name: &str, hand: &Deck) {
    println!("\n{}'s total is: {}", name, calculate_score(hand));
    println!("{}", hand.join(", "));
}

fn print_status(player_cards: &Vec<&str>, dealer_cards: &Vec<&str>) {
    vec!["Player", "Dealer"].iter().zip(vec![player_cards, dealer_cards].iter()).for_each(|(name, hand)| print_player(name, hand));
    println!();
}

fn main() {
    let mut deck = vec!["A", "2", "3", "4", "5", "6", "7", "8", "9", "J", "Q", "K"];
    deck.shuffle(&mut thread_rng());
    let (mut player_hand, mut dealer_hand): (Deck, Deck) = (vec![], vec![]);

    println!("Dealer draws first card.");
    draw(&mut dealer_hand, &mut deck);

    println!("Player receives two cards.");
    (0..2).for_each(|_| draw(&mut player_hand, &mut deck));

    print_status(&player_hand, &dealer_hand);

    let mut reader = BufReader::new(stdin());
    loop {
        println!("Do you want to (H)it, (S)tay, or (Q)uit?");
        let mut line = String::new();
        reader.read_line(&mut line).unwrap();
        
        match line.trim_end().to_uppercase().as_str() {
            "S" => break,
            "Q" => return,
            "H" => {
                draw(&mut player_hand, &mut deck);
                print_status(&player_hand, &dealer_hand);
                if calculate_score(&player_hand) > 21 {
                    println!("You busted! You lose!");
                    return
                }
            }
            _ => println!("Invalid selection.")
        }
    }

    println!("Dealer draws rest of cards.");
    while calculate_score(&dealer_hand) < 17 {
        draw(&mut dealer_hand, &mut deck);
    }

    print_status(&player_hand, &dealer_hand);
    let scores = (calculate_score(&dealer_hand), calculate_score(&player_hand));
    if scores.0 > 21 {
        println!("Dealer busts! You win!");
        return
    }
    if scores.0 > scores.1 {
        println!("Dealer wins!");
        return
    }
    if scores.0 < scores.1 {
        println!("You win!");
        return
    }
    println!("It's a tie!");
}