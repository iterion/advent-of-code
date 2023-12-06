use rayon::prelude::*;
pub(crate) fn run() -> (usize, usize) {
    let input_string = get_input_string();
    (answer_part_1(input_string), answer_part_2(input_string))
}

fn answer_part_1(lines: &str) -> usize {
    FullMap::parse(lines).min_location_for_seeds(true)
}

fn answer_part_2(lines: &str) -> usize {
    FullMap::parse(lines).min_location_for_seeds(false)
}

fn get_input_string() -> &'static str {
    include_str!("../inputs/day05.txt")
}

#[derive(Debug, PartialEq)]
struct FullMap {
    seeds: Vec<usize>,
    seed_to_soil: Vec<PlacementOffset>,
    soil_to_fertilizer: Vec<PlacementOffset>,
    fertilizer_to_water: Vec<PlacementOffset>,
    water_to_light: Vec<PlacementOffset>,
    light_to_temperature: Vec<PlacementOffset>,
    temperature_to_humidity: Vec<PlacementOffset>,
    humidity_to_location: Vec<PlacementOffset>,
}

impl FullMap {
    fn parse(input: &str) -> Self {
        let mut lines = input.lines();
        let seeds = lines
            .next()
            .expect("first line should be seeds")
            .split_once(':')
            .unwrap()
            .1
            .split_whitespace()
            .map(|s| s.parse::<usize>().unwrap())
            .collect();

        // moving on
        lines.next();

        // should be seed-to-soil, skip
        lines.next();

        let mut seed_to_soil = vec![];
        for line in lines.by_ref() {
            // move on to next section
            if line.is_empty() {
                break;
            }

            seed_to_soil.push(PlacementOffset::parse(line));
        }

        // should be soil-to-fertilizer, skip
        lines.next();

        let mut soil_to_fertilizer = vec![];
        for line in lines.by_ref() {
            // move on to next section
            if line.is_empty() {
                break;
            }

            soil_to_fertilizer.push(PlacementOffset::parse(line));
        }

        // should be soil-to-fertilizer, skip
        lines.next();

        let mut fertilizer_to_water = vec![];
        for line in lines.by_ref() {
            // move on to next section
            if line.is_empty() {
                break;
            }

            fertilizer_to_water.push(PlacementOffset::parse(line));
        }

        // should be soil-to-fertilizer, skip
        lines.next();

        let mut water_to_light = vec![];
        for line in lines.by_ref() {
            // move on to next section
            if line.is_empty() {
                break;
            }

            water_to_light.push(PlacementOffset::parse(line));
        }

        // should be soil-to-fertilizer, skip
        lines.next();

        let mut light_to_temperature = vec![];
        for line in lines.by_ref() {
            // move on to next section
            if line.is_empty() {
                break;
            }

            light_to_temperature.push(PlacementOffset::parse(line));
        }

        // should be soil-to-fertilizer, skip
        lines.next();

        let mut temperature_to_humidity = vec![];
        for line in lines.by_ref() {
            // move on to next section
            if line.is_empty() {
                break;
            }

            temperature_to_humidity.push(PlacementOffset::parse(line));
        }

        // should be soil-to-fertilizer, skip
        lines.next();

        let mut humidity_to_location = vec![];
        for line in lines.by_ref() {
            // move on to next section
            if line.is_empty() {
                break;
            }

            humidity_to_location.push(PlacementOffset::parse(line));
        }

        FullMap {
            seeds,
            seed_to_soil,
            soil_to_fertilizer,
            fertilizer_to_water,
            water_to_light,
            light_to_temperature,
            temperature_to_humidity,
            humidity_to_location,
        }
    }

    fn expanded_seed_list(&self) -> Vec<usize> {
        let mut seed_iter = self.seeds.iter();
        let mut new_seeds = vec![];

        while let Some(seed) = seed_iter.next() {
            let count = seed_iter.next().unwrap();
            let mut all_seeds: Vec<usize> = (*seed..(*seed + count)).collect();
            new_seeds.append(&mut all_seeds);
        }
        new_seeds
    }

    fn min_location_for_seeds(&self, v1: bool) -> usize {
        let mut location = usize::MAX;
        let seeds = if v1 {
            self.seeds.clone()
        } else {
            self.expanded_seed_list()
        };
        let total = seeds.len();

        let mut num_processed = 0;
        let locations: Vec<usize> = seeds
            .par_iter()
            .map(|seed| self.location_from_seed(*seed))
            .collect();
        for new_local in &locations {
            if new_local < &location {
                location = *new_local;
            }

            num_processed += 1;
            if num_processed % 10000 == 0 {
                println!(
                    "{}/{} ({}) seeds processed",
                    num_processed,
                    total,
                    num_processed as f64 / total as f64
                );
            }
        }
        location
    }

    fn location_from_seed(&self, seed: usize) -> usize {
        let soil = self
            .seed_to_soil
            .iter()
            .find(|offset| offset.contains_item(seed))
            .map_or(seed, |offset| offset.item_location(seed));

        let fertilizer = self
            .soil_to_fertilizer
            .iter()
            .find(|offset| offset.contains_item(soil))
            .map_or(soil, |offset| offset.item_location(soil));

        let water = self
            .fertilizer_to_water
            .iter()
            .find(|offset| offset.contains_item(fertilizer))
            .map_or(fertilizer, |offset| offset.item_location(fertilizer));

        let light = self
            .water_to_light
            .iter()
            .find(|offset| offset.contains_item(water))
            .map_or(water, |offset| offset.item_location(water));

        let temperature = self
            .light_to_temperature
            .iter()
            .find(|offset| offset.contains_item(light))
            .map_or(light, |offset| offset.item_location(light));

        let humidity = self
            .temperature_to_humidity
            .iter()
            .find(|offset| offset.contains_item(temperature))
            .map_or(temperature, |offset| offset.item_location(temperature));

        self.humidity_to_location
            .iter()
            .find(|offset| offset.contains_item(humidity))
            .map_or(humidity, |offset| offset.item_location(humidity))
    }
}

