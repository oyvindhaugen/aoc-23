pub mod day6 {
    use std::time::Instant;
    pub fn run() {
        let start = Instant::now();
        let input = include_str!("./inputs/day6/input.txt");
        // solution_1(input);
        // println!("Time spent Part 1: {:.2?}", start.elapsed());
        // solution_2(input);
        // println!("Time spent Part 2: {:.2?}", start.elapsed());
        optimized_p2(input);
        println!("Time spent Optimized Part 2: {:.2?}", start.elapsed());
    }
    fn solution_1(input: &str) {
        let splatted: Vec<Vec<u32>> = input
            .lines()
            .map(|line| {
                let i_line: &str = line.split(":").collect::<Vec<&str>>()[1];
                i_line
                    .trim()
                    .split_ascii_whitespace()
                    .map(|r| r.trim().parse::<u32>().unwrap())
                    .collect::<Vec<u32>>()
            })
            .collect();
        let times = &splatted[0];
        let distances = &splatted[1];

        let result: u32 = times
            .iter()
            .zip(distances)
            .map(|(max_time, max_distance)| {
                let mut options = 0;
                for time_spent_charging in 1..*max_time {
                    let remaining = max_time - time_spent_charging;
                    let speed = time_spent_charging;
                    let distance = speed * remaining;
                    if distance > *max_distance {
                        options += 1
                    }
                }

                options
            })
            .product();

        dbg!(result);
    }

    fn solution_2(input: &str) {
        let splatted: Vec<String> = input
            .lines()
            .map(|s| String::from(s.split(":").collect::<Vec<&str>>()[1].replace(" ", "")))
            .collect();
        let time = splatted.iter().map(|s| s.parse::<u64>().unwrap()).collect::<Vec<u64>>()[0];
        let distance = splatted.iter().map(|s| s.parse::<u64>().unwrap()).collect::<Vec<u64>>()[1];
        
        let result: u64 = std::iter::once(time).zip(std::iter::once(distance)).map(|(max_time, max_distance)| {
            let mut options = 0;
            for time_spent_charging in 1..max_time {
                let remainder = max_time - time_spent_charging;
                let speed = time_spent_charging;
                let dist = speed * remainder;
                if dist > max_distance {
                    options += 1
                }
            }
            options
        }).product();

        dbg!(result);
    }
    fn optimized_p2(input: &str) {
            let splatted: Vec<Vec<u64>> = input
                .lines()
                .map(|s| s.split_whitespace().skip(1).map(|s| s.parse::<u64>().unwrap()).collect())
                .collect();
        
            let result: u64 = splatted[0].iter().zip(splatted[1].iter()).map(|(&max_time, &max_distance)| {
                (1..max_time)
                    .filter(|&time_spent_charging| time_spent_charging * (max_time - time_spent_charging) > max_distance)
                    .count() as u64
            }).product();
        
            dbg!(result);        
    }
}
