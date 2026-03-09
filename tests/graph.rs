use algorithms::Graph;
use std::rc::Rc;

#[test]
fn check_cycle() {
    let mut graph = Graph::new();
    let n1 = Rc::new(1);
    let n2 = Rc::new(2);
    let n3 = Rc::new(3);
    graph.insert_node(Rc::clone(&n3), vec![]);
    graph.insert_node(Rc::clone(&n2), vec![(Rc::clone(&n3), 0)]);
    graph.insert_node(Rc::clone(&n1), vec![(Rc::clone(&n2), 0)]);

    assert!(!graph.has_cycle());

    let _ = graph.connect(Rc::clone(&n3), Rc::clone(&n1), 0);

    assert!(graph.has_cycle());
}

#[test]
fn node_removal() {
    let mut graph = Graph::new();
    let n1 = Rc::new(1);
    let n2 = Rc::new(2);
    let n3 = Rc::new(3);
    graph.insert_node(Rc::clone(&n3), vec![]);
    graph.insert_node(Rc::clone(&n2), vec![(Rc::clone(&n3), 0)]);
    graph.insert_node(Rc::clone(&n1), vec![(Rc::clone(&n2), 0)]);
    let _ = graph.connect(Rc::clone(&n3), Rc::clone(&n1), 0);
    assert!(graph.has_cycle());

    let res = graph.remove_node(Rc::clone(&n3));

    assert!(res.is_ok());
    assert!(!graph.has_cycle());
}
