use grid::*;

fn visible_top(grid: &Grid<u32>, row: usize, col: usize) -> bool {
    let tree_size = grid.get(row, col).unwrap();

    for r in 0..row {
        if grid.get(r, col).unwrap().ge(tree_size) {
            return false;
        }
    }
    true
}

fn visible_bottom(grid: &Grid<u32>, row: usize, col: usize) -> bool {
    let tree_size = grid.get(row, col).unwrap();
    for r in row + 1..grid.rows() {
        if grid.get(r, col).unwrap().ge(tree_size) {
            return false;
        }
    }
    true
}

fn visible_left(grid: &Grid<u32>, row: usize, col: usize) -> bool {
    let tree_size = grid.get(row, col).unwrap();
    for c in 0..col {
        if grid.get(row, c).unwrap().ge(tree_size) {
            return false;
        }
    }
    true
}

fn visible_right(grid: &Grid<u32>, row: usize, col: usize) -> bool {
    let tree_size = grid.get(row, col).unwrap();
    for c in col + 1..grid.cols() {
        if grid.get(row, c).unwrap().ge(tree_size) {
            return false;
        }
    }

    true
}

fn is_visible(grid: &Grid<u32>, row: usize, col: usize) -> bool {
    if col == 0 || row == 0 || col == grid.cols() - 1 || row == grid.rows() - 1 {
        return true;
    }

    let top = visible_top(grid, row, col);
    let bottom = visible_bottom(grid, row, col);
    let left = visible_left(grid, row, col);
    let right = visible_right(grid, row, col);

     top | bottom | left | right
}

fn create_grid(input: &str) -> Grid<u32> {
    let lines: Vec<&str> = input.split("\n").collect();
    let col_length = lines.get(0).unwrap().len();

    let data = input.lines()
        .map(|line| line.chars())
        .flatten()
        .map(|t| t.to_digit(10).unwrap())
        .collect::<Vec<u32>>();
    Grid::from_vec(data, col_length)
}

fn count_visible(grid: &Grid<u32>) -> u32 {
    let mut result = 0;


    for row in 0..grid.rows() {
        for col in 0..grid.cols() {
            if is_visible(grid, row, col) {
                result += 1;
            }
        }
    }
    result
}


fn visible_trees_top(grid: &Grid<u32>, row: usize, col: usize) -> u32 {
    let mut result: u32 = 0;
    let tree_size = grid.get(row, col).unwrap();
    for r in (0..row).rev() {
        match grid.get(r, col) {
            None => {}
            Some(ts) => {
                result += 1;
                if ts >= tree_size {
                    break;
                }
            }
        }
    }
    return result;
}

fn visible_trees_bottom(grid: &Grid<u32>, row: usize, col: usize) -> u32 {
    let mut result: u32 = 0;
    let tree_size = grid.get(row, col).unwrap();
    for r in row + 1..grid.rows() {
        match grid.get(r, col) {
            None => {}
            Some(ts) => {
                result += 1;
                if ts >= tree_size {
                    break;
                }
            }
        }
    }
    return result;
}

fn visible_trees_left(grid: &Grid<u32>, row: usize, col: usize) -> u32 {
    let mut result: u32 = 0;
    let tree_size = grid.get(row, col).unwrap();
    for c in (0..col).rev() {
        match grid.get(row, c) {
            None => {}
            Some(ts) => {
                result += 1;
                if ts >= tree_size {
                    break;
                }
            }
        }
    }
    return result;
}

fn visible_trees_right(grid: &Grid<u32>, row: usize, col: usize) -> u32 {
    let mut result: u32 = 0;
    let tree_size = grid.get(row, col).unwrap();
    for c in (col + 1)..grid.cols() {
        match grid.get(row, c) {
            None => {}
            Some(ts) => {
                result += 1;
                if ts >= tree_size {
                    break;
                }
            }
        }
    }
    return result;
}


fn scenic_score(grid: &Grid<u32>, row: usize, col: usize) -> u32 {
    visible_trees_top(grid, row, col) * visible_trees_left(grid, row, col) * visible_trees_bottom(grid, row, col) * visible_trees_right(grid, row, col)
}

fn highest_scenic_score(grid: &Grid<u32>) -> u32 {
    let mut result = 0;
    for row in 0..grid.rows() {
        for col in 0..grid.cols() {
            let score = scenic_score(grid, row, col);
            if result < score {
                result = score;
            }
        }
    }

    result
}


#[aoc(day8, part1)]
fn part1(input: &str) -> u32 {
    let grid = create_grid(input);
    count_visible(&grid)
}

