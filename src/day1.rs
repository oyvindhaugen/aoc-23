pub mod day1 {
    use regex::Regex;
    pub fn solution_1() {
        let mut sum: i32 = 0;
        let file: Vec<&str> = include_str!("inputs/day_1/input.txt").lines().collect();
        for line in file {
            let content: Vec<char> = line.chars().collect();
            let mut num_line: Vec<String> = vec![];
            for char in content {
                match check_validity(char) {
                    Some(res) => num_line.push(res),
                    _ => (),
                }
            }
            let mut placeholder: i32 = 0;
            if num_line.len() == 1 {
               placeholder += num_line[0].parse::<i32>().unwrap() * 10 + num_line[0].parse::<i32>().unwrap();
            } else if num_line.len() > 2 {
               num_line.drain(1..num_line.len() - 1);
               placeholder += num_line[0].parse::<i32>().unwrap() * 10 + num_line[1].parse::<i32>().unwrap();
            } else {
                placeholder += num_line[0].parse::<i32>().unwrap() * 10 + num_line[1].parse::<i32>().unwrap();
            }
            sum += placeholder;
        }
        println!("{}", sum)
    }
    fn check_validity(c: char) -> Option<String> {
        let regex = Regex::new(r"[0-9]+").unwrap();
        let epic = regex.is_match(c.to_string().as_str());
        if epic {
            return Some(c.to_string())
        }
        return None
    }
    pub fn solution_2() {
        use aho_corasick::AhoCorasick;
        let file = include_str!("inputs/day_1/input.txt");
        let patterns = &["one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "1", "2", "3", "4", "5", "6", "7", "8", "9"];
        let alg = AhoCorasick::new(patterns).unwrap();

        let result = file
            .lines()
            .map(|line| {
                let mut matches = alg
                    .find_overlapping_iter(line)
                    .map(|m| m.pattern().as_usize());

                let first = matches.next().unwrap() % 9;
                let last = matches.last().unwrap_or(first) % 9;

                ((first + 1) * 10 + last + 1) as u32
            })
            .sum::<u32>();
        println!("{}", result)
    }
}
 