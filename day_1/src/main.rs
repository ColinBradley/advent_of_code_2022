use std::iter;

fn main() {
    let stdin = std::io::stdin();
    let value = stdin
        .lines()
        .map(|line| line.unwrap())
        .collect::<Vec<String>>()
        .join("\n");

    let crate_stacks = solve(&value, true);

    let last_crates: String = get_last_crates_from_stacks(&crate_stacks);

    println!("{last_crates}");
}

fn solve(value: &str, keep_order_in_move: bool) -> Vec<Vec<char>> {
    let lines: Vec<&str> = value.split('\n').collect();
    let blank_line_index = lines
        .iter()
        .position(|line| line.is_empty())
        .expect("No blank line found in input");

    let crate_stacks_value = &lines[0..blank_line_index - 1];

    let mut crate_stacks = parse_crates(crate_stacks_value);

    let commands_value = &lines[blank_line_index + 1..];

    let commands = parse_commands(commands_value);

    for command in commands {
        let from_crates = crate_stacks
            .get_mut(command.from)
            .expect("Invalid from command");

        let mut creates_to_move = from_crates
            .drain(from_crates.len() - command.amount..)
            .collect::<Vec<_>>();

        if !keep_order_in_move {
            creates_to_move.reverse();
        }

        let to_crates = crate_stacks
            .get_mut(command.to)
            .expect("Invalid to command");

        to_crates.extend(creates_to_move);
    }

    crate_stacks
}

const CRATE_STRING_LENGTH: usize = 4;
fn parse_crates(lines: &[&str]) -> Vec<Vec<char>> {
    let crate_count = lines[0].len() / CRATE_STRING_LENGTH + 1;

    let mut crate_stacks = iter::repeat_with(Vec::<char>::new)
        .take(crate_count)
        .collect::<Vec<_>>();

    for line in lines {
        for crate_index in 0..crate_count {
            let string_start = crate_index * CRATE_STRING_LENGTH;
            let crate_char = line[string_start..string_start + CRATE_STRING_LENGTH - 1]
                .chars()
                .nth(1)
                .expect("Invalid crate format");

            if crate_char == ' ' {
                continue;
            }

            let crates = crate_stacks.get_mut(crate_index).expect("missing crate");
            crates.push(crate_char);
        }
    }

    for crate_stack in crate_stacks.iter_mut() {
        crate_stack.reverse();
    }

    crate_stacks
}

fn parse_commands(lines: &[&str]) -> Vec<Command> {
    lines
        .iter()
        .map(|line| {
            let numbers = line
                .split_ascii_whitespace()
                .map(|segment| segment.parse::<usize>())
                .filter_map(|parse_result| parse_result.ok())
                .collect::<Vec<_>>();

            Command {
                amount: *numbers.first().unwrap(),
                from: *numbers.get(1).unwrap() - 1,
                to: *numbers.get(2).unwrap() - 1,
            }
        })
        .collect()
}

fn get_last_crates_from_stacks(crate_stacks: &[Vec<char>]) -> String {
    crate_stacks
        .iter()
        .map(|crates| *crates.last().unwrap())
        .collect()
}

struct Command {
    amount: usize,
    from: usize,
    to: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_part1() {
        let test_content = include_str!("../test_input.txt");
        let crate_stacks = solve(test_content, false);

        assert_eq!(get_last_crates_from_stacks(&crate_stacks), "CMZ");
    }

    #[test]
    fn example_part2() {
        let test_content = include_str!("../test_input.txt");
        let crate_stacks = solve(test_content, true);

        assert_eq!(get_last_crates_from_stacks(&crate_stacks), "MCD");
    }
}
