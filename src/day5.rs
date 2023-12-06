pub mod day5 {
    pub fn solution_1() {
    static LINES: &str = include_str!("./inputs/day5/input.txt");

    let seeds: Vec<u64> = LINES.lines().next().unwrap().split(": ").last().unwrap()
        .split_whitespace()
        .map(|s| s.parse::<u64>().unwrap())
        .collect();

    let mut maps: Vec<Vec<Vec<u64>>> = Vec::new();

    for group in LINES.split("\r\n\r\n").skip(1) {
        let mut testing: Vec<Vec<u64>> = Vec::new();
        for line in group.lines() {
            let parts: Vec<u64> = line.split_whitespace().map(|n| n.parse::<u64>().unwrap()).collect();
            testing.push(parts);
        }
        maps.push(testing);
    }

    let mut res: Vec<u64> = Vec::new();

    for seed in &seeds {
        let mut curr = *seed;
        for (i, step) in maps.iter().enumerate() {
            for step_line in step {
                let (n, in_range) = num_for_next(curr, step_line.clone());
                curr = n;
                if in_range {
                    break;
                }
            }
        }
        res.push(curr);
    }

    dbg!(res.iter().min());
}

fn num_for_next(seed: u64, step_line: Vec<u64>) -> (u64, bool) {
    if (step_line[1]..step_line[1] + step_line[2]).contains(&seed) {
        for i in step_line[1]..step_line[1] + step_line[2] {
            if seed == step_line[1] + (i - step_line[1]) {
                return (step_line[0] + (i - step_line[1]), true);
            }
        }
    }
    (seed, false)
}


    pub fn solution_2() {
        let lines = include_str!("./inputs/day5/input.txt");
    }
}
