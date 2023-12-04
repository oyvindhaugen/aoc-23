pub mod day4 {
    use std::{collections::HashSet, iter::once, thread::sleep};
    pub fn solution_1() {
        let lines = include_str!("./inputs/day4/input.txt");

        let result: u32 = lines
            .lines()
            .map(|line| {
                // let card_id = &line.split(":").last();
                let line = &line.split(":").collect::<Vec<&str>>()[1]
                    .trim()
                    .split("|")
                    .collect::<Vec<&str>>();
                let mut winning_nums_temp: Vec<&str> = line[0].trim().split(" ").collect();
                winning_nums_temp.retain(|s| !s.is_empty());
                let winning_nums = winning_nums_temp
                    .iter()
                    .map(|n| n.parse::<u32>().unwrap())
                    .collect::<Vec<u32>>();
                let mut my_nums: Vec<&str> = line[1].trim().split(" ").collect();
                my_nums.retain(|s| !s.is_empty());
                let my_nums_pog = my_nums
                    .iter()
                    .map(|n| n.parse::<u32>().unwrap())
                    .collect::<Vec<u32>>();
                let mut card_sum = 0;
                for num in my_nums_pog {
                    if winning_nums.contains(&num) {
                        if card_sum == 0 {
                            card_sum += 1
                        } else {
                            card_sum *= 2
                        }
                    }
                }

                card_sum
            })
            .sum::<u32>();

        println!("Part 1: {:?}", result)
    }

    /*
    functioning solution, but wanted to do it all in one function bc funny



    struct ScratchCard {
           winning_nums: HashSet<u8>,
           my_nums: HashSet<u8>,
       }

       impl ScratchCard {
           fn new(s: &str) -> ScratchCard {
               let (_, nums) = s.split_once(": ").expect("colon");
               let (winning, my) = nums.split_once(" | ").expect("pipe");

               fn parse_nums(s: &str) -> HashSet<u8> {
                   s.trim()
                       .split_ascii_whitespace()
                       .map(|s| s.parse::<u8>().expect("failed to parse string"))
                       .collect()
               }

               let winning_nums = parse_nums(winning);
               let my_nums = parse_nums(my);

               ScratchCard {
                   winning_nums,
                   my_nums,
               }
           }

           fn matching_nums(&self) -> usize {
               self.winning_nums.intersection(&self.my_nums).count()
           }
       }
       pub fn solution_2() {
           static LINES: &str = include_str!("./inputs/day4/input.txt");

           let cards: Vec<_> = LINES.lines().map(ScratchCard::new).collect();
           let original_cards: Vec<_> = cards.iter().map(ScratchCard::matching_nums).collect();

           let mut copies: Vec<_> = cards.iter().map(|_| 1).collect();

           for (i, value) in original_cards.iter().enumerate() {
               for j in 0..*value {
                   let combination = i + 1 + j as usize;
                   let amount = copies[i];
                   copies[combination] += amount;
               }
           }

           let result: usize = copies.iter().sum();
           println!("Total: {}", result);
       }*/
       pub fn solution_2() {
        // Read the content of the file "./inputs/day4/input.txt" into a static string
        static LINES: &str = include_str!("./inputs/day4/input.txt");
        
        // Split the static string into a vector of string slices, where each slice corresponds to a line in the file
        let lines: Vec<&str> = LINES.lines().collect();
    
        // Initialize a vector to store counts for each line
        let mut c_count: Vec<u32> = vec![1; lines.len()];
    
        // Iterate over the lines, enumerate them, and destructure the tuple into `num` and `line`
        for (num, line) in lines.into_iter().enumerate() {
            // Split the line by ":" and take the last part, then split it by "|" and create an iterator
            let mut s_line = line.split(":").last().unwrap().split("|").into_iter();
            
            // Take the first part of the iterator as keys and the second part as values
            let s_keys = s_line.next().unwrap();
            let s_values = s_line.next().unwrap();
    
            // Create vectors to store the keys and values
            let mut keys = Vec::new();
            let mut values = Vec::new();
    
            // Parse each space-separated key into u32 and push it to the keys vector
            for k in s_keys.split_ascii_whitespace() {
                keys.push(k.parse::<u32>().unwrap());
            }
    
            // Parse each space-separated value into u32 and push it to the values vector
            for v in s_values.split_ascii_whitespace() {
                values.push(v.parse::<u32>().unwrap());
            }
    
            // Count the number of values that are also keys
            let count = values.into_iter().filter(|x| keys.contains(x)).count();
    
            // If there are matching values, update counts for subsequent lines
            if count != 0 {
                for i in num + 1..num + count + 1 {
                    c_count[i] += c_count[num];
                }
            }
        }
    
        // Sum up the counts to get the final result
        let res: u32 = c_count.into_iter().sum();
        
        // Print the result
        println!("Part 2: {:?}", res);
    }
}