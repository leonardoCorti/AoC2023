use std::{fs::{self}};

fn main() {
    println!("Hello, world!");
    let input: String = fs::read_to_string("./inputs/1.txt").unwrap();
    println!("part 1: {}", part_1(input));
}

pub fn part_1(input: String) -> String {
    let result: i32 = input.lines()
        .map( |line| {
            let digits : Vec<i32> = line.chars().filter(|character|
                character.is_numeric()
            ).map(|digit| digit.to_string().parse::<i32>().unwrap()).collect();
            let first = digits.iter().next().clone().unwrap();
            let last = digits.iter().last().clone().unwrap();
            println!("{}{}",first, last);
            return first*10 + last;
        }).sum();
    return result.to_string();

}

fn part_2(input:String) -> String {
    let result: i32 = input.lines()
        .map( |line| {
            let digits : Vec<i32> = line.chars().filter(|character|
                character.is_numeric()
            ).map(|digit| digit.to_string().parse::<i32>().unwrap()).collect();
            let first = digits.iter().next().clone().unwrap();
            let last = digits.iter().last().clone().unwrap();
            println!("{}{}",first, last);
            return first*10 + last;
        }).sum();
    return result.to_string();

todo!()

}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_1() {
        let input = "1abc2
            pqr3stu8vwx
            a1b2c3d4e5f
            treb7uchet".to_string();
        assert_eq!("142", part_1(input));
    }

    #[test]
    fn test_part_2() {
        let input = "two1nine
            eightwothree
            abcone2threexyz
            xtwone3four
            4nineeightseven2
            zoneight234
            7pqrstsixteen".to_string();
        assert_eq!("281", part_2(input));
    }
}
