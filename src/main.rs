fn main() {
    sol_day7();
}
 
mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
fn sol_day1() {
    use day1::day1;
    // day1::solution_1();
    day1::solution_2();
}
fn sol_day2() {
    use day2::day2;
    // day2::solution_1();
    day2::solution_2();
}
fn sol_day3() {
    use day3::day3;
    day3::solution_1();
    day3::solution_2();
}
fn sol_day4() {
    use day4::day4;
    day4::solution_1();
    day4::solution_2();
}
fn sol_day5() {
    use day5::day5;
    // day5::test();
    day5::solution_1();
    day5::solution_2();
}

fn sol_day6() {
    use day6::day6;
    day6::run();
}
fn sol_day7() {
    use day7::day7;
    day7::run();
}

