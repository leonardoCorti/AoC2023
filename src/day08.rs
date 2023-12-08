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

    parse_map(lines, &mut map);

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

fn parse_map(mut lines: std::str::Lines<'_>, map: &mut HashMap<String, (String, String)>) {
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
}

pub(crate) fn part_2(input:String) -> String {

    let mut lines = input.lines();
    
    let mut map: HashMap<String, (String,String)> = HashMap::new();
    let instructions: Vec<char> = lines.next().unwrap().trim().chars().collect();

    lines.next(); //skips the empty line

    parse_map(lines, &mut map);

    let mut current_nodes: Vec<&str> = Vec::new();

    for node in map.keys() {
            if is_start_node(node) {
            current_nodes.push(node);
        }
    }

    //println!("starting from: {current_nodes:#?}");

    let result = current_nodes.iter()
        .map(|n| part2_z(&map, n.to_string(), &instructions))
        .reduce(|a,b| lcm(a,b))
        .unwrap();

    result.to_string()
}

fn lcm(a: usize, b: usize) -> usize {
    if a == 0 || b == 0{
        0
    } else {
        (a * b) / gcd(a,b)
    }
}

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd(b, a%b)
    }
}

fn part2_z(map: &HashMap<String, (String, String)>, starting_node: String, instructions: &Vec<char> ) -> usize {

    let mut current_node: &str = &starting_node;
    let mut counter = 0;

    //println!("starting for {current_node}");
    while !(is_end_node(&current_node)){
        let direction = instructions[counter%instructions.len()];        
        //println!("{current_node}");

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
    //println!("result is {counter}");
    counter
}

fn is_start_node(node: &str) -> bool {
    let node2: String = node.to_string();
    is_last_character(&node2, 'A')
}

fn is_end_node(node: &str) -> bool {
    let node2: String = node.to_string();
    is_last_character(&node2, 'Z')
}

fn is_last_character(node: &String, character: char) -> bool{
    node.chars().last().unwrap()==character
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

    const TEST_INPUT_PART_2: &str =
"LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)"
    ;

    #[test]
    fn test_part_1() {
        assert_eq!("2", part_1(TEST_INPUT.to_string()));
        assert_eq!("6", part_1(TEST_INPUT_2.to_string()));
    }

    #[test]
    fn test_part_2() {
        assert_eq!("6", part_2(TEST_INPUT_PART_2.to_string()));
    }
}

