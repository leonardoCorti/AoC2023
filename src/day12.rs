use std::fs;

pub(crate) fn day_12() {
    let input: String = fs::read_to_string("./inputs/12.txt").unwrap();
    println!("day 12:");
    println!("  part 1: {}", part_1(input.clone()));
    println!("  part 2: {}", part_2(input.clone()));
}

pub fn part_1(input: String) -> String {

    let result: usize = input.lines()
        .map(|l| parse_line(l))
        .map(|list| find_number_of_combinations(list))
        .sum();

    result.to_string()
}

#[derive(Debug, Clone)]
struct LineOfInput{
    map: Vec<char>,
    combinations: Vec<usize>,
}

fn parse_line(line: &str) -> LineOfInput {

    let mut line_parts = line.split_ascii_whitespace();
    let map: Vec<char> = line_parts.next().unwrap()
        .chars()
        .collect();
    let combinations: Vec<usize> = line_parts.next().unwrap()
        .split(",")
        .map(|n| n.parse().unwrap())
        .collect();
    
    let result = LineOfInput{map, combinations};
    //println!("{result:#?}");
    result
}

fn find_number_of_combinations(list: LineOfInput) -> usize {
    
    if list.map.contains(&'?'){
        let index: usize = find_first_interrogative(&list.map);
        let mut variation_1 = list.clone();
        variation_1.map[index]='.';
        let mut variation_2 = list.clone();
        variation_2.map[index]='#';
        return find_number_of_combinations(variation_1) + find_number_of_combinations(variation_2);

    } else {
        if is_correct_combination(list){
            return 1;
        } else {
            return 0;
        }
    }

}

fn find_first_interrogative(map: &Vec<char>) -> usize {
    let mut iterator = map.iter().enumerate();
    let mut result: usize = 0;

    while let Some((index, character)) = iterator.next() {
        if character==&'?'{
            result = index;
            break;
        }
    } 
    result
}

fn is_correct_combination(list: LineOfInput) -> bool {
    //println!("comparing {:?} with {:?}", list.map, list.combinations);

    if list.map.contains(&'?') {
        panic!("is_correct_combination received not ready input");
    }
    
    let map = list.map;
    let mut combinations: Vec<usize>= list.combinations.clone();
    combinations.reverse();

    let mut length=0;

    for character in map.iter() {
        if character==&'#'{
            length += 1;
        } else { //there is a '.'

            if length!=0{
                if combinations.is_empty(){
                    return false;
                }else {
                    if &length == combinations.last().unwrap(){
                        length = 0;
                        combinations.pop();
                    }else {
                        return false;
                    }
                }
                
            }
 
        }
        
    }

    if length!=0{
        if combinations.is_empty(){
            return false;
        } else {
            if &length == combinations.last().unwrap(){
                combinations.pop();
            }
        }
    }
    //println!("result is {}\n\n", combinations.is_empty());

    return combinations.is_empty();
}


pub(crate) fn part_2(input:String) -> String {
    "".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = 
"???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1"
    ; 

    const TEST_INPUT_temp: &str = 
"?###???????? 3,2,1"
    ; 

    #[test]
    fn test_part_1() {
        assert_eq!("10", part_1(TEST_INPUT_temp.to_string()));
    }

    #[test]
    fn test_part_2() {
        assert_eq!("todo", part_2(TEST_INPUT.to_string()));
    }
}

