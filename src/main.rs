use std::collections::HashMap;
use std::ops::Index;
use std::ops::IndexMut;

#[derive(Debug)]
struct TrieNode<K, V> {
    children: HashMap<K, TrieNode<K, V>>,
    // key: Option<K>,
    value: Option<V>,
}

// impl IndexMut<Side> for Balance {
//     fn index_mut(&mut self, index: Side) -> &mut Self::Output {
//         println!("Accessing {:?}-side of balance mutably", index);
//         match index {
//             Side::Left => &mut self.left,
//             Side::Right => &mut self.right,
//         }
//     }
// }

// impl<K,V> std::ops::IndexMut<K> for TrieNode<K,V>

// where K: std::cmp::Eq+std::hash::Hash
// {
// fn index_mut(&mut self, index: K) -> &mut Self::Output {

// }

impl<K, V> IndexMut<&K> for TrieNode<K, V>
where
    K: std::cmp::Eq + std::hash::Hash,
{
    fn index_mut(&mut self, key: &K) -> &mut Self::Output {
        self.children.get_mut(key).expect("no entry found for key")
    }
}

impl<K, V> Index<&K> for TrieNode<K, V>
where
    K: std::cmp::Eq + std::hash::Hash + std::borrow::Borrow<K>,
{
    type Output = TrieNode<K, V>;

    fn index(&self, key: &K) -> &Self::Output {
        self.children.get(key).expect("no entry found for key")
    }
}

type Callback<K> = fn(key: K) -> bool;

impl<K, V> TrieNode<K, V>
where
    K: std::cmp::Eq + std::hash::Hash + Copy,
{
    // fn new(key: K, value: V) -> Self {
    //     let children = HashMap::new();
    //     TrieNode {
    //         children: children,
    //         key: Some(key),
    //         value: Some(value),
    //     }
    // }
    fn contains(&self, key: K) -> bool {
        self.children.contains_key(&key)
    }

    fn add(&mut self, key: K, value: V) {
        self[&key] = TrieNode {
            children: HashMap::new(),
            // key: Some(key),
            value: Some(value),
        };
        // if let Some(x) = self.children.get_mut(&key) {
        //     *x = TrieNode {
        //         children: HashMap::new(),
        //         key: Some(key),
        //         value: Some(value),
        //     };
        // }
    }

    // fn get(&self, key: K) -> Option<&TrieNode<K, V>> {
    //     if self.contains(key) {
    //         self.children.get(&key)
    //     } else {
    //         panic!("No key found") //TODO: error type
    //     }
    // }

    // fn containsWhere(self, test: Callback<K>) -> bool {
    //     self.children.into_iter().any(|(key, _)| test(key))
    // }

    // fn getWhere(&self, test: Callback<K>) -> Option<TrieNode<K, V>> {
    //     let keys = self.children.into_iter().map(|(key, _)| key).collect();
    //     while let Some(key) = keys.pop() {
    //         if test(key) {
    //             self.children.get(&key)
    //         } else {
    //             break;
    //         }
    //     }
    // }
}

fn main() {
    let mut node = TrieNode {
        children: HashMap::new(),
        // key: Some("hey"),
        value: None,
    };
    node.add("hey", Some("it's me"));
    // node.add(Some("hey"), Some("it's me"));
    println!("{:?}", node);
    // let n = node.clone();
    // println!("{}", node.contains(Some("hey")));
    println!("{:?}", node);
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn trie_node_test() {
//         let mut node = TrieNode {};
//         node.add(
//             Some("hey".to_owned()),
//             Some(["hey".to_owned(), "hey".to_owned()]),
//         );
//         let eq = node.contains(Some("hey".to_owned()));
//         assert_eq!(eq, true);
//     }
// }
