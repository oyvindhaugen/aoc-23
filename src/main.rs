fn main() {
    sol_day3();
}
 
mod day1;
mod day2;
mod day3;
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