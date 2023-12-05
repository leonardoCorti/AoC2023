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

    let mut input_parts = input.split("\r\n\r\n");
    //let mut input_parts = input.split("\n\n");
    //println!("{input_parts:#?}");
    let mut maps: Vec<Map> = Vec::new();

    let seeds: Vec<usize> = input_parts.next().unwrap()
        .replace("seeds: ", "")
        .split(" ")
        .map(|n| n.trim().parse().unwrap())
        .collect();

    while let Some(map) = input_parts.next(){
        //println!("map is {map:?}");
        
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
    //println!("{maps:#?}");

    let result: usize = seeds.iter()
        .map(|seed| seed_to_location(seed, &maps))
        .min().unwrap();


    result.to_string()
}

fn seed_to_location(seed: &usize, maps: &Vec<Map>) -> usize {

    //println!("START SEED: {seed}");

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

    //println!("LOCATION SEED: {result}");
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

pub(crate) fn part_2_v1(input:String) -> String {

    //let mut input_parts = input.split("\r\n\r\n");
    let mut input_parts = input.split("\n\n");
    //println!("{input_parts:#?}");
    let mut maps: Vec<Map> = Vec::new();

    let seeds_raw: Vec<usize> = input_parts.next().unwrap()
        .replace("seeds: ", "")
        .split(" ")
        .map(|n| n.trim().parse().unwrap())
        .collect();

    let mut seeds_intervals: Vec<(usize,usize)> = Vec::new();

    let mut seeds_raw_iter = seeds_raw.iter();

    while let (Some(start), Some(length)) = (seeds_raw_iter.next(), seeds_raw_iter.next()) {
        seeds_intervals.push((start.clone(),length.clone()));
    }

    while let Some(map) = input_parts.next(){
        //println!("map is {map:?}");
        
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
    //println!("{maps:#?}");

    //let result: usize = seeds.iter()
    //    .map(|seed| seed_to_location(seed, &maps))
    //    .min().unwrap();

    let mut result: usize = 0;
    let mut is_first_round = true;
    let mut sum = 0;
    for (_, l) in seeds_intervals.iter() {
        sum += l;
    }
    println!("DEVO FARNE {sum}");

    for (seed_range_start, seed_range_length) in seeds_intervals {
        println!("ho iniziato da {seed_range_start}");

        for seed in seed_range_start..=(seed_range_start + seed_range_length){
            let location_of_seed = seed_to_location(&seed, &maps);
            if location_of_seed < result || is_first_round{
                result = location_of_seed;
                is_first_round = false;
            }
        }
        
    }


    result.to_string()
}

pub(crate) fn part_2(input: String) -> String {
    let mut groups = input.split("\r\n\r\n");
    let mut nums = groups.next().unwrap()
        .split_ascii_whitespace().skip(1)
        .map(|num| num.parse::<u64>().unwrap());
    let mut items = Vec::new();
    while let Some(num) = nums.next() {
        items.push((num, num + nums.next().unwrap()))
    }

    let mut new_items = Vec::new();
    let mut unmapped_items = Vec::new();

    for group in groups {
        new_items.clear();

        for line in group.lines().skip(1) {
            unmapped_items.clear();

            let mut nums = line.split_ascii_whitespace()
                .map(|num| num.parse::<u64>().unwrap());
            let dst = nums.next().unwrap();
            let src = nums.next().unwrap();
            let len = nums.next().unwrap();
            let src_end = src + len;

            for &(item, item_end) in &items {
                unmapped_items.extend([
                    (item, std::cmp::min(src, item_end)),
                    (std::cmp::max(src_end, item), item_end),
                ].into_iter().filter(|&(start, end)| end > start));

                let start = dst + (std::cmp::min(std::cmp::max(src, item), src_end) - src);
                let end = dst + (std::cmp::max(std::cmp::min(src_end, item_end), src) - src);
                if end > start {
                    new_items.push((start, end));
                }
            }

            std::mem::swap(&mut items, &mut unmapped_items);
        }

        if items.len() > new_items.len() {
            items.extend(new_items.iter().copied());
        } else {
            new_items.extend(items.iter().copied());
            std::mem::swap(&mut items, &mut new_items);
        }

        items.sort();
        new_items.clear();
        for &i in &items {
            let (start, end) = i;
            let mut added = false;
            if let Some(last) = new_items.last_mut() {
                if last.1 == start {
                    last.1 = end;
                    added = true;
                }
            }
            if !added {
                new_items.push(i);
            }
        }
        std::mem::swap(&mut items, &mut new_items);
    }

    items.into_iter().min().unwrap().0.to_string()
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
        assert_eq!("46", part_2(input));
    }
}
