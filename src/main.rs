use std::iter::FromIterator;

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

    fn add(&mut self, path: K, v: V) -> Option<TrieNode<K, V>> {
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
    fn get_where(&self, test: Callback<K>) -> Option<TrieNode<K, V>> {
        let iter = self.children.keys().find(|&&key| test(key));
        match iter {
            Some(result) => self.get(*result),
            _ => None,
        }
    }
}

type Callback<K> = fn(key: K) -> bool;

fn chunk(input: &str) -> bool {
    let chars: Vec<char> = input.chars().collect();
    let slice = chars.as_slice();
    let mut result = Vec::new();
    for i in 1..=slice.len() {
        let s = String::from_iter(&slice[0..i].to_vec());
        result.push(s);
    }
    result.contains(&String::from(input))
}
fn main() {
    let mut trie = TrieNode::new();
    trie.add("Paris", "Geo struct".to_owned());
    let result = trie.get_where(|input| chunk(input));
    println!("{:#?}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trie_contains_test() {
        let mut trie = TrieNode::new();
        trie.add("hey", ["it works"]);

        assert_eq!(trie.contains("hey"), true);
    }

    #[test]
    fn trie_chunk_test() {
        let mut trie = TrieNode::new();
        trie.add("Paris", "Geo struct".to_owned());
        assert_eq!(
            trie.get_where(|input| chunk(input)),
            Some(TrieNode {
                value: Some("Geo struct".to_owned()),
                children: std::collections::HashMap::new()
            })
        );
    }
    #[test]
    fn trie_get_test() {
        let mut trie = TrieNode::new();
        trie.add("hey", ["it works"]);
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
        trie.add("users/:id", ["whatever"]);
        assert_eq!(trie.contains_where(|x| x.contains(":")), true);
    }

    #[test]
    fn trie_get_where_test() {
        let mut trie = TrieNode::new();
        trie.add("users/:id", ["whatever"]);
        assert_eq!(
            trie.get_where(|x| x.contains(":")),
            Some(TrieNode {
                value: Some(["whatever"]),
                children: std::collections::HashMap::new()
            })
        );
    }
}
