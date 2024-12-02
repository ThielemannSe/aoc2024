pub fn process(input: &str) -> Result<String, ()> {
    let (left, right): (Vec<u64>, Vec<u64>) = input
        .lines()
        .map(|line| {
            let mut parts = line.split_whitespace();
            let left = parts.next().unwrap().parse::<u64>().unwrap();
            let right = parts.next().unwrap().parse::<u64>().unwrap();
            (left, right)
        })
        .unzip();

    let result: u64 = left
        .iter()
        .map(|number| number * right.iter().filter(|r| &number == r).count() as u64)
        .sum();

    return Ok(result.to_string());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "3   4
4   3
2   5
1   3
3   9
3   3";

        assert_eq!(process(input), Ok("31".to_string()))
    }
}
