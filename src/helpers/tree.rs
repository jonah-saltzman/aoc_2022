use id_arena::{Arena, Id};
use std::{collections::HashSet, fmt::Debug};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum TreeError {
    #[error("this tree already has a root but no parent was specified")]
    ExistingRoot,
    #[error("given parent node does not exist")]
    NoParent,
    #[error("specified node does not exist")]
    NotFound,
    #[error("cannot delete root node with more than 1 child")]
    DeleteRootWithChildren,
}

type TreeNodeIdInternal<T> = Id<TreeNode<T>>;

pub struct TreeNodeId<T>(TreeNodeIdInternal<T>);

impl<T> Clone for TreeNodeId<T> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}

impl<T> PartialEq for TreeNodeId<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<T> Debug for TreeNodeId<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Id").field("idx", &self.0).finish()
    }
}

impl<T> Copy for TreeNodeId<T> {}

trait TreeNodeIdBehavior<T> {
    fn id(&self) -> TreeNodeIdInternal<T>;
}

impl<T> TreeNodeIdBehavior<T> for TreeNodeId<T> {
    fn id(&self) -> TreeNodeIdInternal<T> {
        self.0
    }
}

struct TreeNode<T> {
    parent: Option<TreeNodeIdInternal<T>>,
    children: HashSet<TreeNodeIdInternal<T>>,
    value: T,
}

#[derive(Default)]
pub struct Tree<T> {
    arena: Arena<TreeNode<T>>,
    root: Option<TreeNodeIdInternal<T>>,
}

impl<T> Tree<T> {
    pub fn new() -> Self {
        Tree {
            arena: Arena::new(),
            root: None,
        }
    }

    // pub fn set_root(&mut self, val: T) -> Option<T> {
    //     if let Some(id) = self.root {
    //         Some(std::mem::replace(&mut self.arena.get_mut(id).unwrap().value, val))
    //     } else {
    //         let new_root = self.arena.
    //         None
    //     }
    // }

    pub fn add(
        &mut self,
        val: T,
        parent_id: Option<TreeNodeId<T>>,
    ) -> Result<TreeNodeId<T>, TreeError> {
        match (parent_id, self.root) {
            (Some(pid), Some(_)) => {
                let new_node = self.arena.alloc(TreeNode {
                    parent: Some(pid.id()),
                    children: HashSet::new(),
                    value: val,
                });
                let parent = self.arena.get_mut(pid.id()).ok_or(TreeError::NoParent)?;
                parent.children.insert(new_node);
                Ok(TreeNodeId(new_node))
            }
            (Some(_), None) => panic!("invalid state: parent without root"),
            (None, None) => {
                let new_root = self.arena.alloc(TreeNode {
                    parent: None,
                    children: HashSet::new(),
                    value: val,
                });
                self.root = Some(new_root);
                Ok(TreeNodeId(new_root))
            }
            (None, Some(_)) => Err(TreeError::ExistingRoot),
        }
    }
    pub fn delete(&mut self, node_id: TreeNodeId<T>) -> impl Iterator<Item = TreeNodeId<T>> {
        let parent = self
            .arena
            .get(node_id.id())
            .expect("invariant failed: node exists but not found")
            .parent;
        let root = self
            .root
            .expect("invariant failed: node exists without root");
        if let Some(pid) = parent {
            let parent = self
                .arena
                .get_mut(pid)
                .expect("invariant failed: parent does not exist");
            if parent.children.remove(&node_id.id()) == false {
                panic!("invariant failed: parent did not point to child")
            }
        } else {
            assert_eq!(
                root,
                node_id.id(),
                "invariant failed: node without parent must be root"
            );
            self.root = None;
        }
        let children = std::mem::take(
            &mut self
                .arena
                .get_mut(node_id.id())
                .expect("already checked above")
                .children,
        );
        children.into_iter().map(|e| TreeNodeId(e))
    }
    pub fn get(&self, node_id: TreeNodeId<T>) -> Option<&T> {
        self.arena.get(node_id.id()).map(|v| &v.value)
    }
    pub fn get_mut(&mut self, node_id: TreeNodeId<T>) -> Option<&mut T> {
        self.arena.get_mut(node_id.id()).map(|v| &mut v.value)
    }
    pub fn children(
        &self,
        node_id: TreeNodeId<T>,
    ) -> Option<impl Iterator<Item = TreeNodeId<T>> + '_> {
        self.arena
            .get(node_id.id())
            .map(|node| node.children.iter().map(|e| TreeNodeId(*e)))
    }
    pub fn parent(&self, node_id: TreeNodeId<T>) -> Option<TreeNodeId<T>> {
        self.arena
            .get(node_id.id())
            .map(|p| p.parent.map(|id| TreeNodeId(id)))
            .flatten()
    }

    // pub fn descendants(&self, node_id: TreeNodeId<T>) -> Option<impl Iterator<Item = TreeNodeId<T>> + '_> {
    //     self.arena.get(node_id.id())
    //         .map(|_| std::iter::successors(None, move |stack| {
    //             if let Some(id) = stack.last() {

    //                 Some(stack.pop().unwrap())
    //             } else {
    //                 None
    //             }
    //         }).flat_map(|stack| stack.into_iter()))
    // }
}
