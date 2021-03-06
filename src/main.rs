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
    fn contains_where(&self, test: fn(key: K) -> bool) -> bool {
        self.children.keys().any(|&key| test(key))
    }

    //find every key that match test predica
    fn get_where(&self, test: fn(key: K) -> bool) -> Vec<TrieNode<K, V>> {
        let matching_keys = self
            .children
            .keys()
            .filter(|&&key| test(key))
            .collect::<Vec<&K>>();
        matching_keys
            .iter()
            .map(|&&key| self.get(key))
            .filter(|x| x.is_some())
            .map(|x| x.unwrap())
            .collect()
    }
}

fn chunk(input: &str, compared: &str) -> bool {
    let chars: Vec<char> = input.chars().collect();
    let slice = chars.as_slice();
    let mut result = Vec::new();
    for i in 1..=slice.len() {
        let s = String::from_iter(&slice[0..i].to_vec());
        result.push(s);
    }
    result.contains(&String::from(compared))
}
fn main() {
    let mut trie = TrieNode::new();
    trie.add(["Paris"], "Geo struct".to_owned());
    let result = trie.get_where(|inputs| inputs.iter().any(|input| chunk(&input, "Paris")));
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
            trie.get_where(|input| chunk(input, "Paris")),
            vec![TrieNode {
                value: Some("Geo struct".to_owned()),
                children: std::collections::HashMap::new()
            }]
        );
    }

    #[test]
    fn trie_chunk_vec_test() {
        let mut trie = TrieNode::new();
        let alternatives_paris = vec!["Paris", "Paris 15", "15ème arrondissement"];
        trie.add(&alternatives_paris, "paris geostruct".to_owned());
        let alternatives = vec!["Paradise Nevada"];
        trie.add(&alternatives, "Nevada geostruct".to_owned());
        assert_eq!(
            trie.get_where(|inputs| inputs.iter().any(|input| chunk(&input, "Par"))),
            vec![
                TrieNode {
                    value: Some("paris geostruct".to_owned()),
                    children: std::collections::HashMap::new()
                },
                TrieNode {
                    value: Some("Nevada geostruct".to_owned()),
                    children: std::collections::HashMap::new()
                }
            ]
        );
        assert_eq!(
            trie.get_where(|inputs| inputs.iter().any(|input| chunk(&input, "Pari"))),
            vec![TrieNode {
                value: Some("paris geostruct".to_owned()),
                children: std::collections::HashMap::new()
            }]
        );

        assert_eq!(
            trie.get_where(|inputs| inputs.iter().any(|input| chunk(&input, "15"))),
            vec![TrieNode {
                value: Some("paris geostruct".to_owned()),
                children: std::collections::HashMap::new()
            }]
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
            vec![TrieNode {
                value: Some(["whatever"]),
                children: std::collections::HashMap::new()
            }]
        );
    }
}