#[aoc(day8, part2)]
fn part2(input: &str) -> u32 {
    let grid = create_grid(input);
    highest_scenic_score(&grid)
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse() {
        let input = r"30373
25512
65332
33549
35390";


        let expected = Grid::from_vec(
            [
                3, 0, 3, 7, 3, 2, 5, 5, 1, 2, 6, 5, 3, 3, 2, 3, 3, 5, 4, 9, 3, 5, 3, 9, 0,
            ]
                .to_vec(),
            5,
        );

        assert_eq!(5, expected.cols());
        assert_eq!(5, expected.rows());
        assert_eq!(create_grid(input), expected);
    }

    #[test]
    fn test_is_visible_outer() {
        let grid: Grid<u32> = Grid::from_vec(
            [
                3, 0, 3, 7, 3, 2, 5, 5, 1, 2, 6, 5, 3, 3, 2, 3, 3, 5, 4, 9, 3, 5, 3, 9, 0,
            ]
                .to_vec(),
            5,
        );

        assert_eq!(is_visible(&grid, 0, 0), true)
    }

    #[test]
    fn test_is_visible_example1() {
        let grid: Grid<u32> = Grid::from_vec(
            [
                3, 0, 3, 7, 3, 2, 5, 5, 1, 2, 6, 5, 3, 3, 2, 3, 3, 5, 4, 9, 3, 5, 3, 9, 0,
            ]
                .to_vec(),
            5,
        );

        assert_eq!(is_visible(&grid, 1, 1), true)
    }

    #[test]
    fn test_is_visible_false_example1() {
        let grid: Grid<u32> = Grid::from_vec(
            [
                3, 0, 3, 7, 3, 2, 5, 5, 1, 2, 6, 5, 3, 3, 2, 3, 3, 5, 4, 9, 3, 5, 3, 9, 0,
            ]
                .to_vec(),
            5,
        );

        assert_eq!(is_visible(&grid, 1, 3), false)
    }

    #[test]
    fn test_is_visible_false_example2() {
        let grid: Grid<u32> = Grid::from_vec(
            [
                3, 0, 3, 7, 3, 2, 5, 5, 1, 2, 6, 5, 3, 3, 2, 3, 3, 5, 4, 9, 3, 5, 3, 9, 0,
            ]
                .to_vec(),
            5,
        );

        assert_eq!(is_visible(&grid, 3, 3), false)
    }

    #[test]
    fn test_check() {
        let grid: Grid<u32> = Grid::from_vec(
            [
                3, 0, 3, 7, 3, 2, 5, 5, 1, 2, 6, 5, 3, 3, 2, 3, 3, 5, 4, 9, 3, 5, 3, 9, 0,
            ]
                .to_vec(),
            5,
        );

        assert_eq!(visible_top(&grid, 3, 1), false);
        assert_eq!(visible_bottom(&grid, 3, 1), false);
        assert_eq!(visible_left(&grid, 3, 1), false);
        assert_eq!(visible_right(&grid, 3, 1), false);
    }

    #[test]
    fn test_example_day_1() {
        let grid: Grid<u32> = Grid::from_vec(
            [
                3, 0, 3, 7, 3, 2, 5, 5, 1, 2, 6, 5, 3, 3, 2, 3, 3, 5, 4, 9, 3, 5, 3, 9, 0,
            ]
                .to_vec(),
            5,
        );

        assert_eq!(count_visible(&grid), 21)
    }

    #[test]
    fn test_is_visible() {
        let tests_from_description: Vec<(usize, usize, bool)> =
            vec![(1, 1, true), (1, 3, false), (3, 1, false)];

        let grid: Grid<u32> = Grid::from_vec(
            [
                3, 0, 3, 7, 3, 2, 5, 5, 1, 2, 6, 5, 3, 3, 2, 3, 3, 5, 4, 9, 3, 5, 3, 9, 0,
            ]
                .to_vec(),
            5,
        );

        tests_from_description
            .iter()
            .for_each(|(row, col, expected)| {
                assert_eq!(
                    is_visible(&grid, row.to_owned(), col.to_owned()),
                    expected.to_owned()
                )
            });
    }

    #[test]
    fn test_example_day_2() {
        let grid: Grid<u32> = Grid::from_vec(
            [
                3, 0, 3, 7, 3, 2, 5, 5, 1, 2, 6, 5, 3, 3, 2, 3, 3, 5, 4, 9, 3, 5, 3, 9, 0,
            ]
                .to_vec(),
            5,
        );

        assert_eq!(highest_scenic_score(&grid), 8)
    }

    #[test]
    fn test_visible_trees() {
        let grid: Grid<u32> = Grid::from_vec(
            [
                3, 0, 3, 7, 3, 2, 5, 5, 1, 2, 6, 5, 3, 3, 2, 3, 3, 5, 4, 9, 3, 5, 3, 9, 0,
            ]
                .to_vec(),
            5,
        );

        assert_eq!(visible_trees_top(&grid, 1, 2), 1);
        assert_eq!(visible_trees_bottom(&grid, 1, 2), 2);
        assert_eq!(visible_trees_left(&grid, 1, 2), 1);
        assert_eq!(visible_trees_right(&grid, 1, 2), 2);
        assert_eq!(scenic_score(&grid, 1, 2), 4);
    }


    #[test]
    fn test_visible_trees_2() {
        let grid: Grid<u32> = Grid::from_vec(
            [
                3, 0, 3, 7, 3, 2, 5, 5, 1, 2, 6, 5, 3, 3, 2, 3, 3, 5, 4, 9, 3, 5, 3, 9, 0,
            ]
                .to_vec(),
            5,
        );

        assert_eq!(visible_trees_top(&grid, 3, 2), 2);
        assert_eq!(visible_trees_bottom(&grid, 3, 2), 1);
        assert_eq!(visible_trees_left(&grid, 3, 2), 2);
        assert_eq!(visible_trees_right(&grid, 3, 2), 2);
        assert_eq!(scenic_score(&grid, 3, 2), 8);
    }
}
