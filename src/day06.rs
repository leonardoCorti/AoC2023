use std::fs;

pub(crate) fn day_06() {
    let input: String = fs::read_to_string("./inputs/6.txt").unwrap();
    println!("day 06:");
    println!("  part 1: {}", part_1(input.clone()));
    println!("  part 2: {}", part_2(input.clone()));
}

pub fn part_1(input: String) -> String {

    let mut races: Vec<(u64, u64)> = Vec::new();


    let new_input = input
        .replace("Time: ", "")
        .replace("Distance: ", "");

    let mut input_raw = new_input.lines();

    let times = input_raw.next().unwrap();
    let records = input_raw.next().unwrap();

    let mut times_iterator = times.trim().split_ascii_whitespace(); 
    let mut records_iterator = records.trim().split_ascii_whitespace(); 

    while let (Some(time), Some(record)) = (times_iterator.next(), records_iterator.next()){
        races.push((time.parse().unwrap(), record.parse().unwrap()));
    }

    let result = races.iter()
        .map(|r| elaboration_race(r))
        .reduce(|a,b| a*b).unwrap();
    

    result.to_string()

}

fn elaboration_race(race_input: &(u64, u64)) -> u64 {

    let duration = race_input.0.clone();
    let record = race_input.1.clone();

    let mut sum = 0;

    for milliseconds in 1..(duration-1){
        let actual_movement_time = duration-milliseconds;
        let actual_movement = actual_movement_time*milliseconds;

        if actual_movement > record {
            sum += 1;
        }
    }
    sum

}

pub(crate) fn part_2(input:String) -> String {

    let mut races: Vec<(u64, u64)> = Vec::new();


    let new_input = input
        .replace("Time: ", "")
        .replace("Distance: ", "")
        .replace(" ", "");

    let mut input_raw = new_input.lines();

    let times = input_raw.next().unwrap();
    let records = input_raw.next().unwrap();

    let mut times_iterator = times.trim().split_ascii_whitespace(); 
    let mut records_iterator = records.trim().split_ascii_whitespace(); 

    while let (Some(time), Some(record)) = (times_iterator.next(), records_iterator.next()){
        races.push((time.parse().unwrap(), record.parse().unwrap()));
    }

    let result = races.iter()
        .map(|r| elaboration_race(r))
        .reduce(|a,b| a*b).unwrap();
    

    result.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = 
"Time:      7  15   30
Distance:  9  40  200"
    ; 

    #[test]
    fn test_part_1() {
        assert_eq!("288", part_1(TEST_INPUT.to_string()));
    }

    #[test]
    fn test_part_2() {
        assert_eq!("71503", part_2(TEST_INPUT.to_string()));
    }
}
