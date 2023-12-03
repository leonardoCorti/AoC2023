use std::fs;

pub fn day_03() {
    let input: String = fs::read_to_string("./inputs/3.txt").unwrap();
    println!("day 03:");
    println!("  part 1: {}", part_1(input.clone()));
    println!("  part 2: {}", part_2(input.clone()));
}

#[derive(Debug, PartialEq)]
struct Numbers{
    value: usize,
    start_position: (usize,usize),
    end_position: (usize,usize),
}

impl Numbers{
    pub fn is_adjacent_to(&self, symbols: &Vec<(usize,usize)>) -> bool{
        let mut number_positions: Vec<(usize, usize)> = Vec::new();
        for i in self.start_position.0..=self.end_position.0 {
            number_positions.push((i,self.start_position.1));
        }
        for position in number_positions{
            if position.0==0 && position.1==0 {
                for i in position.0..=position.0+1{
                    for j in position.1..=position.1+1{
                        if symbols.contains(&(i,j)){
                            return true;
                        }
                    }
                }
            }
            else if position.0 == 0 {
                for i in position.0..=position.0+1{
                    for j in position.1-1..=position.1+1{
                        if symbols.contains(&(i,j)){
                            return true;
                        }
                    }
                }
            }
            else if position.1 == 0 {
                for i in position.0-1..=position.0+1{
                    for j in position.1..=position.1+1{
                        if symbols.contains(&(i,j)){
                            return true;
                        }
                    }
                }
            }
            else{
                for i in position.0-1..=position.0+1{
                    for j in position.1-1..=position.1+1{
                        if symbols.contains(&(i,j)){
                            return true;
                        }
                    }
                }
            }
        }
        return false;
    }
}

pub fn part_1(input: String) -> String {

    let mut symbols_positions: Vec<(usize,usize)> = Vec::new();

    //let mut part_numbers: HashSet<Numbers>= HashSet::new();
    let mut part_numbers: Vec<Numbers>= Vec::new();
    
    for (index, line) in input.lines().enumerate(){

        let mut in_number: bool ;
        let mut current_number: Vec<char> = Vec::new();

        for(index_of_line, character_of_line) in line.chars().enumerate(){
            if !character_of_line.is_ascii_digit() &&
            !(character_of_line=='.'){
                symbols_positions.push((index_of_line,index));
            }

            in_number= character_of_line.is_ascii_digit();
            
            if in_number{
                current_number.push(character_of_line);
            }

            if !in_number || index_of_line==(line.len()-1) {

                if !current_number.is_empty() {
                   let value: usize = current_number.iter()
                        .collect::<String>().parse().unwrap(); 
                    let start_position = index_of_line - current_number.len(); 
                    let end_position = index_of_line; 
                
                    let the_number = Numbers{ 
                        value,
                        start_position: (start_position, index),
                        end_position: (end_position-1, index)
                    };
                    if !part_numbers.contains(&the_number){
                        part_numbers.push(the_number);
                    }
                    current_number = Vec::new();
                }

            }

        }
    }
    //println!("{:#?}", part_numbers);
 
    let result: usize = part_numbers.iter()
        .filter(|pn| pn.is_adjacent_to(&symbols_positions))
        .map(|pn| pn.value)
        .sum();
    
    //"".to_string()
    result.to_string()
}

pub fn part_2(input: String) -> String {

    let mut gear_positions: Vec<(usize,usize)> = Vec::new();

    //let mut part_numbers: HashSet<Numbers>= HashSet::new();
    let mut part_numbers: Vec<Numbers>= Vec::new();
    
    for (index, line) in input.lines().enumerate(){

        let mut in_number: bool ;
        let mut current_number: Vec<char> = Vec::new();

        for(index_of_line, character_of_line) in line.chars().enumerate(){
            if !character_of_line.is_ascii_digit() &&
            !(character_of_line=='.')&&
            character_of_line=='*'{
                gear_positions.push((index_of_line,index));
            }

            in_number= character_of_line.is_ascii_digit();
            
            if in_number{
                current_number.push(character_of_line);
            }

            if !in_number || index_of_line==(line.len()-1) {

                if !current_number.is_empty() {
                   let value: usize = current_number.iter()
                        .collect::<String>().parse().unwrap(); 
                    let start_position = index_of_line - current_number.len(); 
                    let end_position = index_of_line; 
                
                    let the_number = Numbers{ 
                        value,
                        start_position: (start_position, index),
                        end_position: (end_position-1, index)
                    };
                    if !part_numbers.contains(&the_number){
                        part_numbers.push(the_number);
                    }
                    current_number = Vec::new();
                }

            }

        }
    }

    //let result: Vec<&(usize,usize)> = gear_positions.iter()
    //    .filter(|gp| is_adjacent_to_2_of(gp,&part_numbers)).collect();



    let result: usize = gear_positions.iter()
        .filter(|gp| is_adjacent_to_2_of(gp,&part_numbers))
        .map(|gp| gear_ratio(gp, &part_numbers))
        .sum();


    result.to_string()
}

fn is_adjacent_to_2_of( coord: &(usize,usize), part_numbers: &Vec<Numbers>) -> bool {

    let mut coord_vec: Vec<(usize,usize)> = Vec::new();
    coord_vec.push(coord.clone());
    let mut counter = 0;

    for part in part_numbers{
        if part.is_adjacent_to(&coord_vec){
            counter += 1;
        }
    }

    if counter == 2 {
        return true;
    }
    else{
        false
    }
}

fn gear_ratio( coord: &(usize,usize), part_numbers: &Vec<Numbers>) -> usize {

    let mut coord_vec: Vec<(usize,usize)> = Vec::new();
    coord_vec.push(coord.clone());
    let mult: usize;

    mult = part_numbers.iter()
        .filter(|pn| pn.is_adjacent_to(&coord_vec))
        .map(|pn| pn.value)
        .reduce(|a,b| a*b).unwrap();


   mult 
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_1() {
        let input = 
"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
".to_string();
        assert_eq!("4361", part_1(input));
    }

    #[test]
    fn test_part_2() {
        let input = 
"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
".to_string();
        assert_eq!("467835", part_2(input));
    }
}
