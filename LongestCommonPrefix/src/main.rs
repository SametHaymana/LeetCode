use std::{collections::HashMap};

#[derive(Default, Debug)]
struct TrieNode {
    is_end_of_word: bool,
    children: HashMap<char, TrieNode>,
}

#[derive(Default, Debug)]
pub struct Trie {
    root: TrieNode,
}

impl Trie {
    pub fn new() -> Self {
        Trie {
            root: TrieNode::default(),
        }
    }

    pub fn insert(&mut self, word: &str) {
        let mut current_node = &mut self.root;

        for c in word.chars() {
            current_node = current_node.children.entry(c).or_default();
        }
        current_node.is_end_of_word = true;
    }

    pub fn contains(&self, word: &str) -> bool {
        let mut current_node = &self.root;

        for c in word.chars() {
            match current_node.children.get(&c) {
                Some(node) => current_node = node,
                None => return false,
            }
        }

        current_node.is_end_of_word
    }

    pub fn longest_comman_prefix(&mut self) -> String{
        let mut current_node = &self.root;
        let mut prefix = String::new();
       
        while !current_node.is_end_of_word && current_node.children.len() == 1 {
            let (c, node) = current_node.children.iter().next().unwrap();
            prefix.push(*c);
            current_node = node;
        }

        return prefix;
    }
}





struct Solution;

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let mut trie = Trie::new();

        // Insert to trie
        for str in strs.iter() {
            trie.insert(str);
        }


        trie.longest_comman_prefix()
    }
}



fn main() {
    let strs = vec!["flower".to_string(), "flow".to_string(), "flight".to_string()];
    let result = Solution::longest_common_prefix(strs);
    println!("Result: {}", result);

}
