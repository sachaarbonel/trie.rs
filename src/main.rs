#[derive(Debug, PartialEq, Clone)]
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
    K: std::cmp::Eq + std::hash::Hash + Clone + Copy,
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

    fn get(&self, key: K) -> Option<TrieNode<K, V>> {
        if self.contains(key) {
            Some(self.children[&key].clone())
        } else {
            None
        }
    }
    fn contains_where(&self, test: Callback<K>) -> bool {
        self.children.keys().any(|&key| test(key))
    }
}

type Callback<K> = fn(key: K) -> bool;

fn main() {
    let mut trie = TrieNode::new();
    trie.insert("hey", ["it works"]);
    println!("{:#?}", trie.get("hey"));
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
    #[test]
    fn trie_get_test() {
        let mut trie = TrieNode::new();
        trie.insert("hey", ["it works"]);
        assert_eq!(
            trie.get("hey"),
            Some(TrieNode {
                value: Some(["it works"]),
                children: std::collections::HashMap::new()
            })
        );
    }

    #[test]
    fn trie_contains_where_test() {
        let mut trie = TrieNode::new();
        trie.insert("users/:id", ["whatever"]);
        assert_eq!(trie.contains_where(|x| x.contains(":")), true);
    }
}
