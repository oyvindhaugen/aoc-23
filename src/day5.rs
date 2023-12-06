pub mod day5 {
    pub fn solution_1() {
        static LINES: &str = include_str!("./inputs/day5/input.txt");
        let seeds: Vec<u64> = LINES.lines().collect::<Vec<&str>>()[0]
            .split(": ")
            .collect::<Vec<&str>>()[1]
            .split_ascii_whitespace()
            .map(|s| s.parse::<u64>().unwrap())
            .collect();
        let mut maps: Vec<Vec<Vec<u64>>> = Vec::new(); // all_lines { step { step_line { instructions(u64) } } }
        let mut lines: Vec<&str> = LINES.split("\r\n\r\n").collect::<Vec<&str>>();
        lines.remove(0);
        let llines: Vec<_> = lines
            .iter()
            .map(|s| s.split("\r\n").collect::<Vec<&str>>())
            .collect();
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
            let mut curr = seed;
            for i in 0..steps.len() {
                for j in 0..maps[i].len() {
                    let (n, in_range) = num_for_next(curr, maps[i][j].clone());
                    curr = n;
                    if in_range {
                        break;
                    }
                }
            }
            res.push(curr);
        }
        
        dbg!(res.iter().min().unwrap());
    }

    fn num_for_next(seed: u64, step_line: Vec<u64>) -> (u64, bool) {
        // dbg!(&seed);
        // dbg!(&step_line);
        // println!("{:#?} - {:#?}", &step_line[1], &step_line[1] + step_line[2] - 1);
        if (step_line[1]..step_line[1] + step_line[2]).contains(&seed) {
            for i in step_line[1]..step_line[1] + step_line[2] {
                // dbg!(i, i - step_line[1], step_line[0] + (i - step_line[1]));
                if seed == step_line[1] + (i - step_line[1]) {
                    // dbg!("please?");
                    return (step_line[0] + (i - step_line[1]), true)
                }
            }
        }
        (seed, false)
    }

    pub fn solution_2() {
        let lines = include_str!("./inputs/day5/input.txt");
    }
}
