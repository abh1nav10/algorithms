//! A directed-weighted graph!

use std::collections::{HashMap, HashSet, VecDeque};
use std::hash::Hash;
use std::rc::Rc;

struct Node<T, W> {
    value: Rc<T>,
    edges: Vec<(usize, W)>,
}

impl<T, W: Ord> Node<T, W> {
    fn new(value: Rc<T>, edges: Vec<(usize, W)>) -> Self {
        Self { value, edges }
    }
}

pub struct Graph<T, W> {
    list: HashMap<Rc<T>, usize>,
    nodes: Vec<Node<T, W>>,
}

impl<T, W> Default for Graph<T, W>
where
    T: Hash + Eq,
    W: Ord,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<T, W> Graph<T, W>
where
    T: Hash + Eq,
    W: Ord,
{
    pub fn new() -> Self {
        Self {
            list: HashMap::new(),
            nodes: Vec::new(),
        }
    }

    pub fn insert_node(&mut self, value: Rc<T>, edges: Vec<(Rc<T>, W)>) {
        let mut set = HashSet::new();
        let mut edge = Vec::with_capacity(edges.len());
        for element in edges {
            if let Some(index) = self.list.get(&element.0)
                && set.insert(*index)
            {
                edge.push((*index, element.1));
            }
        }
        if let Some(element) = self.list.get(&value) {
            self.nodes[*element].edges = edge;
        } else {
            let node = Node::new(value, edge);
            self.nodes.push(node);
            let index = self.nodes.len() - 1;
            let cloned = Rc::clone(&self.nodes[index].value);
            self.list.insert(cloned, index);
        }
    }

    pub fn connect(&mut self, connect: Rc<T>, to: Rc<T>, weight: W) -> Result<(), &str> {
        if let Some(index) = self.list.get(&connect) {
            if let Some(i) = self.list.get(&to) {
                let edge = (*i, weight);
                if !self.nodes[*index].edges.iter().any(|(first, _)| first == i) {
                    self.nodes[*index].edges.push(edge);
                    Ok(())
                } else {
                    Err("The connection already exists!")
                }
            } else {
                Err("The node being connected to is not a part of the graph!")
            }
        } else {
            Err("The connecting node is not a part of the graph!")
        }
    }

    pub fn dfs(&self, source: Rc<T>, find: Rc<T>) -> Result<usize, &str> {
        if !self.list.contains_key(&find) {
            return Err("The destination node is not in the graph!");
        }
        let root = if let Some(index) = self.list.get(&source) {
            *index
        } else {
            return Err("Source node is not present in the graph!");
        };
        let len = self.nodes.len();
        let mut visited = HashSet::with_capacity(len);
        let mut stack = Vec::with_capacity(len);
        let mut count = 0;
        stack.push(root);
        while let Some(element) = stack.pop() {
            if !visited.insert(element) {
                continue;
            }
            count += 1;
            if self.nodes[element].value == find {
                return Ok(count);
            } else {
                for element in self.nodes[element].edges.iter() {
                    stack.push(element.0);
                }
            }
        }
        Err("No path connects the source node to the destination node!")
    }

    pub fn bfs(&self, source: Rc<T>, find: Rc<T>) -> Result<usize, &str> {
        if !self.list.contains_key(&find) {
            return Err("The destination node is not in the graph!");
        }
        let root = if let Some(index) = self.list.get(&source) {
            *index
        } else {
            return Err("Source node is not present in the graph!");
        };
        let len = self.nodes.len();
        let mut visited = HashSet::with_capacity(len);
        let mut queue = VecDeque::with_capacity(len);
        let mut count = 0;
        queue.push_back(root);
        while let Some(element) = queue.pop_front() {
            if !visited.insert(element) {
                continue;
            }
            count += 1;
            if self.nodes[element].value == find {
                return Ok(count);
            } else {
                for element in self.nodes[element].edges.iter() {
                    queue.push_back(element.0);
                }
            }
        }
        Err("No path connects the source node to the destination node!")
    }

    // Naive but working attempt!
    //
    // pub fn has_cycle(&self) -> bool {
    //     let mut map = HashMap::new();
    //     for (key, value) in self.list.iter() {
    //         let key = Rc::clone(key);
    //         map.entry(key).or_insert(0);
    //         for (index, _) in self.nodes[*value].edges.iter() {
    //             let element = Rc::clone(&self.nodes[*index].value);
    //             map.entry(element).and_modify(|e| *e += 1).or_insert(1);
    //         }
    //     }
    //     let mut elements = Vec::with_capacity(self.list.len());
    //     for (key, value) in map.iter() {
    //         if *value == 0 {
    //             let key = Rc::clone(key);
    //             elements.push(key);
    //         }
    //     }
    //     let mut count = 0;
    //     while let Some(element) = elements.pop() {
    //         count += 1;
    //         let node = self
    //             .list
    //             .get(&element)
    //             .expect("It has already been inserted into the map!");
    //         for (i, _) in self.nodes[*node].edges.iter() {
    //             let rc = Rc::clone(&self.nodes[*i].value);
    //             let index = map.get_mut(&rc).expect("Has to be there");
    //             *index -= 1;
    //             if *index == 0 {
    //                 elements.push(rc);
    //             }
    //         }
    //     }
    //     count != self.list.len()
    // }

    /// Implementation of Kahn's algorithm.
    pub fn has_cycle(&self) -> bool {
        let len = self.nodes.len();
        let mut vector = vec![0; len];
        for element in self.nodes.iter() {
            for (e, _) in element.edges.iter() {
                vector[*e] += 1;
            }
        }
        let mut zero = Vec::with_capacity(len);
        for (index, element) in vector.iter().enumerate() {
            if *element == 0 {
                zero.push(index);
            }
        }
        let mut count = 0;
        while let Some(element) = zero.pop() {
            count += 1;
            for (i, _) in self.nodes[element].edges.iter() {
                let i = *i;
                vector[i] -= 1;
                if vector[i] == 0 {
                    zero.push(i);
                }
            }
        }
        count != len
    }

    pub fn remove_node(&mut self, node: Rc<T>) -> Result<(), &str> {
        let len = self.nodes.len() - 1;
        if let Some(element) = self.list.remove(&node) {
            for node in self.nodes.iter_mut() {
                for (index, (e, _)) in node.edges.iter().enumerate() {
                    if *e == element {
                        node.edges.swap_remove(index);
                        // Break out of the inner loop because it cannot have duplicate edges!
                        break;
                    }
                }
            }
            // If element equals to length, that is an edge case where out fixup is complete!
            if element != len {
                let key = &self.nodes[len].value;
                *self.list.get_mut(key).expect("Has to be there") = element;
                for node in self.nodes.iter_mut() {
                    for (e, _) in node.edges.iter_mut() {
                        if *e == len {
                            *e = element;
                            // Break out of the inner loop because it cannot have duplicate edges!
                            break;
                        }
                    }
                }
            }
            self.nodes.swap_remove(element);
            Ok(())
        } else {
            Err("The node does not exist in the graph!")
        }
    }

    pub fn dijkstra_shortest_path(&self, _source: Rc<T>, _dest: Rc<T>) {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
