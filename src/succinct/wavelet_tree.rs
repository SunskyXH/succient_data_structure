use super::bit_vector::BitVector;
use std::collections::HashMap;

pub enum WaveletTreeNode {
    Empty,
    Node {
        bit_vector: BitVector,
        left: Box<WaveletTreeNode>,
        right: Box<WaveletTreeNode>,
    },
}

impl WaveletTreeNode {
    pub fn new(string: &str, codex: &HashMap<char, Vec<u8>>) -> Self {
        let max_depth = codex.values().map(|v| v.len()).max().unwrap_or(0);

        fn create_node(
            string: &[u8],
            codex: &HashMap<char, Vec<u8>>,
            depth: usize,
            max_depth: usize,
        ) -> WaveletTreeNode {
            if string.is_empty() || depth >= max_depth {
                return WaveletTreeNode::Empty;
            }
            let (left_string, right_string): (Vec<_>, Vec<_>) = string
                .iter()
                .partition(|&&ch| codex[&(ch as char)][depth] == 0);

            let bit_vector = string
                .iter()
                .map(|&ch| codex[&(ch as char)][depth])
                .collect();

            WaveletTreeNode::Node {
                bit_vector: BitVector::new(bit_vector),
                left: Box::new(create_node(&left_string, codex, depth + 1, max_depth)),
                right: Box::new(create_node(&right_string, codex, depth + 1, max_depth)),
            }
        }

        create_node(&string.bytes().collect::<Vec<_>>(), codex, 0, max_depth)
    }

    pub fn access(&self, index: usize) -> BitVector {
        let mut current_node = self;
        let mut i = index;
        let mut s: Vec<u8> = vec![];
        loop {
            match current_node {
                WaveletTreeNode::Empty => break,
                WaveletTreeNode::Node {
                    bit_vector,
                    left,
                    right,
                } => {
                    let bit = bit_vector[i];
                    s.push(bit);
                    i = bit_vector.rank(bit, i) - 1;
                    current_node = if bit == 0 { left } else { right };
                }
            }
        }
        BitVector::new(s)
    }

    pub fn rank(&self, char: &char, index: usize) -> usize {
        let mut current_node = self;
        let mut i = index;
        let mut count = 0;
        let codex = construct_codex("$abcdefg");
        let t = codex.get(char).unwrap();
        let mut d = 0;
        loop {
            match current_node {
                WaveletTreeNode::Empty => break,
                WaveletTreeNode::Node {
                    bit_vector,
                    left,
                    right,
                } => {
                    let bit = t[d];
                    count = bit_vector.rank(bit, i);
                    i = count - 1;
                    current_node = if bit == 0 { left } else { right };
                    d += 1;
                }
            }
        }
        count
    }

    pub fn select(&self, char: &char, index: usize) -> usize {
        let mut current_node = self;
        let mut i = index;
        let codex = construct_codex("$abcdefg");
        let coding_sequence = codex.get(char);
        match coding_sequence {
            None => {
                panic!("given char is not in the codex");
            }
            Some(t) => {
                let mut depth = 0;
                // traverse to the leaf node, store the sequence of nodes in stack
                let mut stack: Vec<&WaveletTreeNode> = vec![];
                loop {
                    match current_node {
                        WaveletTreeNode::Empty => break,
                        WaveletTreeNode::Node {
                            bit_vector,
                            left,
                            right,
                        } => {
                            stack.push(current_node);
                            let bit = t[depth];
                            i = bit_vector.rank(bit, i);
                            current_node = if bit == 0 { left } else { right };
                            depth += 1;
                        }
                    }
                }
                // pop the stack and traverse back to the root node
                i = index;
                while let Some(WaveletTreeNode::Node { bit_vector, .. }) = stack.pop() {
                    depth -= 1;
                    let bit = t[depth];
                    i = bit_vector.select(bit, i) + 1;
                }
                // convert to 0-based index
                i - 1
            }
        }
    }
}

pub fn construct_codex(string: &str) -> HashMap<char, Vec<u8>> {
    let mut map = HashMap::new();
    let mut unique_chars = string.chars().collect::<Vec<_>>();
    unique_chars.sort_unstable();
    unique_chars.dedup();

    for (i, ch) in unique_chars.iter().enumerate() {
        let binary = format!("{:03b}", i); // Format into 3-bits binary
        let binary_vec = binary
            .chars()
            .map(|b| b.to_digit(2).unwrap() as u8)
            .collect();
        map.insert(*ch, binary_vec);
    }
    map
}
