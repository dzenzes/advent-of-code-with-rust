use std::{
    collections::HashMap,
};

#[derive(Debug)]
enum Node {
    Directory(Vec<String>, HashMap<String, Node>),
    File(String, usize),
}

impl Node {
    fn size(&self) -> usize {
        match self {
            Node::Directory(_, memory) => memory.values().map(|n| n.size()).sum(),
            Node::File(_, size) => *size,
        }
    }

    fn total_at_most(&self, max: usize) -> usize {
        match self {
            Node::Directory(_, memory) => {
                let size = self.size();
                (if size <= max { size } else { 0 })
                    + memory.values().map(|n| n.total_at_most(max)).sum::<usize>()
            }
            Node::File(_, _) => 0,
        }
    }

    fn smallest_directory_of_at_least(&self, space_to_delete: usize) -> Option<usize> {
        if self.size() < space_to_delete {
            None
        } else {
            match self {
                Node::Directory(_, memory) => memory
                    .values()
                    .filter_map(|f| f.smallest_directory_of_at_least(space_to_delete))
                    .chain(std::iter::once(self.size()))
                    .min(),
                Node::File(_, _) => None,
            }
        }
    }
}

fn parse_input(input: &str) -> Node {
    let mut root = Node::Directory(vec!["/".into()], Default::default());

    let mut current_directory: &mut Node = &mut root;
    for line in input.lines() {
        if line.starts_with("$") {
            let command = &line[2..line.len()];
            // command
            match command {
                line if line.starts_with("cd") => {
                    let location = &command[2..command.len()];

                    match location.trim() {
                        "/" => {
                            current_directory = &mut root;
                        }
                        ".." => {
                            if let Node::Directory(path, _members) = current_directory {
                                let parent_path: Vec<String> =
                                    path.iter().rev().skip(1).rev().cloned().collect();
                                for loc in parent_path {
                                    if loc == "/" {
                                        current_directory = &mut root;
                                    } else {
                                        if let Node::Directory(_, members) = current_directory {
                                            current_directory = members.get_mut(&loc).unwrap();
                                        }
                                    }
                                }
                            }
                        }
                        _ => {
                            if let Node::Directory(path, members) = current_directory {
                                let new_path = path
                                    .clone()
                                    .into_iter()
                                    .chain(std::iter::once(location.to_owned()))
                                    .collect();
                                current_directory = members
                                    .entry(location.to_owned())
                                    .or_insert(Node::Directory(new_path, Default::default()));
                            }
                        }
                    }
                }
                line if line.starts_with("ls") => {
                    continue;
                }
                _ => panic!("unsupported command"),
            }
        } else {
            // file
            let (file_size, name) = line.split_once(' ').unwrap();
            if file_size == "dir" {
                continue;
            }
            let file_size: usize = file_size.parse().unwrap();
            let name = name.trim().to_owned();
            if let Node::Directory(_, members) = current_directory {
                let file_node = Node::File(name.clone(), file_size);
                members.insert(name, file_node);
            }
        }
    }
    root
}

#[aoc(day7, part1)]
fn part1(input: &str) -> usize {
    let root = parse_input(input);

    root.total_at_most(100000)
}

#[aoc(day7, part2)]
fn part2(input: &str) -> usize {
    let root = parse_input(input);

    let total_space = 70000000;
    let required_space = 30000000;
    let cur_space = root.size();
    let space_to_delete = cur_space - (total_space - required_space);

    root.smallest_directory_of_at_least(space_to_delete).unwrap()
}


