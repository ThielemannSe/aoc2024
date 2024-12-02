use day_01::part1::process;

fn main() {
    let file = std::fs::read_to_string("day-01/input.txt").unwrap();

    let result = process(&file).unwrap();
    println!("{}", result);
}
