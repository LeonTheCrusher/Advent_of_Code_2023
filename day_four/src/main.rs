use std::fs;

#[derive(Debug, Clone)]
struct Card {
    winning_numbers: Vec<i32>,
    my_numbers: Vec<i32>,
    points: i32,
    instances: i32,
}

fn main() {
    let scratch_cards = fs::read_to_string("cards.txt").unwrap();

    println!("{}", easy(&scratch_cards));
    println!("{}", hard(&scratch_cards));
}

fn easy(scratch_cards: &String) -> i32 {
    let mut cards = parse_cards(scratch_cards);
    check_winner_easy(&mut cards);

    let total_points = tally_points(&cards);

    total_points
}

fn hard(scratch_cards: &String) -> i32 {
    let mut cards = parse_cards(scratch_cards);
    check_winner_hard(&mut cards);
    let tally_instances = tally_instances(&cards);

    tally_instances
}

fn parse_cards(scratch_cards: &String) -> Vec<Card> {
    let mut cards: Vec<Card> = Vec::new();

    for line in scratch_cards.lines() {
        let current_card = line.split(":").collect::<Vec<&str>>()[1];
        let numbers = current_card.split("|").collect::<Vec<&str>>();

        let card = Card {
            winning_numbers: numbers[0]
                .split_whitespace()
                .map(|x| x.parse().unwrap())
                .collect(),
            my_numbers: numbers[1]
                .split_whitespace()
                .map(|x| x.parse().unwrap())
                .collect(),
            points: 0,
            instances: 1,
        };

        cards.push(card);
    }

    cards
}

fn check_winner_easy(cards: &mut Vec<Card>) {
    for card in cards.iter_mut() {
        let mut points = 0;
        let mut win_count: i32 = 0;
        for number in card.winning_numbers.iter() {
            if card.my_numbers.contains(number) && win_count == 0 {
                points += 1;
                win_count += 1;
            } else if card.my_numbers.contains(number) {
                points *= 2;
                win_count += 1;
            }
        }
        card.points = points
    }
}

fn check_winner_hard(cards: &mut Vec<Card>) {
    let len = cards.len();
    let mut copies_vec = vec![1; len];

    for (card_num, card) in cards.iter_mut().enumerate() {
        let mut win_count = 0;
        for number in card.winning_numbers.iter() {
            card.instances = copies_vec[card_num];
            if card.my_numbers.contains(number) {
                win_count += 1;
            }
        }
        for _ in 0..copies_vec[card_num] {
            for i in card_num + 1..card_num + 1 + win_count {
                copies_vec[i] += 1;
            }
        }
    }

    for (card_num, card) in cards.iter_mut().enumerate() {
        card.instances = copies_vec[card_num];
    }
}

fn tally_points(cards: &Vec<Card>) -> i32 {
    let mut total_points = 0;

    for card in cards.iter() {
        total_points += card.points;
    }

    total_points
}

fn tally_instances(cards: &Vec<Card>) -> i32 {
    let mut total = 0;

    for card in cards.iter() {
        total += card.instances;
    }

    total
}
