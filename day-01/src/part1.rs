pub fn process(input: &str) -> Result<String, ()> {
    let (mut left, mut right): (Vec<i32>, Vec<i32>) = input
        .lines()
        .map(|line| {
            let mut parts = line.split_whitespace();
            let left = parts.next().unwrap().parse::<i32>().unwrap();
            let right = parts.next().unwrap().parse::<i32>().unwrap();
            (left, right)
        })
        .unzip();

    left.sort();
    right.sort();

    let result: i32 = std::iter::zip(left, right)
        .map(|(l, r)| (l - r).abs())
        .sum();

    Ok(result.to_string())
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

        assert_eq!(process(input), Ok("11".to_string()))
    }
}
