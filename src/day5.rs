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
    pub fn test() {
        
        static LINES: &str = include_str!("./inputs/day5/input.txt");
        // let lines: Vec<&str> = LINES.split("\r\n\r\n").collect();
        let seeds: Vec<u64> = LINES.lines().collect::<Vec<&str>>()[0]
            .split(": ")
            .collect::<Vec<&str>>()[1]
            .split_ascii_whitespace()
            .map(|s| s.parse::<u64>().unwrap())
            .collect();
        let mut maps: Vec<Vec<Vec<u64>>> = Vec::new(); // all_lines { step { step_line { instructions(u64) } } }
        let mut lines: Vec<&str> = LINES.split("\r\n\r\n").skip(1).collect::<Vec<&str>>();
        let llines: Vec<_> = lines
            .iter()
            .map(|s| s.split("\r\n").collect::<Vec<&str>>())
            .collect();
        dbg!(&lines);
        // for testing in lines.iter().
        let mut steps: [&str; 7] = [""; 7];
        for (i, line) in llines.iter().enumerate() {
            let mut testing: Vec<Vec<u64>> = Vec::new();
            for (idx, l) in line.iter().enumerate() {
                if idx == 0 {
                    steps[i] = l.split_ascii_whitespace().collect::<Vec<&str>>()[0]
                } else {
                    let ll: Vec<u64> = l
                        .split_ascii_whitespace()
                        .map(|n| n.parse::<u64>().unwrap())
                        .collect::<Vec<u64>>();
                    // maps.push(ll);
                    testing.push(ll);
                    // dbg!(ll);
                }
            }
            maps.push(testing);
        }
        let mut res: Vec<u64> = Vec::new();
        for seed in seeds {
            // let seed = 13;
            let mut penis = seed;
            for i in 0..steps.len() {
                for j in 0..maps[i].len() {
                    let (n, in_range) = num_for_next(penis, maps[i][j].clone());
                    penis = n;
                    if in_range {
                        break;
                    }
                }
                // print!("\n");
                // dbg!(penis);
                // print!("\n");
            }
            res.push(penis);
        }

        dbg!(res.iter().min().unwrap());
        // dbg!(steps);
        // dbg!(&maps[0]);
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
            let mut penis = *seed;

            for (_, step) in maps.iter().enumerate() {
                for j in step {
                    let (n, in_range) = num_for_next(penis, j.clone());
                    penis = n;
                    if in_range {
                        break;
                    }
                }
            }

            res.push(penis);
        }

        // Print or use res as needed
        dbg!(res.iter().min().unwrap());
        dbg!(start.elapsed());
    }

    pub fn solution_2() {
        let lines = include_str!("./inputs/day5/input.txt");
    }
}
