use algorithms::Tree;

#[test]
fn insert() {
    let mut new = Tree::new(7);
    new.insert(9);
    new.insert(6);
    new.insert(3);
    new.insert(4);
    new.insert(13);
    new.insert(97);
    assert_eq!(7, new.length());
}

#[test]
fn in_order() {
    let mut new = Tree::new(9);
    new.insert(76);
    new.insert(23);
    new.insert(56);
    new.insert(12);
    new.insert(98);
    new.insert(2);
    let mut vec = Vec::with_capacity(7);
    new.traverse_in_order(|e| vec.push(*e));
    assert!(vec.is_sorted());

    let successor = new.in_order_successor(&23);
    assert!(successor.is_some());
    assert_eq!(**successor.unwrap(), 56);

    let predecessor = new.in_order_predecessor(&98);
    assert!(predecessor.is_some());
    assert_eq!(**predecessor.unwrap(), 76);
}

#[test]
fn smallest_largest_search() {
    let mut tree = Tree::new(67);
    tree.insert(45);
    tree.insert(103);
    tree.insert(34);
    tree.insert(59);
    tree.insert(99);

    assert_eq!(tree.smallest(), Some(&34));
    assert_eq!(tree.largest(), Some(&103));

    assert!(tree.search(&59));
    assert!(!tree.search(&11));
}

#[test]
fn delete() {
    let mut tree = Tree::new(10);
    tree.insert(33);
    tree.insert(12);
    tree.insert(78);
    tree.insert(45);
    tree.insert(67);

    // no parent
    tree.delete(&12);

    tree.delete(&78);
    tree.delete(&33);
    tree.delete(&45);

    assert_eq!(tree.length(), 2);
}
