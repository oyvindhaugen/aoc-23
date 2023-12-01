pub mod day1_test {
    pub fn solution() {
        let file: Vec<&str> = include_str!("inputs/day_1/input.txt").lines().collect();
        for line in file {
            let content: Vec<char> = line.chars().collect();
            // println!("{:?}", content)
        }
    }
}
