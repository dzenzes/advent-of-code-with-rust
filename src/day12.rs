use grid::*;
use std::collections::{HashSet, VecDeque};

type Coordinates = (i32, i32);

const LOWEST_ELEVATION: u32 = 'a' as u32;
const HIGHEST_ELEVATION: u32 = 'z' as u32;

struct Map {
    elevations: Grid<u32>,
    current_position: Coordinates,
    best_signal_location: Coordinates,
}

impl Map {
    fn neighbor_elevations(&self, current_position: Coordinates) -> Vec<(Coordinates, u32)> {
        [(-1, 0), (1, 0), (0, -1), (0, 1)]
            .into_iter()
            .map(|(x, y)| (current_position.0 + x, current_position.1 + y))
            .filter(|(x, y)| {
                *x >= 0
                    && (*x as usize) < self.elevations.cols()
                    && *y >= 0
                    && (*y as usize) < self.elevations.rows()
            })
            .map(|(x, y)| {
                let elevation = self
                    .elevations
                    .get(y as usize, x as usize)
                    .unwrap()
                    .to_owned();
                ((x, y), elevation)
            })
            .collect()
    }
}

#[aoc_generator(day12)]
fn parse_input(input: &str) -> Map {
    let mut current_position = (0, 0);
    let mut best_signal_location = (0, 0);
    /*
       let lines = input.split("\n").collect::<Vec<&str>>();
       let rows = lines.len();
       let cols = lines.get(0).unwrap().len();
    */
    let mut elevations = Grid::new(0, 0);

    for (y, row) in input.lines().enumerate() {
        let row = row
            .chars()
            .enumerate()
            .map(|(x, col)| match col {
                'S' => {
                    current_position = (x as i32, y as i32);
                    LOWEST_ELEVATION
                }
                'E' => {
                    best_signal_location = (x as i32, y as i32);
                    HIGHEST_ELEVATION
                }
                col => col as u32,
            })
            .collect();

        elevations.push_row(row);
    }

    Map {
        elevations,
        current_position,
        best_signal_location,
    }
}

#[aoc(day12, part1)]
fn part1(map: &Map) -> usize {
    let current_position = map.current_position;
    let mut queue = VecDeque::from(vec![(current_position, LOWEST_ELEVATION, 0)]);
    let mut visited: HashSet<Coordinates> = HashSet::from([current_position]);

    while !queue.is_empty() {
        let (current_position, current_elevation, path_length) = queue.pop_front().unwrap();

        if current_position == map.best_signal_location {
            return path_length;
        }

        for (neighbor_position, neighbor_elevation) in map.neighbor_elevations(current_position) {
            if neighbor_elevation as i32 - current_elevation as i32 <= 1
                && !visited.contains(&neighbor_position)
            {
                queue.push_back((neighbor_position, neighbor_elevation, path_length + 1));
                visited.insert(neighbor_position);
            }
        }
    }

    0
}

#[aoc(day12, part2)]
fn part2(map: &Map) -> usize {
    let current_position = map.best_signal_location;
    let mut queue = VecDeque::from(vec![(current_position, HIGHEST_ELEVATION, 0)]);
    let mut visited: HashSet<Coordinates> = HashSet::from([current_position]);

    while !queue.is_empty() {
        let (current_position, current_elevation, path_length) = queue.pop_front().unwrap();

        if current_elevation == LOWEST_ELEVATION {
            return path_length;
        }

        for (neighbor_position, neighbor_elevation) in map.neighbor_elevations(current_position) {
            if current_elevation as i32 - neighbor_elevation as i32 <= 1
                && !visited.contains(&neighbor_position)
            {
                queue.push_back((neighbor_position, neighbor_elevation, path_length + 1));
                visited.insert(neighbor_position);
            }
        }
    }

    0
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = r"Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse_input(TEST_INPUT)), 31);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse_input(TEST_INPUT)), 29);
    }
}
