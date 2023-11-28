use aoc_2022::tree::Tree;
use crate::parser::{LsFile, LsFolder, Group, Command};

enum DirNode {
    File(LsFile),
    Folder(LsFolder)
}

pub struct Calculator {
    tree: Tree<DirNode>,
}

impl Calculator {
    pub fn new() -> Self {
        Self { tree: Tree::new() }
    }

    pub fn handle_group(&mut self, group: Group) {
        todo!()
    }

    pub fn get_result(&self) -> usize {
        1
    }
}
