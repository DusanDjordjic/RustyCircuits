use crate::edge::Edge;
use crate::error::EcsError;
use crate::node::Node;

use std::cell::RefCell;
use std::collections::HashMap;
use std::hash::Hash;
use std::rc::{Rc, Weak};

#[derive(Debug)]
pub struct Network<K>
where
    K: Eq + Hash + Copy,
{
    nodes: HashMap<K, Node<K>>,
    edges: Vec<Rc<RefCell<Edge<K>>>>,
    edges_from_node: HashMap<K, Vec<Weak<RefCell<Edge<K>>>>>,
}

impl<K> Network<K>
where
    K: Eq + Hash + Copy,
{
    pub fn new(
        nodes: HashMap<K, Node<K>>,
        edges: Vec<Rc<RefCell<Edge<K>>>>,
        edges_from_node: HashMap<K, Vec<Weak<RefCell<Edge<K>>>>>,
    ) -> Self {
        Self {
            nodes,
            edges,
            edges_from_node,
        }
    }

    pub fn insert_node(&mut self, node: Node<K>) -> Result<(), EcsError> {
        if self.nodes.contains_key(&node.key) {
            return Err(EcsError::node_exists());
        }

        self.nodes.insert(node.key, node);
        Ok(())
    }

    pub fn insert_egde(&mut self, edge: Edge<K>) {
        let (ukey, vkey) = edge.keys();
        let rc = Rc::new(RefCell::new(edge));

        self.edges_from_node
            .entry(ukey)
            .or_insert(Vec::new())
            .push(Rc::downgrade(&rc));

        self.edges_from_node
            .entry(vkey)
            .or_insert(Vec::new())
            .push(Rc::downgrade(&rc));

        self.edges.push(rc);
    }

    pub fn get_edge_by_id(&self, edge_id: usize) -> Option<Weak<RefCell<Edge<K>>>> {
        self.edges
            .iter()
            .find(|edge| edge.borrow().get_id() == edge_id)
            .map(Rc::downgrade)
    }
}

impl<K> Default for Network<K>
where
    K: Eq + Hash + Copy,
{
    fn default() -> Self {
        Self {
            nodes: HashMap::new(),
            edges: Vec::new(),
            edges_from_node: HashMap::new(),
        }
    }
}
