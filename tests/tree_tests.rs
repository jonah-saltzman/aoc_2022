use aoc_2022::tree::Tree;
use itertools::Itertools;

#[test]
fn mutability() {
    let mut tree: Tree<i32> = Tree::new();
    let root = tree.add_node(123, None).unwrap();
    let val = tree.get_mut(root);
    *val = 500;
    let val = tree.get_mut(tree.root().unwrap());
    assert_eq!(*val, 500);
}

#[test]
fn children() {
    let mut tree: Tree<i32> = Tree::new();
    let root = tree.add_node(1, None).unwrap();
    let a = tree.add_node(2, Some(root)).unwrap();
    let b = tree.add_node(3, Some(root)).unwrap();
    let _ = tree.add_node(4, Some(b)).unwrap();
    let _ = tree.add_node(5, Some(b)).unwrap();
    let _ = tree.add_node(6, Some(b)).unwrap();
    let mut children = vec![a, b];
    tree.children(root).for_each(|child| {
        assert!(children.iter().contains(&child.0));
        children.remove(children.iter().position(|&e| e == child.0).unwrap());
    });
    assert_eq!(children.len(), 0);
}

#[test]
fn descendants() {
    let mut tree: Tree<i32> = Tree::new();
    let root = tree.add_node(1, None).unwrap();
    let a = tree.add_node(2, Some(root)).unwrap();
    let b = tree.add_node(3, Some(root)).unwrap();
    let c = tree.add_node(4, Some(b)).unwrap();
    let d = tree.add_node(5, Some(b)).unwrap();
    let e = tree.add_node(6, Some(b)).unwrap();
    let mut descendants_of_root = vec![a, b, c, d, e];
    let mut descendants_of_b = vec![c, d, e];
    tree.descendants(root).for_each(|child| {
        assert!(descendants_of_root.iter().contains(&child.0));
        descendants_of_root.remove(descendants_of_root.iter().position(|&e| e == child.0).unwrap());
    });
    tree.descendants(b).for_each(|child| {
        assert!(descendants_of_b.iter().contains(&child.0));
        descendants_of_b.remove(descendants_of_b.iter().position(|&e| e == child.0).unwrap());
    });
    assert_eq!(descendants_of_root.len(), 0);
    assert_eq!(descendants_of_b.len(), 0);
}

#[test]
fn parent() {
    let mut tree: Tree<i32> = Tree::new();
    let root = tree.add_node(1, None).unwrap();
    let a = tree.add_node(2, Some(root)).unwrap();
    let b = tree.add_node(4, Some(a)).unwrap();
    let c = tree.add_node(5, Some(a)).unwrap();
    let b_parent = tree.parent(b).unwrap();
    let c_parent = tree.parent(c).unwrap();
    assert_eq!(a, b_parent);
    assert_eq!(b_parent, c_parent);
}

#[test]
fn ancestors() {
    let mut tree: Tree<i32> = Tree::new();
    let root = tree.add_node(1, None).unwrap();
    let a = tree.add_node(2, Some(root)).unwrap();
    let b = tree.add_node(4, Some(a)).unwrap();
    let c = tree.add_node(5, Some(b)).unwrap();
    let x = tree.add_node(2, Some(root)).unwrap();
    let y = tree.add_node(4, Some(x)).unwrap();
    let z = tree.add_node(5, Some(y)).unwrap();
    let mut ancestors_z = vec![y, x, root];
    let mut ancestors_c = vec![b, a, root];
    tree.ancestors(z).for_each(|a| {
        assert!(ancestors_z.contains(&a.0));
        ancestors_z.remove(ancestors_z.iter().position(|&e| e == a.0).unwrap());
    });
    tree.ancestors(c).for_each(|a| {
        assert!(ancestors_c.contains(&a.0));
        ancestors_c.remove(ancestors_c.iter().position(|&e| e == a.0).unwrap());
    })
}

#[test]
fn ancestors_mut() {
    let mut tree: Tree<i32> = Tree::new();
    let mut node = tree.add_node(1, None).unwrap();
    for _ in 0..4 {
        node = tree.add_node(1, Some(node)).unwrap();
    }
    tree.mutate_ancestors(node, |_, val| {
        *val = 2;
    });
    tree.ancestors(node).for_each(|(_, v)| assert_eq!(2, *v));
    let sum: i32 = tree.ancestors(node).map(|(_, v)| *v).sum();
    assert_eq!(8, sum);
}