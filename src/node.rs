pub struct Node<K = u32> {
    pub key: K,
}

impl<K> Node<K> {
    pub fn new(key: K) -> Self {
        Self { key }
    }
}
