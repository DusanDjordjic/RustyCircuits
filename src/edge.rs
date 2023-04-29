use crate::node::Node;

pub struct Edge<K = u32> {
    u: Node<K>,
    v: Node<K>,
}

impl<K> Edge<K> {
    pub fn new(u: Node<K>, v: Node<K>) -> Self {
        Self { u, v }
    }

    pub fn keys(&self) -> (K, K)
    where
        K: Copy,
    {
        (self.u.key, self.v.key)
    }

    pub fn start_key(&self) -> K
    where
        K: Copy,
    {
        self.u.key
    }

    pub fn end_key(&self) -> K
    where
        K: Copy,
    {
        self.v.key
    }
}
