//! A directed-weighted graph!

use std::collections::{HashMap, HashSet, VecDeque};
use std::hash::Hash;
use std::rc::Rc;

struct Node<T, W> {
    value: Rc<T>,
    edges: Vec<(usize, W)>,
}

impl<T, W: Ord> Node<T, W> {
    fn new(value: T, edges: Vec<(usize, W)>) -> Self {
        Self {
            value: Rc::new(value),
            edges,
        }
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

    pub fn insert_node(&mut self, value: T, edges: Vec<(Rc<T>, W)>) {
        let mut edge = Vec::with_capacity(edges.len());
        for element in edges {
            if let Some(index) = self.list.get(&element.0) {
                edge.push((*index, element.1));
            }
        }
        let node = Node::new(value, edge);
        self.nodes.push(node);
        let index = self.nodes.len() - 1;
        let cloned = Rc::clone(&self.nodes[index].value);
        self.list.insert(cloned, index);
    }

    pub fn connect(&mut self, connect: Rc<T>, to: Rc<T>, weight: W) -> Result<(), &str> {
        if let Some(index) = self.list.get(&connect) {
            if let Some(i) = self.list.get(&to) {
                self.nodes[*index].edges.push((*i, weight));
                Ok(())
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

    pub fn has_cycle(&self) -> bool {
        todo!()
    }

    pub fn remove_node(&self, node: Rc<T>) -> Result<(), &str> {
        todo!()
    }

    pub fn dijkstra_shortest_path(&self, source: Rc<T>, dest: Rc<T>) {
        todo!()
    }
}
