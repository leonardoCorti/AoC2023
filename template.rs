use std::fs;

pub(crate) fn day_0X() {
    let input: String = fs::read_to_string("./inputs/X.txt").unwrap();
    println!("day 0X:");
    println!("  part 1: {}", part_1(input.clone()));
    println!("  part 2: {}", part_2(input.clone()));
}

pub fn part_1(input: String) -> String {

    "".to_string()

}

pub(crate) fn part_2(input:String) -> String {
    "".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = 
""
    ; 

    #[test]
    fn test_part_1() {
        assert_eq!("288", part_1(TEST_INPUT.to_string()));
    }

    #[test]
    fn test_part_2() {
        assert_eq!("todo", part_2(TEST_INPUT.to_string()));
    }
}

