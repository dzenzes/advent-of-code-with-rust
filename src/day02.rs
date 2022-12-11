use std::fs::File;
use std::io::{BufRead, BufReader};

enum Strategy {
    SCISSOR,
    PAPER,
    ROCK,
}

impl Strategy {
    fn counter_strategy(&self, outcome: &GameOutcome) -> Strategy {
        match outcome {
            GameOutcome::LOSE => match &self {
                Strategy::SCISSOR => Strategy::PAPER,
                Strategy::PAPER => Strategy::ROCK,
                Strategy::ROCK => Strategy::SCISSOR,
            },

            GameOutcome::DRAW => match &self {
                Strategy::SCISSOR => Strategy::SCISSOR,
                Strategy::PAPER => Strategy::PAPER,
                Strategy::ROCK => Strategy::ROCK,
            },

            GameOutcome::WIN => match &self {
                Strategy::SCISSOR => Strategy::ROCK,
                Strategy::PAPER => Strategy::SCISSOR,
                Strategy::ROCK => Strategy::PAPER,
            },
        }
    }
}

enum GameOutcome {
    WIN,
    LOSE,
    DRAW,
}

pub struct Duel {
    elf_strategy: Strategy,
    own_strategy: Strategy,
}

pub struct DuelWithOutcome {
    elf_strategy: Strategy,
    outcome: GameOutcome,
}

const LOST_GAME_POINTS: u32 = 0;
const DRAW_GAME_POINTS: u32 = 3;
const WON_GAME_POINTS: u32 = 6;

const ROCK_POINTS: u32 = 1;
const PAPER_POINTS: u32 = 2;
const SCISSOR_POINTS: u32 = 3;

impl Duel {
    fn create(elf_raw: &str, own_raw: &str) -> Duel {
        let elf_strategy = match elf_raw {
            "A" => Strategy::ROCK,
            "B" => Strategy::PAPER,
            "C" => Strategy::SCISSOR,
            _ => panic!("couldn't parse strategy"),
        };
        let own_strategy = match own_raw {
            "X" => Strategy::ROCK,
            "Y" => Strategy::PAPER,
            "Z" => Strategy::SCISSOR,
            _ => panic!("couldn't parse strategy"),
        };

        Duel {
            elf_strategy,
            own_strategy,
        }
    }

    fn play(&self) -> u32 {
        match &self.own_strategy {
            Strategy::SCISSOR => {
                SCISSOR_POINTS
                    + match &self.elf_strategy {
                    Strategy::SCISSOR => DRAW_GAME_POINTS,
                    Strategy::PAPER => WON_GAME_POINTS,
                    Strategy::ROCK => LOST_GAME_POINTS,
                }
            }
            Strategy::PAPER => {
                PAPER_POINTS
                    + match &self.elf_strategy {
                    Strategy::SCISSOR => LOST_GAME_POINTS,
                    Strategy::PAPER => DRAW_GAME_POINTS,
                    Strategy::ROCK => WON_GAME_POINTS,
                }
            }
            Strategy::ROCK => {
                ROCK_POINTS
                    + match &self.elf_strategy {
                    Strategy::SCISSOR => WON_GAME_POINTS,
                    Strategy::PAPER => LOST_GAME_POINTS,
                    Strategy::ROCK => DRAW_GAME_POINTS,
                }
            }
        }
    }
}

impl DuelWithOutcome {
    fn create(elf_raw: &str, own_raw: &str) -> DuelWithOutcome {
        let elf_strategy = match elf_raw {
            "A" => Strategy::ROCK,
            "B" => Strategy::PAPER,
            "C" => Strategy::SCISSOR,
            _ => panic!("couldn't parse strategy"),
        };
        let outcome = match own_raw {
            "X" => GameOutcome::LOSE,
            "Y" => GameOutcome::DRAW,
            "Z" => GameOutcome::WIN,
            _ => panic!("couldn't parse strategy"),
        };

        DuelWithOutcome {
            elf_strategy,
            outcome,
        }
    }

    fn play(&self) -> u32 {
        let own_strategy: Strategy = self.elf_strategy.counter_strategy(&self.outcome);

        let elf_strategy = match &self.elf_strategy {
            Strategy::SCISSOR => Strategy::SCISSOR,
            Strategy::PAPER => Strategy::PAPER,
            Strategy::ROCK => Strategy::ROCK,
        };

        let duel = Duel {
            elf_strategy,
            own_strategy,
        };
        duel.play()
    }
}

#[aoc_generator(day2, part1)]
pub fn parse_input(input: &str) -> Vec<Duel> {
    input
        .split("\n")
        .map(|line| {
            let mut duel_raw = line.split_whitespace();
            let elf_raw = duel_raw.next().expect("now elf raw");
            let own_raw = duel_raw.next().expect("no own raw");
            Duel::create(&elf_raw, &own_raw)
        })
        .collect()
}

#[aoc_generator(day2, part2)]
pub fn parse_input_2(input: &str) -> Vec<DuelWithOutcome> {
    input
        .split("\n")
        .map(|line| {
            let mut duel_raw = line.split_whitespace();
            let elf_raw = duel_raw.next().expect("now elf raw");
            let own_raw = duel_raw.next().expect("no own raw");
            DuelWithOutcome::create(&elf_raw, &own_raw)
        })
        .collect()
}

#[aoc(day2, part1)]
fn part1(duels: &[Duel]) -> u32 {
    duels.iter().map(|duel| duel.play()).sum()
}

#[aoc(day2, part2)]
fn part2(duels: &[DuelWithOutcome]) -> u32 {
    duels.iter().map(|duel| duel.play()).sum()
}

fn _unsued() {
    let path = "./input.txt";
    let input = File::open(path).expect("couldn't open file");
    let buffered = BufReader::new(input);

    let mut duels: Vec<Duel> = Vec::new();
    let mut duels_with_outcome: Vec<DuelWithOutcome> = Vec::new();

    for line in buffered.lines() {
        if let Ok(ip) = line {
            let mut duel_raw = ip.split_whitespace();
            let elf_raw = duel_raw.next().expect("now elf raw");
            let own_raw = duel_raw.next().expect("no own raw");
            duels.push(Duel::create(&elf_raw, &own_raw));
            duels_with_outcome.push(DuelWithOutcome::create(&elf_raw, &own_raw));
        }
    }
    let mut result: u32 = 0;
    for duel in duels {
        result += duel.play()
    }
    println!("Result for ay 02/01: {result}");

    let mut result: u32 = 0;
    for duel in duels_with_outcome {
        result += duel.play()
    }
    println!("Result for ay 02/02: {result}");
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_duel_1() {
        let duel = Duel::create("A", "Y");
        assert_eq!(duel.play(), 8);
    }

    #[test]
    fn test_duel_2() {
        let duel = Duel::create("B", "X");
        assert_eq!(duel.play(), 1);
    }

    #[test]
    fn test_duel_3() {
        let duel = Duel::create("C", "Z");
        assert_eq!(duel.play(), 6);
    }
}
