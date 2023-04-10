use fxhash::FxBuildHasher;
use std::collections::HashMap;

#[derive(Debug, Default)]
struct Node {
    at_end: bool,
    children: HashMap<u8, Node, FxBuildHasher>,
}

#[derive(Debug, Default)]
struct Trie {
    root: Node,
    len: usize,
}

impl Trie {
    fn new() -> Self {
        Self::default()
    }

    fn insert(&mut self, text: &str) {
        let mut current_node = &mut self.root;

        for c in text.bytes() {
            current_node = current_node.children.entry(c).or_default()
        }

        current_node.at_end = true;
        self.len += 1;
    }

    fn contains(&self, text: &str) -> bool {
        let mut current_node = &self.root;

        for c in text.bytes() {
            match current_node.children.get(&c) {
                Some(node) => current_node = node,
                None => return false,
            }
        }

        current_node.at_end
    }

    fn len(&self) -> usize {
        self.len
    }
}

fn main() {
    let mut urls = Trie::new();

    urls.insert("https://google.com");

    let contains = urls.contains("https://google.com");

    println!("{contains}")
}
