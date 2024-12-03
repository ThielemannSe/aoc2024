pub fn process(input: &str) -> Result<String, ()> {
    let num_valid_lines = input
        .lines()
        .filter(|line| {
            let numbers: Vec<i32> = line
                .split_whitespace()
                .map(|number| number.parse().unwrap())
                .collect();

            (is_strictly_ordered(&numbers) && has_max_gap_3(&numbers))
                || is_sorted_by_removal(&numbers)
        })
        .count();

    Ok(num_valid_lines.to_string())
}

fn is_strictly_ordered(numbers: &[i32]) -> bool {
    numbers.windows(2).all(|pair| pair[0] > pair[1])
        || numbers.windows(2).all(|pair| pair[0] < pair[1])
}

fn has_max_gap_3(numbers: &[i32]) -> bool {
    numbers
        .windows(2)
        .all(|pair| (pair[0] - pair[1]).abs() <= 3)
}

fn is_sorted_by_removal(numbers: &[i32]) -> bool {
    for i in 0..numbers.len() {
        let mut numbers = numbers.to_vec();
        numbers.remove(i);

        if is_strictly_ordered(&numbers) && has_max_gap_3(&numbers) {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

        assert_eq!(process(input), Ok("4".to_string()));
    }
}
