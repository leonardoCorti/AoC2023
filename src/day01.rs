use std::collections::HashMap;

use std::fs;

pub(crate) fn day_01() {
    let input: String = fs::read_to_string("./inputs/1.txt").unwrap();
    println!("day 01:");
    println!("  part 1: {}", part_1(input.clone()));
    println!("  part 2: {}", part_2(input.clone()));
}

pub fn part_1(input: String) -> String {
    let result: i32 = input.lines()
        .map( |line| {
            let digits : Vec<i32> = line.chars().filter(|character|
                character.is_numeric()
            ).map(|digit| digit.to_string().parse::<i32>().unwrap()).collect();
            let first = digits.iter().next().clone().unwrap();
            let last = digits.iter().last().clone().unwrap();
            //println!("{}{}",first, last);
            return first*10 + last;
        }).sum();
    return result.to_string();

}

pub(crate) fn part_2(input:String) -> String {

    let mut string_to_number: HashMap<&str, &str> = HashMap::new();
    string_to_number.insert("one", "1");
    string_to_number.insert("two", "2");
    string_to_number.insert("three", "3");
    string_to_number.insert("four", "4");
    string_to_number.insert("five", "5");
    string_to_number.insert("six", "6");
    string_to_number.insert("seven", "7");
    string_to_number.insert("eight", "8");
    string_to_number.insert("nine", "9");
    string_to_number.insert("zero", "0");

    let mut new_input = input.clone();

    let mut specific_cases : HashMap<&str, &str> = HashMap::new();
    specific_cases.insert("eightwo", "82");
    specific_cases.insert("eighthree", "83");
    specific_cases.insert("twone", "21");
    specific_cases.insert("oneight", "18");
    specific_cases.insert("nineight", "98");

    for (string_form, number_form) in specific_cases {
        new_input = new_input.replace(string_form, number_form);        
    }

    for (string_form, number_form) in string_to_number {
        new_input = new_input.replace(string_form, number_form);        
    }
    return part_1(new_input)
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
