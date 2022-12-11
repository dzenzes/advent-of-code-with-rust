#[derive(Eq, PartialEq, Debug)]
struct Area {
    from: i32,
    to: i32,
}

impl Area {
    fn from(input: &str) -> Area {
        let mut fields = input.split("-");

        let from = fields.next().unwrap().parse::<i32>().unwrap();
        let to = fields.next().unwrap().parse::<i32>().unwrap();

        Area { from, to }
    }

    fn contains(&self, area: &Area) -> bool {
        self.from <= area.from && self.to >= area.to
    }

    fn overlaps(&self, area: &Area) -> bool {
        (self.to >= area.from && self.from <= area.from)
            || (self.from <= area.to && self.from >= area.from)
    }
}

fn pairs_where_one_range_includes_the_other(input: Vec<&str>) -> u32 {
    input
        .iter()
        .map(|line| get_areas(line))
        .map(|area_tuple| {
            if pair_contains_the_other(&area_tuple.0, &area_tuple.1) {
                1
            } else {
                0
            }
        })
        .sum()
}

fn get_areas(input: &str) -> (Area, Area) {
    let mut areas = input.split(",");

    let area_1 = Area::from(areas.next().unwrap());
    let area_2 = Area::from(areas.next().unwrap());

    (area_1, area_2)
}

fn pair_contains_the_other(area_1: &Area, area_2: &Area) -> bool {
    area_1.contains(area_2) || area_2.contains(area_1)
}

fn pairs_overlap(area_1: &Area, area_2: &Area) -> bool {
    area_1.overlaps(area_2)
}

fn pairs_where_ranges_overlap(input: Vec<&str>) -> u32 {
    input
        .iter()
        .map(|line| get_areas(line))
        .map(|area_tuple| {
            if pairs_overlap(&area_tuple.0, &area_tuple.1) {
                // println!("{} overlaps {}", &area_tuple.0, &area_tuple.1);
                1
            } else {
                0
            }
        })
        .sum()
}

#[aoc(day4, part1)]
fn part1(input: &str) -> u32 {
    let lines = input.split("\n")
        .collect();
    pairs_where_one_range_includes_the_other(lines)
}

#[aoc(day4, part2)]
fn part2(input: &str) -> u32 {
    let lines = input.split("\n").collect();
    pairs_where_ranges_overlap(lines)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pairs_where_one_range_includes_the_other() {
        let input = [
            "2-4,6-8",
            "2-3,4-5",
            "5-7,7-9",
            "2-8,3-7",
            "6-6,4-6",
            "2-6,4-8",
        ]
            .to_vec();

        let expected: u32 = 2;

        assert_eq!(pairs_where_one_range_includes_the_other(input), expected);
    }

    #[test]
    fn test_to_area() {
        assert_eq!(Area::from("2-4"), Area { from: 2, to: 4 })
    }

    #[test]
    fn test_to_areas() {
        assert_eq!(
            get_areas("2-4,6-8"),
            (Area { from: 2, to: 4 }, Area { from: 6, to: 8 })
        )
    }

    #[test]
    fn test_area_contains_negative() {
        let area_1 = Area { from: 2, to: 4 };
        let area_2 = Area { from: 6, to: 8 };

        assert_eq!(area_1.contains(&area_2), false);
        assert_eq!(area_2.contains(&area_1), false);
    }

    #[test]
    fn test_area_contains_positive() {
        let area_1 = Area { from: 2, to: 8 };
        let area_2 = Area { from: 3, to: 7 };

        assert_eq!(area_1.contains(&area_2), true);
        assert_eq!(area_2.contains(&area_1), false);
    }

    #[test]
    fn test_area_overlaps_negative() {
        let area_1 = Area { from: 2, to: 4 };
        let area_2 = Area { from: 6, to: 8 };

        assert_eq!(area_1.overlaps(&area_2), false);
    }

    #[test]
    fn test_area_overlaps_positive() {
        let area_1 = Area { from: 2, to: 8 };
        let area_2 = Area { from: 3, to: 7 };

        assert_eq!(area_1.overlaps(&area_2), true);
    }

    #[test]
    fn test_pair_contains_the_other() {
        let area_1 = Area { from: 2, to: 4 };
        let area_2 = Area { from: 6, to: 8 };

        assert_eq!(pair_contains_the_other(&area_1, &area_2), false);
    }

    #[test]
    fn test_pair_contains_the_other_positive() {
        let area_1 = Area { from: 2, to: 8 };
        let area_2 = Area { from: 3, to: 7 };

        assert_eq!(pair_contains_the_other(&area_1, &area_2), true);
    }

    #[test]
    fn test_pairs_overlap() {
        let area_1 = Area { from: 2, to: 4 };
        let area_2 = Area { from: 6, to: 8 };

        assert_eq!(pairs_overlap(&area_1, &area_2), false);
    }

    #[test]
    fn test_pairs_overlap_positive() {
        let area_1 = Area { from: 2, to: 8 };
        let area_2 = Area { from: 3, to: 7 };

        assert_eq!(pairs_overlap(&area_1, &area_2), true);
    }

    #[test]
    fn test_pairs_where_ranges_overlap() {
        let input = [
            "2-4,6-8",
            "2-3,4-5",
            "5-7,7-9",
            "2-8,3-7",
            "6-6,4-6",
            "2-6,4-8",
        ]
            .to_vec();

        assert_eq!(pairs_where_ranges_overlap(input), 4);
    }
}
