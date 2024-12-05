use nom::{
    bytes::complete::tag,
    character::complete::{self, anychar},
    multi::{many1, many_till},
    sequence::{delimited, separated_pair},
    IResult, Parser,
};

pub fn process(input: &str) -> Result<String, ()> {
    let instructions = parse(input).map_err(|_| ())?.1;

    let result = instructions
        .iter()
        .map(|ins| match ins {
            Instruction::Mul { lhs, rhs } => lhs * rhs,
        })
        .sum::<u32>();

    Ok(result.to_string())
}

enum Instruction {
    Mul { lhs: u32, rhs: u32 },
}

fn instruction(input: &str) -> IResult<&str, Instruction> {
    let (input, _) = tag("mul")(input)?;
    let (input, pair) = delimited(
        tag("("),
        separated_pair(complete::u32, tag(","), complete::u32),
        tag(")"),
    )(input)?;

    Ok((
        input,
        Instruction::Mul {
            lhs: pair.0,
            rhs: pair.1,
        },
    ))
}

fn parse(input: &str) -> IResult<&str, Vec<Instruction>> {
    many1(many_till(anychar, instruction).map(|(_discard, ins)| ins))(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

        assert_eq!(
            process(&input),
            Ok((2 * 4 + 5 * 5 + 11 * 8 + 8 * 5).to_string())
        )
    }
}
