use std::cell::RefCell;
use std::collections::HashMap;
use std::hash::Hash;
use std::rc::{Rc, Weak};

use crate::error::EcsError;

pub struct Node<K = u32> {
    key: K,
}

pub struct Edge<K = u32> {
    u: Node<K>,
    v: Node<K>,
}

pub struct Network<K = u32>
where
    K: Eq + Hash + Copy,
{
    nodes: HashMap<K, Node<K>>,
    edges: Vec<Rc<RefCell<Edge<K>>>>,
    edges_from_node: HashMap<K, Vec<Weak<RefCell<Edge<K>>>>>,
}

impl<K> Default for Network<K>
where
    K: Eq + Hash + Copy,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<K> Network<K>
where
    K: Eq + Hash + Copy,
{
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            edges: Vec::new(),
            edges_from_node: HashMap::new(),
        }
    }

    // TODO: Add error type Node already exists
    pub fn insert_node(&mut self, node: Node<K>) -> Result<(), EcsError> {
        if self.nodes.contains_key(&node.key) {
            return Err(EcsError::node_exists());
        }

        self.nodes.insert(node.key, node);
        Ok(())
    }

    pub fn insert_egde(&mut self, edge: Edge<K>) {
        let rc = Rc::new(RefCell::new(edge));

        self.edges_from_node
            .entry(rc.borrow().u.key)
            .or_insert(Vec::new())
            .push(Rc::downgrade(&rc));

        self.edges_from_node
            .entry(rc.borrow().v.key)
            .or_insert(Vec::new())
            .push(Rc::downgrade(&rc));

        self.edges.push(rc);
    }
}
