pub mod day2 {
    pub fn solution_1() {
        let input = include_str!("inputs/day2/input.txt");
        let games = parse_games(input);

        const RED_MAX: u32 = 12;
        const GREEN_MAX: u32 = 13;
        const BLUE_MAX: u32 = 14;

        let p_sum: u32 = games
            .iter()
            .filter(|g| g.possible(RED_MAX, GREEN_MAX, BLUE_MAX))
            .map(|g| g.no)
            .sum();

        println!("{}", p_sum)
    }

    pub fn solution_2() {
        let input = include_str!("inputs/day2/input.txt");
        let games = parse_games(input);

        let pow = games
            .iter()
            .map(|g| g.get_min_pow())
            .sum::<u32>();

        println!("{}", pow);
    }

    struct GameSet {
        green: u32,
        red: u32,
        blue: u32,
    }

    impl GameSet {
        fn possible(&self, red: u32, green: u32, blue: u32) -> bool {
            self.red <= red && self.green <= green && self.blue <= blue
        }
    }

    struct Game {
        no: u32,
        sets: Vec<GameSet>,
    }

    impl Game {
        fn possible(&self, red: u32, green: u32, blue: u32) -> bool {
            self.sets.iter().all(|set| set.possible(red, green, blue))
        }
        fn get_min_pow(&self) -> u32 {
            // get the max amount of cubes of each color
            let mut red: u32 = 0;
            let mut green: u32 = 0;
            let mut blue: u32 = 0;

            for set in &self.sets {
                if set.red > red {
                    red = set.red;
                }
                if set.green > green {
                    green = set.green;
                }
                if set.blue > blue {
                    blue = set.blue;
                }
            }

            // return the 'power' of the max set
            red * green * blue
        }
    }
    fn game_no(input: &str) -> u32 {
        let mut game_no_string: String = String::new();
        for c in input.chars() {
            if c.is_numeric() {
                game_no_string.push(c);
            } else if c == ':' {
                break;
            }
        }

        game_no_string.parse::<u32>().unwrap()
    }

    fn parse_set(input: &str) -> GameSet {
        let mut green: u32 = 0;
        let mut red: u32 = 0;
        let mut blue: u32 = 0;

        // split by ',' to get the individual cubes
        let cubes_split = input.split(',');

        // parse every cube into an instance of GameSet
        for cube in cubes_split {
            let cube_split: Vec<&str> = cube.trim().split(' ').collect();
            let num = cube_split[0].parse::<u32>().unwrap();
            let color = cube_split[1];

            match color {
                "green" => green = num,
                "red" => red = num,
                "blue" => blue = num,
                _ => panic!("Invalid color: {}", color),
            }
        }

        GameSet { green, red, blue }
    }
    fn parse_game(input: &str) -> Game {
        let game_no = game_no(input);

        // split by ':' to get rid of the game no
        let mut input_split = input.split(':');

        // split by ';' to get the sets
        let sets_split = input_split.nth(1).unwrap().split(';');

        // parse each set into an instance of GameSet
        let sets: Vec<GameSet> = sets_split.map(|set| parse_set(set)).collect();

        Game {
            no: game_no,
            sets: sets,
        }
    }
    fn parse_games(input: &str) -> Vec<Game> {
        input.lines().map(|line| parse_game(line)).collect()
    }
}
