use crate::Element;

#[derive(Debug)]
pub struct Edge<K>
where
    K: Copy,
{
    id: usize,
    u: K,
    v: K,
    pub elements: Vec<Element>,
    pub current: Option<f32>,
}

impl<K> Edge<K>
where
    K: Copy,
{
    pub fn new(u: K, v: K) -> Self {
        Self {
            id: 0,
            u,
            v,
            elements: Vec::new(),
            current: None,
        }
    }

    pub fn keys(&self) -> (K, K) {
        (self.u, self.v)
    }

    pub fn start_key(&self) -> K {
        self.u
    }

    pub fn end_key(&self) -> K {
        self.v
    }

    pub fn get_id(&self) -> usize {
        self.id
    }

    pub fn set_id(&mut self, edge_id: usize) {
        self.id = edge_id;
    }
}
