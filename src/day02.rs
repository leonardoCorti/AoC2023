use std::fs;

pub fn day_02() {
    let input: String = fs::read_to_string("./inputs/2.txt").unwrap();
    println!("day 02:");
    println!("  part 1: {}", part_1(input.clone()));
    println!("  part 2: {}", part_2(input.clone()));
}


pub fn part_1(input: String) -> String {
    let red_max = 12;
    let green_max = 13;
    let blue_max = 14;

    let games: Vec<(i32, String)> = input.lines().map(|line|{
        let parts: Vec<&str> = line.split(":").collect();
        let id: i32 = parts[0].trim().split_once(" ").unwrap().1.trim().parse().unwrap();
        (id, parts[1].to_string())
    }).collect();
    //println!("{:?}", games);

    let mut sum_of_ids: i32 = 0;

    for (id, game) in games{
        let matches: Vec<&str> = game.split(";").collect();
        let mut too_much = false;

        for single_match in matches{
            let cubes: Vec<&str> = single_match.split(",").collect();            
            let (mut red, mut green, mut blue) = (0, 0 ,0);

            for c in cubes {
                if c.contains("blue"){
                    blue = c.replace("blue", "").trim().parse().unwrap();
                }
                else if c.contains("red"){
                    red = c.replace("red", "").trim().parse().unwrap();
                }
                else if c.contains("green"){
                    green = c.replace("green", "").trim().parse().unwrap();
                }
                else {
                    panic!("no blue red or green");
                }
            }
            
            if red > red_max || blue > blue_max || green > green_max{
                too_much = true;
            }
            
        }
        if !too_much {
            sum_of_ids += id;
        }

    }

    return sum_of_ids.to_string();
}

pub fn part_2(input: String) -> String {

    let games: Vec<(i32, String)> = input.lines().map(|line|{
        let parts: Vec<&str> = line.split(":").collect();
        let id: i32 = parts[0].trim().split_once(" ").unwrap().1.trim().parse().unwrap();
        (id, parts[1].to_string())
    }).collect();

    let result: i32 = games.iter().map(|(_, game)|{
        
        let matches: Vec<&str> = game.split(";").collect();
        let (mut red_min, mut blue_min, mut green_min) = (-1, -1, -1);

        for single_match in matches{
            let cubes: Vec<&str> = single_match.split(",").collect();            
            let (mut red, mut green, mut blue) = (0, 0 ,0);

            for c in cubes {
                if c.contains("blue"){
                    blue = c.replace("blue", "").trim().parse().unwrap();
                }
                else if c.contains("red"){
                    red = c.replace("red", "").trim().parse().unwrap();
                }
                else if c.contains("green"){
                    green = c.replace("green", "").trim().parse().unwrap();
                }
                else {
                    panic!("no blue red or green");
                }
            }

            if red_min == -1 || red_min < red {
                red_min = red;
            }
            if blue_min == -1 || blue_min < blue {
                blue_min = blue;
            }
            if green_min == -1 || green_min < green {
                green_min = green;
            }
            
        }

        return green_min*blue_min*red_min;

    }).sum();

    return result.to_string();
}



#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_1() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green".to_string();
        assert_eq!("8", part_1(input));
    }

    #[test]
    fn test_part_2() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
            Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
            Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
            Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
            Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green".to_string();
        assert_eq!("2286", part_2(input));
    }
}