#[derive(Debug, PartialEq)]
struct PlacementOffset {
    source: usize,
    destination: usize,
    count: usize,
}

impl PlacementOffset {
    fn parse(line: &str) -> Self {
        let mut parts = line.split_whitespace();
        let destination = parts
            .next()
            .expect("should always have destination")
            .parse()
            .expect("destination should be number");
        let source = parts
            .next()
            .expect("should always have source")
            .parse()
            .expect("source should be number");
        let count = parts
            .next()
            .expect("should always have count")
            .parse()
            .expect("count should be number");
        PlacementOffset {
            source,
            destination,
            count,
        }
    }

    fn contains_item(&self, item: usize) -> bool {
        (self.source..(self.source + self.count)).contains(&item)
    }

    fn item_location(&self, item: usize) -> usize {
        // don't want to deal with casting to isize
        if self.destination > self.source {
            self.destination
                .saturating_sub(self.source)
                .saturating_add(item)
        } else {
            item.saturating_sub(self.source.saturating_sub(self.destination))
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::day05::{answer_part_1, answer_part_2, get_input_string, FullMap, PlacementOffset};
    #[test]
    fn test_all_lines() {
        let lines = get_input_string();

        assert_eq!(answer_part_1(lines), 26273516);
        // run this if you dare, it uses all cores and still takes a while
        //assert_eq!(answer_part_2(lines), 34039469);
    }
    const EXAMPLE_LINES: &str = r#"seeds: 79 14 55 13

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

"#;

    #[test]
    fn test_short_example() {
        assert_eq!(answer_part_1(EXAMPLE_LINES), 35);
        assert_eq!(answer_part_2(EXAMPLE_LINES), 46);
    }

    #[test]
    fn test_parsing() {
        let offset_str = "50 98 2";
        assert_eq!(
            PlacementOffset::parse(offset_str),
            PlacementOffset {
                destination: 50,
                source: 98,
                count: 2
            }
        );

        assert_eq!(
            FullMap::parse(EXAMPLE_LINES),
            FullMap {
                seeds: vec![79, 14, 55, 13],
                seed_to_soil: vec![
                    PlacementOffset {
                        destination: 50,
                        source: 98,
                        count: 2
                    },
                    PlacementOffset {
                        destination: 52,
                        source: 50,
                        count: 48
                    },
                ],
                soil_to_fertilizer: vec![
                    PlacementOffset {
                        destination: 0,
                        source: 15,
                        count: 37
                    },
                    PlacementOffset {
                        destination: 37,
                        source: 52,
                        count: 2
                    },
                    PlacementOffset {
                        destination: 39,
                        source: 0,
                        count: 15
                    },
                ],
                fertilizer_to_water: vec![
                    PlacementOffset {
                        destination: 49,
                        source: 53,
                        count: 8
                    },
                    PlacementOffset {
                        destination: 0,
                        source: 11,
                        count: 42
                    },
                    PlacementOffset {
                        destination: 42,
                        source: 0,
                        count: 7
                    },
                    PlacementOffset {
                        destination: 57,
                        source: 7,
                        count: 4
                    },
                ],
                water_to_light: vec![
                    PlacementOffset {
                        destination: 88,
                        source: 18,
                        count: 7,
                    },
                    PlacementOffset {
                        destination: 18,
                        source: 25,
                        count: 70,
                    },
                ],
                light_to_temperature: vec![
                    PlacementOffset {
                        destination: 45,
                        source: 77,
                        count: 23,
                    },
                    PlacementOffset {
                        destination: 81,
                        source: 45,
                        count: 19,
                    },
                    PlacementOffset {
                        destination: 68,
                        source: 64,
                        count: 13,
                    },
                ],
                temperature_to_humidity: vec![
                    PlacementOffset {
                        destination: 0,
                        source: 69,
                        count: 1,
                    },
                    PlacementOffset {
                        destination: 1,
                        source: 0,
                        count: 69
                    },
                ],
                humidity_to_location: vec![
                    PlacementOffset {
                        destination: 60,
                        source: 56,
                        count: 37
                    },
                    PlacementOffset {
                        destination: 56,
                        source: 93,
                        count: 4,
                    },
                ],
            }
        );
    }

    #[test]
    fn test_offset_calcs() {
        let map = FullMap::parse(EXAMPLE_LINES);

        assert!(map.seed_to_soil[1].contains_item(79));
        assert_eq!(map.seed_to_soil[1].item_location(79), 81);
        assert_eq!(map.location_from_seed(79), 82);
    }

    #[test]
    fn test_expanded_seed_list() {
        let map = FullMap::parse(EXAMPLE_LINES);

        assert_eq!(
            map.expanded_seed_list(),
            vec![
                79, 80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90, 91, 92, 55, 56, 57, 58, 59, 60, 61,
                62, 63, 64, 65, 66, 67,
            ]
        );
    }
}
