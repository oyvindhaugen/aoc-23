pub mod day3 {
    use regex::Regex;
    use std::cmp::min;
    use std::collections::HashMap;
    
    pub fn solution_1() {
        /*
            small tl;dr

            here it checks for number adjacent to symbol and adds it to Vector and sums at the end.
        */
        let lines: Vec<String> = include_str!("./inputs/day3/input.txt").lines().map(|s|s.to_string()).collect();
        let re = Regex::new(r"\d+").unwrap();
        let mut sum = 0;
        
        for i in 0..lines.len() {
            let row = &lines[i];

            'outer: for number in re.find_iter(row) {
                //above
                if i > 0 {
                    let over = &lines[i - 1].as_bytes();

                    for j in number.start().saturating_sub(1)..min(over.len(), number.end() + 1) {
                        if over[j] != b'.' &&  !over[j].is_ascii_digit() {
                            sum += number.as_str().parse::<i32>().unwrap();
                            continue 'outer
                        }
                    }
                }

                //below
                if i < lines.len() - 1 {
                    let under = &lines[i + 1].as_bytes();
                    for j in number.start().saturating_sub(1)..min(under.len(), number.end() + 1) {
                        if under[j] != b'.' && !under[j].is_ascii_digit() {
                            sum += number.as_str().parse::<i32>().unwrap();
                            continue 'outer
                        } 
                    }
                }

                //left index
                if number.start() > 0 {
                    let left = row.as_bytes()[number.start() - 1];

                    if left != b'.' && !left.is_ascii_digit() {
                        sum += number.as_str().parse::<i32>().unwrap();
                        continue 'outer
                    }
                }

                //right index
                if number.end() < row.len() {
                    let right = row.as_bytes()[number.end()];

                    if right != b'.' && !right.is_ascii_digit() {
                        sum += number.as_str().parse::<i32>().unwrap();
                        continue 'outer
                    }
                }
            }
        }
        println!("Part 1: {}", sum);
    }
    pub fn solution_2() {
        /*
            small tl;dr:
            
            it looks for '*' in the line and adds it to the HashMap with the key (row_number, index) 
            and in the end it filters and sums together to get the final result  
        */
        let lines: Vec<String> = include_str!("./inputs/day3/input.txt").lines().map(|s|s.to_string()).collect();
        let re = Regex::new(r"\d+").unwrap();
        let mut gears: HashMap<(usize, usize), Vec<i32>> = HashMap::new();

        for i in 0..lines.len() {
            let row = &lines[i];

            for number in re.find_iter(row) {
                //above
                if i > 0 {
                    let over = &lines[i - 1].as_bytes();

                    for j in number.start().saturating_sub(1)..min(over.len(), number.end() + 1) {
                        if over[j] == b'*' {
                            gears.entry((i - 1, j)).or_default().push(number.as_str().parse::<i32>().unwrap());
                        }
                    }
                }

                //below
                if i < lines.len() - 1 {
                    let under = &lines[i + 1].as_bytes();
                    for j in number.start().saturating_sub(1)..min(under.len(), number.end() + 1) {
                        if under[j] == b'*' {
                            gears.entry((i + 1, j)).or_default().push(number.as_str().parse::<i32>().unwrap());
                        }
                    }
                }

                //left index
                if number.start() > 0 {
                    let left = row.as_bytes()[number.start() - 1];

                    if left == b'*' {
                        gears.entry((i, number.start() - 1)).or_default().push(number.as_str().parse::<i32>().unwrap());
                    }
                }

                //right index
                if number.end() < row.len() {
                    let right = row.as_bytes()[number.end()];

                    if right == b'*' {
                        gears.entry((i, number.end())).or_default().push(number.as_str().parse::<i32>().unwrap());
                    }
                }
            }
        }
        let mut sum = 0;
        gears.values().filter(|vec| vec.len() == 2).for_each(|vec| sum += vec[0] * vec[1]);

        println!("Part 2: {}", sum);
    }
}