use std::{fs, collections::HashMap};

pub(crate) fn day_13() {
    let input: String = fs::read_to_string("./inputs/13.txt").unwrap();
    println!("day 13:");
    println!("  part 1: {}", part_1(input.clone()));
    println!("  part 2: {}", part_2(input.clone()));
}

#[derive(Debug)]
struct Mirror {
    map: Vec<Vec<char>>,
}

impl Mirror {
    fn from_str(input: &str) -> Mirror{
        let mut map: Vec<Vec<char>> = Vec::new();

        for (_i, line) in input.lines().enumerate(){
            let mut line_new: Vec<char> = Vec::new();
            for (_j,character) in line.chars().enumerate(){
                line_new.push(character);
            }
            map.push(line_new);
        }
        Mirror { map }
    }

    fn horizontal(&self) -> Option<usize> {

        let mut Table: HashMap<Vec<char>, Vec<usize>> = HashMap::new();
        for (i,line) in self.map.iter().enumerate() {
            if let Some(list) = Table.get_mut(line){
                list.push(i);
            } else {
                Table.insert(line.to_vec(), vec![i]);
            }
        }

        todo!()
    }

    fn vertical(&self) -> Option<usize> {
        todo!()
    }

    fn find_value(&self) -> usize {

        if let Some(value) = self.horizontal(){
            return value;
        }
        if let Some(value) = self.vertical(){
            return value;
        }
        panic!("didn't find simmetry");
    }

    fn horizontal_no_hash(&self) -> Option<usize> {
        for i in 0..self.map.len()-1{
            if is_symmetric(&self.map, i){
                return Some(i);
            }
        }        
        None
    }

    fn vertical_no_hash(&self) -> Option<usize> {
        for column_index in 0..self.map[0].len(){
            if is_symmetric_vertical(&self.map, column_index){
                return Some(column_index);
            }
        }
        None
    }

    fn find_value_no_hash(&self) -> usize {

        if let Some(value) = self.horizontal_no_hash(){
            //println!("found horizontal at {value}");
            return (value+1)*100;
        }
        if let Some(value) = self.vertical_no_hash(){
            //println!("found vertical at {value}");
            return value+1;
        }
        panic!("didn't find simmetry");
    }

}

fn is_symmetric_vertical(map: &Vec<Vec<char>>, column_index: usize) -> bool {


    let y: isize = column_index as isize;
    let x_length: isize = map.len() as isize;
    let y_length: isize = map[0].len() as isize;
    let mut count = 0;

    while 0<=(y-count) && (y+count)<=y_length-2 {
        for i in 0..x_length {
            if map[i as usize][(y-count) as usize] != map[i as usize][(y+count+1) as usize] {
                return false;
            }
        }
        count += 1;
    }

    return true;
}

fn print_map(map: &Vec<Vec<char>>) {
    for line in map{
        for character in line{
            print!("{character}");
        }
        println!("");
    }
}

fn is_symmetric(map: &Vec<Vec<char>>, i: usize) -> bool {

    let x: isize = i as isize;
    let length: isize = map.len() as isize;
    let mut count = 0;

    while 0<=(x-count) && (x+count)<=length-2 {
        if map[(x-count) as usize]!=map[(x+count+1) as usize] {
            return false;
        }
        count += 1;
    }
    true
}

pub fn part_1(input: String) -> String {

    let maps:Vec<Mirror> = input.split("\n\n")
        .map(|map| Mirror::from_str(map))
        .collect();

    let result: usize = maps.iter().map(|mirror| mirror.find_value_no_hash()).sum();

    result.to_string()

}

pub(crate) fn part_2(input:String) -> String {
    "".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = 
"#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#"
    ; 

    #[test]
    fn test_part_1() {
        assert_eq!("405", part_1(TEST_INPUT.to_string()));
    }

    #[test]
    fn test_part_2() {
        assert_eq!("todo", part_2(TEST_INPUT.to_string()));
    }
}

