use std::collections::VecDeque;
use std::str::SplitAsciiWhitespace;

#[derive(Clone, Debug)]
enum Operand {
    NUMBER(u64),
    OLD,
}

impl Operand {
    fn from(input: &str) -> Operand {
        if let Ok(worry_level) = input.parse::<u64>() {
            Operand::NUMBER(worry_level)
        } else {
            Operand::OLD
        }
    }
}

#[derive(Clone, Debug)]
enum Operator {
    ADD,
    MULTIPLY,
}

impl Operator {
    fn from(input: &str) -> Operator {
        match input {
            "+" => Operator::ADD,
            "*" => Operator::MULTIPLY,
            _ => unreachable!(),
        }
    }
}

#[derive(Clone, Debug)]
struct Operation {
    operand_left: Operand,
    operator: Operator,
    operand_right: Operand,
}

impl Operation {
    fn from(mut input: SplitAsciiWhitespace) -> Operation {
        let operand_left = Operand::from(input.next().unwrap());
        let operator = Operator::from(input.next().unwrap());
        let operand_right = Operand::from(input.next().unwrap());

        Operation {
            operand_left,
            operator,
            operand_right,
        }
    }

    fn execute(&self, old: u64) -> u64 {
        let lhs = match self.operand_left {
            Operand::NUMBER(operand) => operand,
            Operand::OLD => old,
        };

        let rhs = match self.operand_right {
            Operand::NUMBER(operand) => operand,
            Operand::OLD => old,
        };

        match self.operator {
            Operator::ADD => lhs + rhs,
            Operator::MULTIPLY => lhs * rhs,
        }
    }
}

#[derive(Clone, Debug)]
struct Test {
    divisible_by: u64,
    if_true: usize,
    if_false: usize,
}

#[derive(Clone, Debug)]
struct Monkey {
    starting_items: VecDeque<u64>,
    operation: Operation,
    test: Test,
}

impl Monkey {
    fn from(monkey_definition: Vec<&str>) -> Monkey {
        let starting_items: VecDeque<u64> = monkey_definition
            .get(0)
            .unwrap()
            .split_once("  Starting items: ")
            .unwrap()
            .1
            .split(", ")
            .map(|token| token.parse().unwrap())
            .collect();

        let operation_tokens = monkey_definition
            .get(1)
            .unwrap()
            .split_once("  Operation: new = ")
            .unwrap()
            .1
            .split_ascii_whitespace();

        let operation: Operation = Operation::from(operation_tokens);

        let divisible_by = monkey_definition
            .get(2)
            .unwrap()
            .split_once("  Test: divisible by ")
            .unwrap()
            .1
            .parse()
            .unwrap();
        let if_true = monkey_definition
            .get(3)
            .unwrap()
            .split_once("    If true: throw to monkey ")
            .unwrap()
            .1
            .parse()
            .unwrap();
        let if_false = monkey_definition
            .get(4)
            .unwrap()
            .split_once("    If false: throw to monkey ")
            .unwrap()
            .1
            .parse()
            .unwrap();

        let test: Test = Test {
            divisible_by,
            if_true,
            if_false,
        };

        Monkey {
            starting_items,
            operation,
            test,
        }
    }
}

fn monkey_business(monkeys: &[Monkey], worry_relief: bool, number_of_rounds: usize) -> usize {
    let mut inspected_items: Vec<usize> = vec![0; monkeys.len()];
    let mut monkeys = monkeys.to_vec();

    let common_multiple: u64 = monkeys
        .iter()
        .map(|monkey| monkey.test.divisible_by)
        .product();

    for _round in 0..number_of_rounds {
        for id in 0..monkeys.len() {
            let monkey = &mut monkeys[id];
            inspected_items[id] += monkey.starting_items.len();

            let if_true_id = monkey.test.if_true;
            let mut if_true_items = VecDeque::with_capacity(monkey.starting_items.len());

            let if_false_id = monkey.test.if_false;
            let mut if_false_items = VecDeque::with_capacity(monkey.starting_items.len());

            while !monkey.starting_items.is_empty() {
                let mut worry_level = monkey.starting_items.pop_front().unwrap();
                worry_level = monkey.operation.execute(worry_level);

                if worry_relief {
                    worry_level /= 3;
                } else {
                    worry_level %= common_multiple;
                }

                if worry_level % monkey.test.divisible_by == 0 {
                    if_true_items.push_back(worry_level);
                } else {
                    if_false_items.push_back(worry_level);
                }
            }

            monkeys[if_true_id]
                .starting_items
                .append(&mut if_true_items);
            monkeys[if_false_id]
                .starting_items
                .append(&mut if_false_items);
        }
    }

    inspected_items.sort_unstable();
    inspected_items.iter().rev().take(2).product()
}

#[aoc_generator(day11)]
fn parse_input(input: &str) -> Vec<Monkey> {
    input
        .split("\n\n")
        .map(|monkey| {
            // no need for the first line
            let monkey_definition = monkey.lines().skip(1).collect::<Vec<&str>>();
            Monkey::from(monkey_definition)
        })
        .collect()
}

#[aoc(day11, part1)]
fn part1(monkeys: &Vec<Monkey>) -> usize {
    monkey_business(monkeys, true, 20)
}

#[aoc(day11, part2)]
fn part2(monkeys: &Vec<Monkey>) -> usize {
    monkey_business(monkeys, false, 10000)
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = r"Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse_input(TEST_INPUT)), 10605);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse_input(TEST_INPUT)), 2713310158);
    }
}
