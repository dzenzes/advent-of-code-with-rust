use std::{
    collections::HashSet,
};

enum Direction {
    Left,
    Right,
    Up,
    Down,
}

struct Motion {
    direction: Direction,
    steps: usize,
}

type Coordinates = (i32, i32);

struct Rope {
    tail_positions: Vec<Coordinates>,
}

impl Rope {
    fn tail(&self) -> Coordinates {
        *self.tail_positions.last().unwrap()
    }

    fn pull_tail(&mut self) {
        let mut previous_position = *self.tail_positions.first().unwrap();

        for tail_position in self.tail_positions.iter_mut().skip(1) {
            let (distance_x, distance_y) = (previous_position.0 - tail_position.0, previous_position.1 - tail_position.1);

            if distance_x.abs() > 1 || distance_y.abs() > 1 {
                tail_position.0 += distance_x.signum();
                tail_position.1 += distance_y.signum();
            }

            previous_position = *tail_position;
        }
    }

    fn step(&mut self, direction: &Direction) {
        let mut head = self.tail_positions.first_mut().unwrap();

        match direction {
            Direction::Left => head.0 -= 1,
            Direction::Right => head.0 += 1,
            Direction::Up => head.1 += 1,
            Direction::Down => head.1 -= 1,
        }

        self.pull_tail();
    }
}


#[aoc_generator(day9)]
fn input_generator(input: &str) -> Vec<Motion> {
    input
        .split("\n")
        .map(|motion| {
            let mut tokens = motion.split_ascii_whitespace();

            Motion {
                direction: match tokens.next().unwrap() {
                    "L" => Direction::Left,
                    "R" => Direction::Right,
                    "U" => Direction::Up,
                    "D" => Direction::Down,
                    _ => unreachable!(),
                },
                steps: tokens.next().unwrap().parse().unwrap(),
            }
        })
        .collect()
}

#[aoc(day9, part1)]
fn part1(series_of_motions: &[Motion]) -> usize {
    let mut rope = Rope {
        tail_positions: vec![(0, 0); 2],
    };

    let mut tail_visited = HashSet::with_capacity(series_of_motions.len());
    tail_visited.insert((0, 0));

    for motion in series_of_motions {
        for _ in 0..motion.steps {
            rope.step(&motion.direction);
            tail_visited.insert(rope.tail());
        }
    }

    tail_visited.len()
}

#[aoc(day9, part2)]
fn part2(series_of_motions: &[Motion]) -> usize {
    let mut rope = Rope {
        tail_positions: vec![(0, 0); 10],
    };

    let mut tail_visited = HashSet::with_capacity(series_of_motions.len());
    tail_visited.insert((0, 0));

    for motion in series_of_motions {
        for _ in 0..motion.steps {
            rope.step(&motion.direction);
            tail_visited.insert(rope.tail());
        }
    }

    tail_visited.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT_1: &str = r"R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

    static TEST_INPUT_2: &str = r"R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&input_generator(TEST_INPUT_1)), 13);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&input_generator(TEST_INPUT_1)), 1);
        assert_eq!(part2(&input_generator(TEST_INPUT_2)), 36);
    }
}