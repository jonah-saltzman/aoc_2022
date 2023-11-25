use aoc_2022::tree::*;

struct Opaque(i32);

#[test]
fn children_iter() {
    let mut tree: Tree<Opaque> = Tree::new();
    let parent = tree.add(Opaque(1), None).unwrap();
    let child1 = tree.add(Opaque(2), Some(parent)).unwrap();
    let child2 = tree.add(Opaque(3), Some(parent)).unwrap();
    let child3 = tree.add(Opaque(4), Some(parent)).unwrap();
    let mut children = tree.children(parent).unwrap();
    assert_eq!(children.next().unwrap(), child1);
    assert_eq!(children.next().unwrap(), child2);
    assert_eq!(children.next().unwrap(), child3);
}