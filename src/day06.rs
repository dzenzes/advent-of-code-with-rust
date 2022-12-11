use std::collections::HashSet;

fn no_duplicates(input: &str) -> bool {
    let char_vector: Vec<char> = input.chars().collect();
    let unique_characters: HashSet<char> = HashSet::from_iter(char_vector);
    let number_of_duplicates = input.len() - unique_characters.len();
    return number_of_duplicates == 0;
}

fn first_marker_at(input: &str, marker_size: usize) -> usize {
    for index in 0..input.len() {
        if index >= marker_size {
            let marker_candidate = &input[index - marker_size..index];
            if no_duplicates(&marker_candidate) {
                return index;
            }
        }
    }
    panic!("Couldn't find marker");
}

#[aoc(day6, part1)]
fn part1(input: &str) -> usize {
    first_marker_at(input, 4)
}

#[aoc(day6, part2)]
fn part2(input: &str) -> usize {
    first_marker_at(input, 14)
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example_1() {
        let input = "bvwbjplbgvbhsrlpgdmjqwftvncz";
        assert_eq!(first_marker_at(input, 4), 5);
    }

    #[test]
    fn test_example_2() {
        let input = "nppdvjthqldpwncqszvftbrmjlhg";
        assert_eq!(first_marker_at(input, 4), 6);
    }

    #[test]
    fn test_example_3() {
        let input = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
        assert_eq!(first_marker_at(input, 4), 10);
    }

    #[test]
    fn test_example_4() {
        let input = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";
        assert_eq!(first_marker_at(input, 4), 11);
    }

    // part 2
    #[test]
    fn test_example_2_1() {
        let input = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
        assert_eq!(first_marker_at(input, 14), 19);
    }

    #[test]
    fn test_example_2_2() {
        let input = "bvwbjplbgvbhsrlpgdmjqwftvncz";
        assert_eq!(first_marker_at(input, 14), 23);
    }

    #[test]
    fn test_example_2_3() {
        let input = "nppdvjthqldpwncqszvftbrmjlhg";
        assert_eq!(first_marker_at(input, 14), 23);
    }

    #[test]
    fn test_example_2_4() {
        let input = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
        assert_eq!(first_marker_at(input, 14), 29);
    }

    #[test]
    fn test_example_2_5() {
        let input = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";
        assert_eq!(first_marker_at(input, 14), 26);
    }
}
