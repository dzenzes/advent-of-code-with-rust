use crate::GameOutcome::{DRAW, LOSE, WIN};
use crate::Strategy::{PAPER, ROCK, SCISSOR};
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
            LOSE => match &self {
                SCISSOR => PAPER,
                PAPER => ROCK,
                ROCK => SCISSOR,
            },

            DRAW => match &self {
                SCISSOR => SCISSOR,
                PAPER => PAPER,
                ROCK => ROCK,
            },

            WIN => match &self {
                SCISSOR => ROCK,
                PAPER => SCISSOR,
                ROCK => PAPER,
            },
        }
    }
}

enum GameOutcome {
    WIN,
    LOSE,
    DRAW,
}

struct Duel {
    elf_strategy: Strategy,
    own_strategy: Strategy,
}

struct DuelWithOutcome {
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
            "A" => ROCK,
            "B" => PAPER,
            "C" => SCISSOR,
            _ => panic!("couldn't parse strategy"),
        };
        let own_strategy = match own_raw {
            "X" => ROCK,
            "Y" => PAPER,
            "Z" => SCISSOR,
            _ => panic!("couldn't parse strategy"),
        };

        Duel {
            elf_strategy,
            own_strategy,
        }
    }

    fn play(&self) -> u32 {
        match &self.own_strategy {
            SCISSOR => {
                SCISSOR_POINTS
                    + match &self.elf_strategy {
                        SCISSOR => DRAW_GAME_POINTS,
                        PAPER => WON_GAME_POINTS,
                        ROCK => LOST_GAME_POINTS,
                    }
            }
            PAPER => {
                PAPER_POINTS
                    + match &self.elf_strategy {
                        SCISSOR => LOST_GAME_POINTS,
                        PAPER => DRAW_GAME_POINTS,
                        ROCK => WON_GAME_POINTS,
                    }
            }
            ROCK => {
                ROCK_POINTS
                    + match &self.elf_strategy {
                        SCISSOR => WON_GAME_POINTS,
                        PAPER => LOST_GAME_POINTS,
                        ROCK => DRAW_GAME_POINTS,
                    }
            }
        }
    }
}

impl DuelWithOutcome {
    fn create(elf_raw: &str, own_raw: &str) -> DuelWithOutcome {
        let elf_strategy = match elf_raw {
            "A" => ROCK,
            "B" => PAPER,
            "C" => SCISSOR,
            _ => panic!("couldn't parse strategy"),
        };
        let outcome = match own_raw {
            "X" => LOSE,
            "Y" => DRAW,
            "Z" => WIN,
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
            SCISSOR => SCISSOR,
            PAPER => PAPER,
            ROCK => ROCK,
        };

        let duel = Duel {
            elf_strategy,
            own_strategy,
        };
        duel.play()
    }
}

fn main() {
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
