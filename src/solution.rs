use aoc_2022::{tree::Tree, arena::NodeId};
use crate::parser::{LsFile, LsFolder, Group, Command, LsOutput, CdTarget};

#[derive(Debug)]
struct Directory {
    name: String,
    size: usize
}

impl Directory {
    fn new(name: &str) -> Self {
        Self { name: name.to_owned(), size: 0 }
    }
}

pub struct Calculator {
    tree: Tree<Directory>,
    current_node: Option<NodeId>,
}

impl Calculator {
    pub fn new() -> Self {
        Self { tree: Tree::new(), current_node: None }
    }

    pub fn handle_group(&mut self, group: Group) {
        match group {
            Group::Input(Command::Ls) => assert!(self.current_node.is_some()),
            Group::Input(Command::Cd(dir)) => self.change_dir(dir),
            Group::Output(out) => self.handle_output(out)
        }
    }

    fn change_dir(&mut self, dir: CdTarget) {
        println!("cd {:?}", dir);
        match dir {
            CdTarget::Root => {
                if self.current_node.is_none() {
                    let root = Directory::new("/");
                    let root = self.tree.add_node(root, None).unwrap();
                    self.current_node = Some(root);
                } else {
                    self.current_node = Some(self.tree.root().unwrap());
                }
            },
            CdTarget::Parent => {
                let parent = self.tree.parent(self.current_node.unwrap()).unwrap();
                self.current_node = Some(parent);
            },
            CdTarget::Named(target) => {
                let current = self.current_node.unwrap();
                let child = self.tree.children(current)
                    .find(|&(_, dir)| dir.name == target);
                match child {
                    Some((id, _)) => {
                        self.current_node = Some(id);
                    },
                    None => {
                        let new_child = Directory::new(&target);
                        let new_child = self.tree.add_node(new_child, Some(self.current_node.unwrap())).unwrap();
                        self.current_node = Some(new_child);
                    }
                }
            }
        }
        let new_current = self.current();
        println!("now in {}", new_current.name);
    }

    fn handle_output(&mut self, list: LsOutput) {
        let current = self.current();
        println!("in {}:\n{:?}", current.name, list);
    }

    fn current(&self) -> &Directory {
        let current = self.current_node.unwrap();
        self.tree.get(current)
    }

    pub fn get_result(&self) -> usize {
        1
    }
}
