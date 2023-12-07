use std::collections::HashSet;
use std::fs;
use std::cmp::Ordering;

pub(crate) fn day_07() {
    let input: String = fs::read_to_string("./inputs/7.txt").unwrap();
    println!("day 07:");
    println!("  part 1: {}", part_1(input.clone()));
    println!("  part 2: {}", part_2(input.clone()));
}

pub fn part_1(input: String) -> String {

    let mut hands: Vec<(Vec<u32>,u32)> = Vec::new();

    for line in input.lines(){
        let mut parts_of_line = line.split_ascii_whitespace();
        let hand_raw = parts_of_line.next().unwrap();
        let bid = parts_of_line.next().unwrap().parse::<u32>().unwrap();

        let hand = convert_hand(&hand_raw);

        hands.push((hand, bid));
    }
    //println!("BEFORE: {hands:#?}");

    hands.sort_by(|a,b|{

        compare_hands(&a.0,&b.0)

    });

   // println!("AFTER: {hands:#?}");

    let mut result: u32 = 0;

    for (index, (_, value)) in hands.iter().enumerate(){
        result += ((index as u32) + 1) * value;
    }


    result.to_string()

}

fn compare_hands(a: &Vec<u32>, b: &Vec<u32>) -> Ordering {

    let a_type: u32 = type_of_and(a);
    let b_type: u32 = type_of_and(b);

    if a_type!=b_type {
        if a_type > b_type{
            return Ordering::Greater;
        }else{
            return Ordering::Less;
        }
    }
    else {
        return compare_hands_raw(a,b);
    }
}

fn compare_hands_raw(a: &[u32], b: &[u32]) -> Ordering {
    for i in 0..a.len(){
        if a[i]!=b[i]{
            if a[i] > b[i] {
                return Ordering::Greater;
            }
            else {
                return Ordering::Less;
            }
        }
    }
    panic!("ordering bad");
}

fn type_of_and(a: &[u32]) -> u32 {
    const FIVE_OF_A_KIND: u32 = 7;
    const FOUR_OF_A_KIND: u32 = 6;
    const FULL_HOUSE: u32 = 5;
    const THREE_OF_A_KIND: u32 = 4;
    const TWO_PAIR: u32 = 2;
    const ONE_PAIR: u32 = 1;
    const HIGH_CARD: u32 = 0;
    
    let mut cards: HashSet<&u32> = HashSet::new();
    for num in a.iter(){
        cards.insert(num);
    }

    let mut how_man_cards_for_type: Vec<u32> = Vec::new();
    for kind in cards.clone(){
        let how_many = a.iter()
            .filter(|n| n==&kind)
            .count();
        how_man_cards_for_type.push(how_many as u32);
    }


    if cards.len()==1{
        return FIVE_OF_A_KIND;
    }
    else if cards.len()==2{

        
        if (how_man_cards_for_type[0] == 1 && how_man_cards_for_type[1] == 4) ||
        (how_man_cards_for_type[0] == 4 && how_man_cards_for_type[1] == 1){
            return FOUR_OF_A_KIND;
        }else{
            return FULL_HOUSE;
        }
    }
    else if cards.len()==3{

        if(how_man_cards_for_type[0] == 3 && how_man_cards_for_type[1] == 1 && how_man_cards_for_type[2]==1) ||
        (how_man_cards_for_type[0] == 1 && how_man_cards_for_type[1] == 3 && how_man_cards_for_type[2]==1) ||
        (how_man_cards_for_type[0] == 1 && how_man_cards_for_type[1] == 1 && how_man_cards_for_type[2]==3) {

            return THREE_OF_A_KIND;
        } else{
            return TWO_PAIR;
        }


    }
    else if cards.len()==4{
        return ONE_PAIR;
    }
    else if cards.len()==5{
        return HIGH_CARD;
    } else {
        panic!("type of card bad");
    }


}

pub(crate) fn part_2(input:String) -> String {
    "".to_string()
}

fn convert_hand(hand: &str) -> Vec<u32> {

    let hand_in_numbers = hand.trim()
        .replace("2", " 2 ")
        .replace("3", " 3 ")
        .replace("4", " 4 ")
        .replace("5", " 5 ")
        .replace("6", " 6 ")
        .replace("7", " 7 ")
        .replace("8", " 8 ")
        .replace("9", " 9 ")
        .replace("T", " 10 ")
        .replace("J", " 11 ")
        .replace("Q", " 12 ")
        .replace("K", " 13 ")
        .replace("A", " 14 ");

    let hand_in_vec: Vec<u32> = hand_in_numbers.split_ascii_whitespace()
        .map(|n| n.parse().unwrap())
        .collect();

    hand_in_vec
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = 
"32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483"
    ; 

    #[test]
    fn test_part_1() {
        assert_eq!("6440", part_1(TEST_INPUT.to_string()));
    }

    #[test]
    fn test_part_2() {
        assert_eq!("todo", part_2(TEST_INPUT.to_string()));
    }
}

