pub mod day5 {

    use std::time::Instant;
    fn num_for_next(seed: u64, step_line: Vec<u64>) -> (u64, bool) {
        let range_start = step_line[1];
        let range_end = range_start + step_line[2];

        if (range_start..range_end).contains(&seed) {
            let offset = seed - range_start;
            return (step_line[0] + offset, true);
        }

        (seed, false)
    }

    pub fn solution_1() {
        let start = Instant::now();
        static LINES: &str = include_str!("./inputs/day5/input.txt");
        let seeds: Vec<u64> = LINES
            .lines()
            .next()
            .unwrap()
            .split(": ")
            .last()
            .unwrap()
            .split_ascii_whitespace()
            .map(|s| s.parse::<u64>().unwrap())
            .collect();

        let mut maps: Vec<Vec<Vec<u64>>> = Vec::new();

        for line in LINES.split("\r\n\r\n").skip(1) {
            let testing: Vec<Vec<u64>> = line
                .split("\r\n")
                .filter(|f| !f.contains(&"map"))
                .map(|l| {
                    l.split_ascii_whitespace()
                        .map(|n| n.parse::<u64>().unwrap())
                        .collect()
                })
                .collect();
            maps.push(testing);
        }

        let mut res: Vec<u64> = Vec::new();

        for seed in &seeds {
            let mut silly = *seed;

            for (_, step) in maps.iter().enumerate() {
                for j in step {
                    let (n, in_range) = num_for_next(silly, j.clone());
                    silly = n;
                    if in_range {
                        break;
                    }
                }
            }

            res.push(silly);
        }

        // Print or use res as needed
        dbg!(res.iter().min().unwrap());
        dbg!(start.elapsed());
    }
    

    pub fn solution_2() {
        let input = include_str!("./inputs/day5/input.txt");
        let start = Instant::now();
        let solved = solve(input);
        dbg!(solved);
        dbg!(start.elapsed());
    }
    fn solve(input: &str) -> u64 {
        let input = Input::<MySeeds>::parse(input);
        let important_points = important_points(&input.maps);
        let p = important_points
            .iter()
            .filter(|p| input.seeds.entries.iter().any(|s| s.contains(**p)))
            .collect::<Vec<_>>();

        p.iter()
            .map(|seed| input.mapped_value(**seed))
            .min()
            .unwrap()
    }
    trait Seeds {
        fn parse(input: &str) -> Self;
        fn seeds(&self) -> Vec<u64>;
    }

    #[derive(Debug)]
    struct SeedEntry {
        seed_start: u64,
        count: u64,
    }

    impl SeedEntry {
        fn seeds(&self) -> Vec<u64> {
            (self.seed_start..self.seed_start + self.count).collect::<Vec<_>>()
        }

        fn contains(&self, p: u64) -> bool {
            p >= self.seed_start && p < self.seed_start + self.count
        }
    }

    #[derive(Debug)]
    struct MySeeds {
        entries: Vec<SeedEntry>,
    }

    impl Seeds for MySeeds {
        fn parse(input: &str) -> Self {
            let seeds = input
                .strip_prefix("seeds: ")
                .unwrap()
                .split_whitespace()
                .collect::<Vec<_>>();
    
            let entries = (0..seeds.len())
                .step_by(2)
                .map(|i| {
                    let seed_start = seeds[i].parse().unwrap();
                    let count = seeds[i + 1].parse().unwrap();
    
                    SeedEntry { seed_start, count }
                })
                .collect::<Vec<_>>();
    
            Self { entries }
        }

        fn seeds(&self) -> Vec<u64> {
            self.entries.iter().flat_map(|e| e.seeds()).collect()
        }
    }

    #[derive(Debug)]
    struct Input<SeedType: Seeds> {
        seeds: SeedType,
        maps: Vec<Map>,
    }
    impl<SeedType: Seeds> Input<SeedType> {
        fn parse(input: &str) -> Self {
            let mut sections = input.split("\r\n\r\n");

            let seeds = sections.next().unwrap();
            let seeds = SeedType::parse(seeds);

            let maps = sections.map(Map::parse).collect::<Vec<_>>();

            Self { seeds, maps }
        }

        fn mapped_value(&self, mut seed: u64) -> u64 {
            for map in &self.maps {
                let entry: Option<u64> = map.entries.iter().find_map(|e| e.translate_down(seed));
                seed = entry.unwrap_or(seed);
            }
            seed
        }
    }

    #[derive(Debug, Clone)]
    struct Map {
        entries: Vec<MapEntry>,
    }

    impl Map {
        fn parse(input: &str) -> Self {
            let mut lines = input.lines();
            let _ = lines.next().unwrap();

            let entries = lines.map(MapEntry::parse).collect::<Vec<_>>();

            Self { entries }
        }
    }
    #[derive(Debug, Clone)]
    struct MapEntry {
        dest_range_start: u64,
        source_range_start: u64,
        range_length: u64,
    }

    fn important_points(maps: &[Map]) -> Vec<u64> {
        let mut maps = maps.to_vec();
        maps.reverse();
        let maps = maps;

        let mut points = vec![];

        for m in maps {
            let mut translated_points = points
                .iter()
                .map(|p| {
                    m.entries
                        .iter()
                        .find_map(|e| e.translate_up(*p))
                        .unwrap_or(*p)
                })
                .collect::<Vec<_>>();
            let mut new_points = m
                .entries
                .iter()
                .map(|e| e.source_range_start)
                .collect::<Vec<_>>();
            translated_points.append(&mut new_points);

            points = translated_points;
        }

        points
    }

    impl MapEntry {
        fn parse(l: &str) -> Self {
            let nums = l
                .split_whitespace()
                .map(|n| n.parse().unwrap())
                .collect::<Vec<u64>>();

            Self {
                dest_range_start: nums[0],
                source_range_start: nums[1],
                range_length: nums[2],
            }
        }

        fn translate_down(&self, seed: u64) -> Option<u64> {
            if seed >= self.source_range_start && seed < self.source_range_start + self.range_length
            {
                let offset = seed - self.source_range_start;
                Some(self.dest_range_start + offset)
            } else {
                None
            }
        }

        fn translate_up(&self, seed: u64) -> Option<u64> {
            if seed >= self.dest_range_start && seed < self.dest_range_start + self.range_length {
                let offset = seed - self.dest_range_start;
                Some(self.source_range_start + offset)
            } else {
                None
            }
        }
    }
}
