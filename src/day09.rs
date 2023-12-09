use std::fs;

pub(crate) fn day_09() {
    let input: String = fs::read_to_string("./inputs/9.txt").unwrap();
    println!("day 09:");
    println!("  part 1: {}", part_1(input.clone()));
    println!("  part 2: {}", part_2(input.clone()));
}

pub fn part_1(input: String) -> String {

    let result: isize = input.lines()
        .map(|line| parse_line(line))
        .map(|sequence| extrapolate_new_number_right(sequence))
        .sum();

    result.to_string()
}

fn parse_line(line: &str) -> Vec<isize> {
    let result: Vec<isize> = line.split_ascii_whitespace()
        .map(|n| n.parse().unwrap())
        .collect();
    result
}

pub(crate) fn part_2(input:String) -> String {

    let result: isize = input.lines()
        .map(|line| parse_line(line))
        .map(|sequence| extrapolate_new_number_left(sequence))
        .sum();

    result.to_string()
}

fn extrapolate_new_number_left(initial_sequence: Vec<isize>) -> isize {

    let all_sequences = find_all_sequences(initial_sequence);
    
    let mut new_number: isize = 0;

    for seq in all_sequences.iter().rev(){
        let last_number: &isize = seq.first().unwrap();
        new_number = *last_number - new_number;
    }
    new_number
}

fn extrapolate_new_number_right(initial_sequence: Vec<isize>) -> isize {

    let all_sequences = find_all_sequences(initial_sequence);
    
    let mut new_number: isize = 0;

    for seq in all_sequences.iter().rev(){
        let last_number: &isize = seq.last().unwrap();
        new_number = new_number + *last_number;
    }
    new_number
}

fn find_all_sequences(initial_sequence: Vec<isize>) -> Vec<Vec<isize>> {
    let mut all_sequences: Vec<Vec<isize>> = Vec::new();
    all_sequences.push(initial_sequence);
    
    while !is_all_zero( all_sequences.last().unwrap() ) {
        let mut new_sequence: Vec<isize> = Vec::new();
        let last_sequence = all_sequences.last().unwrap();
        for i in 0..(last_sequence.len()-1) {
            new_sequence.insert(i, 
                last_sequence.get(i+1).unwrap()-last_sequence.get(i).unwrap()
            )
        }
        all_sequences.push(new_sequence);
    }
    all_sequences.pop();
    //remove the only 0 sequence
    all_sequences
}

fn is_all_zero(vector: &Vec<isize>) -> bool {
    for value in vector {
        if *value!=0{
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = 
"0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45"
    ; 

    #[test]
    fn test_part_1() {
        assert_eq!("114", part_1(TEST_INPUT.to_string()));
    }

    #[test]
    fn test_part_2() {
        assert_eq!("2", part_2(TEST_INPUT.to_string()));
    }

    #[test]
    fn test_extrapolate_number() {
        assert_eq!(18, extrapolate_new_number_right(vec![0,3,6,9,12,15]));
        assert_eq!(28, extrapolate_new_number_right(vec![1,3,6,10,15,21]));
        assert_eq!(68, extrapolate_new_number_right(vec![10,13,16,21,30,45]));
    }
}

