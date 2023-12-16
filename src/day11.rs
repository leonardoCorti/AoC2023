#![allow(unused_variables,unused_mut,unused_assignments)]
use std::fs;

pub(crate) fn day_11() {
    let input: String = fs::read_to_string("./inputs/11.txt").unwrap();
    println!("day 11:");
    println!("  part 1: {}", part_1(input.clone()));
    println!("  part 2: {}", part_2(input.clone()));
}

type SpaceMap = Vec<Vec<char>>;

fn print_space_map(map: &SpaceMap){
    for line in map{
        for character in line{
            print!("{character}");
        }
        println!("");
    }
}

fn expand_space_map(map: SpaceMap) -> SpaceMap {
    let mut new_map: SpaceMap = map.clone();
    let x_lenght = map.len();
    let y_lenght = map[0].len();

    //expand lines
    let mut count = 0;
    for (i,line) in map.iter().enumerate() {
        if !line.contains(&'#') {
            new_map.insert(i+count, line.clone());
            count +=1;
        }
    }

    //expand columns
    let new_x_length = new_map.len();

    let mut count = 0;

    for y in 0..y_lenght{
        let mut is_empty = true;
        for x in 0..x_lenght{
            if map[x][y]!='.'{
                is_empty = false;
            }            
        }
        if is_empty {
            for x in 0..new_x_length {
                new_map[x].insert(y+count, '.');
            }
            count += 1;
        }
    }

    new_map
}


pub fn part_1(input: String) -> String {

    let mut map: SpaceMap = Vec::new();

    for line in input.lines(){
        let mut line_vec: Vec<char> = Vec::new();
        for character in line.chars(){
            line_vec.push(character);                        
        }
        map.push(line_vec);
    }

    let expanded_map = expand_space_map(map);
   
    let mut galaxies: Vec<(usize,usize)> = Vec::new();

    for (x,line) in expanded_map.iter().enumerate(){
        for (y,character) in line.iter().enumerate(){
            if character==&'#' {
                galaxies.push((x,y));
            }
        }
    }

    let result: usize = distance_between_pairs(galaxies);

    result.to_string()
}

fn distance_between_pairs(galaxies: Vec<(usize, usize)>) -> usize {

    let mut result = 0;
    for (i, (x1,y1)) in galaxies.iter().enumerate(){
        for (x2,y2) in galaxies.iter().skip(i){
            let disatnce = x2.abs_diff(*x1) + y2.abs_diff(*y1);
            result += disatnce;
        }
    }

    result
}

pub(crate) fn part_2(input:String) -> String {
    "".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = 
"...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#....."
    ; 

    #[test]
    fn test_part_1() {
        assert_eq!("374", part_1(TEST_INPUT.to_string()));
    }

    #[test]
    fn test_part_2() {
        assert_eq!("todo", part_2(TEST_INPUT.to_string()));
    }
}

