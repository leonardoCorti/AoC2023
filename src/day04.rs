use std::fs;

pub(crate) fn day_04() {
    let input: String = fs::read_to_string("./inputs/4.txt").unwrap();
    println!("day 04:");
    println!("  part 1: {}", part_1(input.clone()));
    println!("  part 2: {}", part_2(input.clone()));
}

pub fn part_1(input: String) -> String {

    let mut sum: i32 = 0;

    for line in input.lines(){
        let line_without_card: &str= line.split(":").last().unwrap(); 
        let mut parts_of_card = line_without_card.split("|");

        let first_part_of_card: String = parts_of_card.next().unwrap().trim().replace("  ", " ").to_string();
        let second_part_of_card: String = parts_of_card.next().unwrap().trim().replace("  ", " ").to_string();

        let winning_numbers: Vec<i32> = first_part_of_card.split(" ")
            .map(|n| n.trim().parse::<i32>().unwrap())
            .collect();
        //println!("{:?}", winning_numbers);
        let my_numbers: Vec<i32> = second_part_of_card.split(" ")
            .map(|n| n.trim().parse::<i32>().unwrap())
            .collect();
        //println!("{:?}", my_numbers);
        
        let my_winning_number: i32 = winning_numbers.iter()
            .map(|n| {
                if my_numbers.contains(n){
                    1
                }
                else {
                    0
                }
            }).sum();  
        
        if my_winning_number > 0 {
            let base: i32 = 2;
            let points = base.pow((my_winning_number-1).try_into().unwrap());
            sum += points;
        }
    }

    sum.to_string()
}

pub(crate) fn part_2(input:String) -> String {

    let mut list_of_cards: Vec<(i32,i32)> = Vec::new();
    let mut copies_of_cards: Vec<(i32,i32)> = Vec::new();

    for (id, line) in input.lines().enumerate(){
        let line_without_card: &str= line.split(":").last().unwrap(); 
        let mut parts_of_card = line_without_card.split("|");

        let first_part_of_card: String = parts_of_card.next().unwrap().trim().replace("  ", " ").to_string();
        let second_part_of_card: String = parts_of_card.next().unwrap().trim().replace("  ", " ").to_string();

        let winning_numbers: Vec<i32> = first_part_of_card.split(" ")
            .map(|n| n.trim().parse::<i32>().unwrap())
            .collect();
        //println!("{:?}", winning_numbers);
        let my_numbers: Vec<i32> = second_part_of_card.split(" ")
            .map(|n| n.trim().parse::<i32>().unwrap())
            .collect();
        //println!("{:?}", my_numbers);

        let points: i32 = winning_numbers.iter()
            .map(|n| {
                if my_numbers.contains(n){
                    1
                }
                else {
                    0
                }
            }).sum();
        let new_id = (id as i32)+1;
        list_of_cards.push((new_id, points));

        copies_of_cards.push((new_id, 1));
    }
    //println!("{:?}", list_of_cards);
    //println!("{:?}", copies_of_cards);

    let length: usize = list_of_cards.len();

    for (id, points) in list_of_cards {
        if points != 0 {
            for i in 1..=points{
                let new_id = id + i;
                if (new_id as usize) <= length{
                    let new_count = copies_of_cards[(new_id as usize)-1].1 + copies_of_cards[(id as usize)-1].1;
                    copies_of_cards[new_id as usize-1] = (new_id,new_count);
                }
            }
        }

    }
    //println!("{:?}", copies_of_cards);
    let result: i32 = copies_of_cards.iter().map(|p| p.1).sum();
    

    result.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_1() {
        let input= 
"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
".to_string();
        assert_eq!("13", part_1(input));
    }

    #[test]
    fn test_part_2() {
        let input=
"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
".to_string();
        
        assert_eq!("30", part_2(input));
    }
}
