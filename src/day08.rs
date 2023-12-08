use std::{fs, collections::HashMap};

pub(crate) fn day_08() {
    let input: String = fs::read_to_string("./inputs/8.txt").unwrap();
    println!("day 08:");
    println!("  part 1: {}", part_1(input.clone()));
    println!("  part 2: {}", part_2(input.clone()));
}

pub fn part_1(input: String) -> String {
    let mut lines = input.lines();
    
    let mut map: HashMap<String, (String,String)> = HashMap::new();
    let instructions: Vec<char> = lines.next().unwrap().trim().chars().collect();

    lines.next(); //skips the empty line

    while let Some(line) = lines.next(){
        let mut parts = line.trim().split(" = ");
        let from = parts.next().unwrap();
        let replace = &parts.next().unwrap()
            .replace("(", "")
            .replace(")", "");
        let mut to = replace
            .split(", ");
        let left = to.next().unwrap();
        let right = to.next().unwrap();
        map.insert(from.to_owned(), (left.to_owned(),right.to_owned()));
    }

    let mut counter = 0;
    let mut current_node = "AAA";

    while !(current_node=="ZZZ") {
        let direction = instructions[counter%instructions.len()];        

        if direction=='L'{
            current_node = &map.get(current_node).unwrap().0;
        }
        else if direction=='R'{
            current_node = &map.get(current_node).unwrap().1;
        }
        else{
            panic!("instructions unclear");
        }
        counter += 1;
    }
    counter.to_string()
}

pub(crate) fn part_2(input:String) -> String {
    "".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = 
"RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)"
    ; 

    const TEST_INPUT_2: &str = 
"LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)"
    ; 
    #[test]
    fn test_part_1() {
        assert_eq!("2", part_1(TEST_INPUT.to_string()));
        assert_eq!("6", part_1(TEST_INPUT_2.to_string()));
    }

    #[test]
    fn test_part_2() {
        assert_eq!("todo", part_2(TEST_INPUT.to_string()));
    }
}

