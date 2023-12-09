pub mod day7 {
    use std::time::Instant;
    pub fn run() {
        let start = Instant::now();
        let input = String::from(include_str!("./inputs/day7/input.txt"));
        let res = solve(input);
        // I just baked them in together, couldn't be arsed to seperate them, but hey, it works :3
        println!("part 1: {}", res.0);
        println!("part 2: {}", res.1);
        println!("time spent: {:?}", start.elapsed());
    }
    struct Hand {
        bid: u32,
        strength: u32,
    }

    impl Hand {
        fn new(line: &str, with_jokers: bool) -> Self {
            let line_parts: Vec<_> = line.split_whitespace().collect();

            let mut card_ch = line_parts.iter().nth(0).unwrap().chars();
            let bid: u32 = line_parts.iter().nth(1).unwrap().parse().unwrap();

            let mut strength: u32 = 0;
            let mut jokers = 0;

            let mut cards: [u32; 13] = [0; 13];
            for i in 0..5 {
                if let Some(card) = card_ch.next() {
                    let value = match card {
                        'A' => 12,
                        'K' => 11,
                        'Q' => 10,
                        'J' => {
                            if with_jokers {
                                0
                            } else {
                                9
                            }
                        }
                        'T' => {
                            if with_jokers {
                                9
                            } else {
                                8
                            }
                        }
                        n => n.to_digit(10).unwrap() as u32 - (if with_jokers { 1 } else { 2 }),
                    };

                    if with_jokers && value == 0 {
                        jokers += 1;
                    } else {
                        cards[value as usize] += 1;
                    }

                    strength |= value << ((4 - i) * 4);
                }
            }

            cards.sort_unstable();
            let hand_type = match cards[12] + jokers {
                5 => 6,
                4 => 5,
                3 if cards[11] == 2 => 4,
                3 => 3,
                2 if cards[11] == 2 => 2,
                2 => 1,
                _ => 0,
            };

            strength |= hand_type << 20;

            Hand { bid, strength }
        }
    }
    fn solve(input: String) -> (usize, usize) {
        let p1 = calculate_winnings(&input, false);
        let p2 = calculate_winnings(&input, true);

        (p1, p2)
    }

    fn calculate_winnings(inputs: &str, with_jokers: bool) -> usize {
        let mut hands: Vec<Hand> = inputs
            .lines()
            .map(|line| Hand::new(line, with_jokers))
            .collect();
        hands.sort_unstable_by_key(|hand| hand.strength);

        hands
            .iter()
            .enumerate()
            .fold(0, |acc, (i, hand)| acc + ((i + 1) * hand.bid as usize))
            .into()
    }
}
