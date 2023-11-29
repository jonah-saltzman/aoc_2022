use crate::parser::{CdTarget, Command, Group, LsLine, LsOutput};
use aoc_2022::{arena::NodeId, tree::Tree};

#[derive(Debug)]
struct Directory {
    name: String,
    size_direct: usize,
    size_indirect: Option<usize>,
}

impl Directory {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_owned(),
            size_direct: 0,
            size_indirect: None,
        }
    }
}

pub struct Calculator {
    tree: Tree<Directory>,
    current_node: Option<NodeId>,
}

impl Calculator {
    pub fn new() -> Self {
        Self {
            tree: Tree::new(),
            current_node: None,
        }
    }

    pub fn handle_group(&mut self, group: Group) {
        match group {
            Group::Input(Command::Ls) => assert!(self.current_node.is_some()),
            Group::Input(Command::Cd(dir)) => self.change_dir(dir),
            Group::Output(out) => self.handle_output(out),
        }
    }

    fn change_dir(&mut self, dir: CdTarget) {
        match dir {
            CdTarget::Root => {
                if self.current_node.is_none() {
                    let root = Directory::new("/");
                    let root = self.tree.add_node(root, None).unwrap();
                    self.current_node = Some(root);
                } else {
                    self.current_node = Some(self.tree.root().unwrap());
                }
            }
            CdTarget::Parent => {
                let parent = self.tree.parent(self.current_node.unwrap()).unwrap();
                self.current_node = Some(parent);
            }
            CdTarget::Named(target) => {
                let current = self.current_node.unwrap();
                let (child_id, _) = self
                    .tree
                    .children(current)
                    .find(|&(_, dir)| dir.name == target)
                    .unwrap();
                self.current_node = Some(child_id);
            }
        }
    }

    fn handle_output(&mut self, list: LsOutput) {
        for element in list.into_iter() {
            match element {
                LsLine::File(file) => {
                    let current = self.tree.get_mut(self.current_node.unwrap());
                    current.size_direct += file.size;
                }
                LsLine::Folder(folder) => {
                    let current = self.current_node.unwrap();
                    let new_node = Directory::new(&folder.name);
                    self.tree.add_node(new_node, Some(current)).unwrap();
                }
            }
        }
    }

    fn node_indirect(&mut self, node_id: NodeId) -> usize {
        if let Some(size) = self.tree.get(node_id).size_indirect {
            return size;
        }
        let children: Vec<NodeId> = self.tree.children_ids(node_id).copied().collect();
        let mut indirect: usize = 0;
        for child_id in children {
            let child = self.tree.get(child_id);
            let child_direct = child.size_direct;
            let initial_child_indirect = child.size_indirect;
            let child_indirect =
                initial_child_indirect.unwrap_or_else(|| self.node_indirect(child_id));
            indirect += child_direct + child_indirect;
        }
        self.tree.get_mut(node_id).size_indirect = Some(indirect);
        indirect
    }

    pub fn get_result(mut self) -> usize {
        let root = self.tree.root().unwrap();
        self.node_indirect(root);
        self.tree
            .into_iter()
            .filter_map(|node| {
                let total = node.size_direct + node.size_indirect.unwrap();
                if total <= 100000 {
                    Some(total)
                } else {
                    None
                }
            })
            .sum()
    }
}
