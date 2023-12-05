use std::fs;

pub(crate) fn day_05() {
    let input: String = fs::read_to_string("./inputs/5.txt").unwrap();
    println!("day 05:");
    println!("  part 1: {}", part_1(input.clone()));
    println!("  part 2: {}", part_2(input.clone()));
}
#[derive(Debug)]
struct Map{
    src: String,
    dst: String,
    src_start: usize,
    src_end: usize,
    dst_start:usize,
    dst_end:usize,
}

pub fn part_1(input: String) -> String {

    let mut input_parts = input.split("\n\n");
    println!("{input_parts:#?}");
    let mut maps: Vec<Map> = Vec::new();

    let seeds: Vec<usize> = input_parts.next().unwrap()
        .replace("seeds: ", "")
        .split(" ")
        .map(|n| n.trim().parse().unwrap())
        .collect();

    while let Some(map) = input_parts.next(){
        println!("map is {map:?}");
        
        let mut lines = map.split("\n");

        let replace = &lines.next().unwrap()
            .trim()
            .replace(" map:", "")
            .clone();
        let src_and_dst: Vec<&str> = replace
            .split("-to-")
            .collect();
        let src: String = src_and_dst[0].to_string();
        let dst: String = src_and_dst[1].to_string();

        while let Some(ranges_string) = lines.next(){
            if ranges_string.is_empty() {
                break;
            }
            let numbers: Vec<usize> = ranges_string.trim()
                .split(" ")
                .map(|n| n.parse().unwrap())
                .collect();

            let src_start: usize = numbers[1];
            let dst_start: usize = numbers[0];
            let duration: usize = numbers[2];
            let src_end: usize = src_start + duration;
            let dst_end: usize = dst_start + duration;

            let my_map: Map = Map { 
                src: src.clone() ,
                dst: dst.clone() ,
                src_start ,
                src_end ,
                dst_start ,
                dst_end  
            };

            maps.push(my_map);
        }
    }
    println!("{maps:#?}");

    let result: usize = seeds.iter()
        .map(|seed| seed_to_location(seed, &maps))
        .min().unwrap();


    result.to_string()
}

fn seed_to_location(seed: &usize, maps: &Vec<Map>) -> usize {

    println!("START SEED: {seed}");

    let seed_to_soil: Vec<&Map> = maps.iter()
        .filter(|map| map.src.eq("seed") && map.dst.eq("soil")).collect();
    let soil_to_fertilizer: Vec<&Map> = maps.iter()
        .filter(|map| map.src.eq("soil") && map.dst.eq("fertilizer")).collect();
    let fertilizer_to_water: Vec<&Map> = maps.iter()
        .filter(|map| map.src.eq("fertilizer") && map.dst.eq("water")).collect();
    let water_to_light: Vec<&Map> = maps.iter()
        .filter(|map| map.src.eq("water") && map.dst.eq("light")).collect();
    let light_to_temperature: Vec<&Map> = maps.iter()
        .filter(|map| map.src.eq("light") && map.dst.eq("temperature")).collect();
    let temperature_to_humidity: Vec<&Map> = maps.iter()
        .filter(|map| map.src.eq("temperature") && map.dst.eq("humidity")).collect();
    let humidity_to_location: Vec<&Map> = maps.iter()
        .filter(|map| map.src.eq("humidity") && map.dst.eq("location")).collect();


    let result = map_translation(humidity_to_location, 
        &map_translation(temperature_to_humidity, 
            &map_translation(light_to_temperature, 
                &map_translation(water_to_light,
                    &map_translation(fertilizer_to_water, 
                        &map_translation(soil_to_fertilizer,
                            &map_translation(seed_to_soil, seed)
                        )
                    )
                )
            )
        )

    );

    println!("LOCATION SEED: {result}");
    result
}

fn map_translation(maps: Vec<&Map>, start: &usize ) -> usize {


    for map in maps {
        if start >= &map.src_start && start <= &map.src_end{

            let distance = start - map.src_start;
            return map.dst_start+distance;

        }
    }

    start.clone()
}

pub(crate) fn part_2(input:String) -> String {
    "".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_1() {
        let input = 
"seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
".to_string();
        assert_eq!("35", part_1(input));
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
