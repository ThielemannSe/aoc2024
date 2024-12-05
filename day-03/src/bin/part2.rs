use day_03::part2::process;

fn main() {
    let file = std::fs::read_to_string("../input.txt").unwrap();

    let result = process(&file).unwrap();
    println!("{}", result);
}
