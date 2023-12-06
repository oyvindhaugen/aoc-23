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
        let lines = include_str!("./inputs/day5/input.txt");
    }
}
