use day_02::part2::process;

fn main() {
    let file = std::fs::read_to_string("day-02/input.txt").unwrap();

    let result = process(&file).unwrap();
    println!("{}", result)
}
