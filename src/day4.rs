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
        static LINES: &str = include_str!("./inputs/day4/input.txt");
        let lines: Vec<&str> = LINES.lines().collect();

        let mut c_count: Vec<u32> = vec![1; lines.len()];

        for (num, line) in lines.into_iter().enumerate() {
            let mut s_line = line.split(":").last().unwrap().split("|").into_iter();
            let s_keys = s_line.next().unwrap(); //takes the first element of the iterator
            let s_values = s_line.next().unwrap(); // takes the second part of the iterator

            // create vectors for the keys and values to inhabit
            let mut keys = Vec::new(); 
            let mut values = Vec::new();

            // push the winning nums in as keys
            for k in s_keys.split_ascii_whitespace() {
                keys.push(k.parse::<u32>().unwrap());
            }

            // push your nums in as values
            for v in s_values.split_ascii_whitespace() {
                values.push(v.parse::<u32>().unwrap());
            }

            // get the count of original cards where the cards match
            let count = values.into_iter().filter(|x| keys.contains(x)).count();

            // if there are more than 0 cards
            if count != 0 {
                // here it iterates through the range (num + 1) to (num + count + 1)
                for i in num + 1..num + count + 1 {
                    // and adds to the card count at [i] the amount that's at [num]
                    c_count[i] += c_count[num]
                }
            }
        }
        let res: u32 = c_count.into_iter().sum();
        println!("Part 2: {:?}", res);
    }
}
