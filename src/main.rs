#[derive(Debug, PartialEq)]
pub struct TrieNode<K, V>
where
    K: Eq + std::hash::Hash + Clone,
    V: Clone,
{
    value: Option<V>,
    children: std::collections::HashMap<K, TrieNode<K, V>>,
}

impl<K, V> TrieNode<K, V>
where
    K: std::cmp::Eq + std::hash::Hash + Clone,
    V: Clone,
{
    fn new() -> TrieNode<K, V> {
        TrieNode {
            value: None,
            children: std::collections::HashMap::new(),
        }
    }

    fn insert(&mut self, path: K, v: V) -> Option<TrieNode<K, V>> {
        self.children.insert(
            path,
            TrieNode {
                value: Some(v),
                children: std::collections::HashMap::new(),
            },
        )
    }

    fn contains(&self, key: K) -> bool {
        self.children.contains_key(&key)
    }
}

fn main() {
    let mut trie = TrieNode::new();
    trie.insert("hey", ["it works"]);
    println!("{:#?}", trie);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trie_contains_test() {
        let mut trie = TrieNode::new();
        trie.insert("hey", ["it works"]);

        assert_eq!(trie.contains("hey"), true);
    }
}
