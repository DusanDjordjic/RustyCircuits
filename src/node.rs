#[derive(Debug)]
pub struct Node<K> {
    pub key: K,
}

impl<K> Node<K> {
    pub fn new(key: K) -> Self {
        Self { key }
    }
}
