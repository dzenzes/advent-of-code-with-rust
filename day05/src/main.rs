type Stacks = Vec<Vec<char>>;

#[derive(Debug)]
struct Instruction {
    from: usize,
    to: usize,
    size: u32,
}

#[derive(Debug)]
struct InstructionError;

pub fn parse_stacks(input: &str) -> Stacks {
    let mut stacks: Stacks = Vec::new();

    for line in input.lines().rev() {
        line.as_bytes()
            .chunks(4)
            .enumerate()
            .for_each(|(idx, group)| {
                if stacks.get(idx).is_none() {
                    stacks.push(Vec::new());
                }

                if group[0] == b'[' {
                    stacks[idx].push(group[1] as char);
                }
            });
    }

    stacks
}

fn parse_moves(input: &str) -> Vec<Instruction> {
    let mut instructions = Vec::new();

    for line in input.lines() {
        let trimmed_line = line.trim();
        if trimmed_line == "" {
            continue;
        }

        let mut tokens = trimmed_line
            .split(" ")
            .filter(|val| i32::from_str_radix(val, 10).is_ok());

        let instruction = Instruction {
            size: u32::from_str_radix(tokens.next().unwrap(), 10).unwrap(),
            from: usize::from_str_radix(tokens.next().unwrap(), 10).unwrap(),
            to: usize::from_str_radix(tokens.next().unwrap(), 10).unwrap(),
        };
        instructions.push(instruction);
    }
    instructions
}

fn execute_instruction(
    stacks: &mut Stacks,
    move_instruction: &Instruction,
) -> Result<(), InstructionError> {
    for _ in 0..move_instruction.size {
        let val = stacks[move_instruction.from - 1]
            .pop()
            .ok_or(InstructionError)?;

        stacks[move_instruction.to - 1].push(val);
    }

    Ok(())
}

fn execute_instruction_crane_9001(
    stacks: &mut Stacks,
    move_instruction: &Instruction,
) -> Result<(), InstructionError> {
    let from_idx = move_instruction.from - 1;
    let stack_from_len = stacks[from_idx].len();
    let elems = stacks[from_idx]
        .drain((stack_from_len - move_instruction.size as usize)..stack_from_len)
        .collect::<Vec<_>>();
    stacks[move_instruction.to - 1].extend(elems);
    Ok(())
}

fn execute_moves(stacks: &mut Stacks, instructions: &Vec<Instruction>) {
    instructions
        .iter()
        .try_for_each(|instruction| execute_instruction(stacks, instruction))
        .expect("instructions should be valid");
}

fn execute_moves_crane_9001(stacks: &mut Stacks, instructions: &Vec<Instruction>) {
    instructions
        .iter()
        .try_for_each(|instruction| execute_instruction_crane_9001(stacks, instruction))
        .expect("instructions should be valid");
}

fn top_crates(stacks: &Stacks) -> String {
    let mut top_crates = String::new();

    for stack in stacks {
        let top_char = stack.get(stack.len() - 1);

        if let Some(top_char) = top_char {
            top_crates.push(*top_char);
        }
    }
    top_crates
}

fn day_5_part_1(input_stacks: &str, input_moves: &str) -> String {
    let mut stacks = parse_stacks(input_stacks);
    let moves = parse_moves(input_moves);
    execute_moves(&mut stacks, &moves);

    top_crates(&stacks)
}

fn day_5_part_2(input_stacks: &'static str, input_moves: &'static str) -> String {
    let mut stacks = parse_stacks(input_stacks);
    let moves = parse_moves(input_moves);
    execute_moves_crane_9001(&mut stacks, &moves);

    top_crates(&stacks)
}

fn main() {
    let input = include_str!("../input.txt").split_once("\n\n").unwrap();
    let result = day_5_part_1(input.0, input.1);
    println!("Result for day 05/01: {result}");
    let result = day_5_part_2(input.0, input.1);
    println!("Result for day 05/02: {result}");
}

#[cfg(test)]
mod test {
    use super::*;

    static TEST_INPUT: &str = r#"
    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 
    "#;

    static TEST_INPUT_MOVES: &str = r#"
    move 1 from 2 to 1
    move 3 from 1 to 3
    move 2 from 2 to 1
    move 1 from 1 to 2
    "#;

    #[test]
    fn test_parse_moves() {
        let moves = parse_moves(TEST_INPUT_MOVES);

        // Index values should be 1 less than text values
        assert_eq!(moves[0].from, 2);
        assert_eq!(moves[0].to, 1);
        assert_eq!(moves[0].size, 1);

        assert_eq!(moves[1].from, 1);
        assert_eq!(moves[1].to, 3);
        assert_eq!(moves[1].size, 3);
        assert_eq!(moves.len(), 4);
    }

    #[test]
    fn test_parse_stacks() {
        let stacks = parse_stacks(TEST_INPUT);

        assert_eq!(stacks[0][0], 'Z');
        assert_eq!(stacks[1][2], 'D');
        assert_eq!(stacks[2][0], 'P');

        assert_eq!(stacks.len(), 3);
    }

    #[test]
    fn test_execute_moves() {
        let mut stacks = parse_stacks(TEST_INPUT);
        let moves = parse_moves(TEST_INPUT_MOVES);

        execute_instruction(&mut stacks, &moves[0])
            .map_err(|err| println!("{:?}", err))
            .unwrap();

        assert_eq!(stacks[0][2], 'D');

        execute_instruction(&mut stacks, &moves[1])
            .map_err(|err| println!("{:?}", err))
            .unwrap();

        assert_eq!(stacks[2][3], 'Z');

        execute_instruction(&mut stacks, &moves[2])
            .map_err(|err| println!("{:?}", err))
            .unwrap();

        assert_eq!(stacks.get(1), Some(&vec![]));
    }

    #[test]
    fn part_1() {
        let mut stacks = parse_stacks(TEST_INPUT);
        let moves = parse_moves(TEST_INPUT_MOVES);
        execute_moves(&mut stacks, &moves);

        assert_eq!(stacks[1][0], 'M');

        let top_crates = top_crates(&stacks);

        assert_eq!(top_crates, "CMZ".to_string());
    }

    #[test]
    fn part_2() {
        let mut stacks = parse_stacks(TEST_INPUT);
        let moves = parse_moves(TEST_INPUT_MOVES);
        execute_moves_crane_9001(&mut stacks, &moves);

        // assert_eq!(stacks[1][0], 'M');

        let top_crates = top_crates(&stacks);

        assert_eq!(top_crates, "MCD".to_string());
    }
}
