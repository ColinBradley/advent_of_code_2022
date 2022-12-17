use core::panic;

fn main() {
    let input = include_str!("../input.txt");

    let result = solve(input, PART_1_BUFFER_SIZE);
    println!("{result}");

    let result = solve(input, PART_2_BUFFER_SIZE);
    println!("{result}");
}

const PART_1_BUFFER_SIZE: usize = 4;
const PART_2_BUFFER_SIZE: usize = 14;

fn solve(input: &str, buffer_size: usize) -> usize {
    let input_chars = input.chars().collect::<Vec<_>>();

    for start_index in 0..input_chars.len() - buffer_size {
        let not_all_equal = (start_index..start_index + buffer_size)
            .flat_map(|index| {
                (index + 1..start_index + buffer_size)
                    .map(move |compare_index| (index, compare_index))
            })
            .map(|(index, compare_index)| input_chars.get(index) == input_chars.get(compare_index))
            .all(|is_match| !is_match);

        if not_all_equal {
            return start_index + buffer_size;
        }
    }

    panic!("uh oh");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1_part1() {
        let test_content = "bvwbjplbgvbhsrlpgdmjqwftvncz";
        let result = solve(test_content, PART_1_BUFFER_SIZE);

        assert_eq!(result, 5);
    }

    #[test]
    fn example2_part1() {
        let test_content = "nppdvjthqldpwncqszvftbrmjlhg";
        let result = solve(test_content, PART_1_BUFFER_SIZE);

        assert_eq!(result, 6);
    }

    #[test]
    fn example3_part1() {
        let test_content = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
        let result = solve(test_content, PART_1_BUFFER_SIZE);

        assert_eq!(result, 10);
    }

    #[test]
    fn example4_part1() {
        let test_content = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";
        let result = solve(test_content, PART_1_BUFFER_SIZE);

        assert_eq!(result, 11);
    }

    #[test]
    fn example0_part2() {
        let test_content = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
        let result = solve(test_content, PART_2_BUFFER_SIZE);

        assert_eq!(result, 19);
    }

    #[test]
    fn example1_part2() {
        let test_content = "bvwbjplbgvbhsrlpgdmjqwftvncz";
        let result = solve(test_content, PART_2_BUFFER_SIZE);

        assert_eq!(result, 23);
    }

    #[test]
    fn example2_part2() {
        let test_content = "nppdvjthqldpwncqszvftbrmjlhg";
        let result = solve(test_content, PART_2_BUFFER_SIZE);

        assert_eq!(result, 23);
    }

    #[test]
    fn example3_part2() {
        let test_content = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
        let result = solve(test_content, PART_2_BUFFER_SIZE);

        assert_eq!(result, 29);
    }

    #[test]
    fn example4_part2() {
        let test_content = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";
        let result = solve(test_content, PART_2_BUFFER_SIZE);

        assert_eq!(result, 26);
    }
}
