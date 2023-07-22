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
    pub fn new(_string: &str) -> Self {
        // FIXME: hard coded
        // println!("string: {}", string);
        // let codex = construct_codex("$abcdefg");
        let n00 = WaveletTreeNode::Node {
            bit_vector: BitVector::new(vec![1, 1, 1, 0]),
            left: Box::new(WaveletTreeNode::Empty),
            right: Box::new(WaveletTreeNode::Empty),
        };
        let n01 = WaveletTreeNode::Node {
            bit_vector: BitVector::new(vec![0, 1, 1, 0, 1, 0]),
            left: Box::new(WaveletTreeNode::Empty),
            right: Box::new(WaveletTreeNode::Empty),
        };
        let n11 = WaveletTreeNode::Node {
            bit_vector: BitVector::new(vec![0, 1, 1]),
            left: Box::new(WaveletTreeNode::Empty),
            right: Box::new(WaveletTreeNode::Empty),
        };

        let n0 = WaveletTreeNode::Node {
            bit_vector: BitVector::new(vec![0, 1, 1, 0, 1, 1, 0, 1, 1, 0]),
            left: Box::new(n00),
            right: Box::new(n01),
        };

        let n1 = WaveletTreeNode::Node {
            bit_vector: BitVector::new(vec![1, 1, 1]),
            left: Box::new(WaveletTreeNode::Empty),
            right: Box::new(n11),
        };

        WaveletTreeNode::Node {
            bit_vector: BitVector::new(vec![0, 0, 0, 0, 1, 0, 1, 0, 0, 1, 0, 0, 0]),
            left: Box::new(n0),
            right: Box::new(n1),
        }
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

    pub fn rank(&self, char: &str, index: usize) -> usize {
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

    pub fn select(&self, char: &str, index: usize) -> usize {
        let mut current_node = self;
        let mut i = index;
        let codex = construct_codex("$abcdefg");
        let t = codex.get(char).unwrap();
        let mut d = 0;
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
                    let bit = t[d];
                    i = bit_vector.rank(bit, i);
                    current_node = if bit == 0 { left } else { right };
                    d += 1;
                }
            }
        }
        // pop the stack and traverse back to the root node
        i = index;
        loop {
            match stack.pop() {
                None => break,
                Some(node) => match node {
                    WaveletTreeNode::Empty => break,
                    WaveletTreeNode::Node { bit_vector, .. } => {
                        d -= 1;
                        let bit = t[d];
                        i = bit_vector.select(bit, i) + 1;
                    }
                },
            }
        }
        // convert to 0-based index
        i - 1
    }
}

pub fn construct_codex(_string: &str) -> HashMap<&str, Vec<u8>> {
    let mut map = HashMap::new();
    map.insert("$", vec![0, 0, 0]);
    map.insert("a", vec![0, 0, 1]);
    map.insert("b", vec![0, 1, 0]);
    map.insert("c", vec![0, 1, 1]);
    map.insert("d", vec![1, 0, 0]);
    map.insert("e", vec![1, 0, 1]);
    map.insert("f", vec![1, 1, 0]);
    map.insert("g", vec![1, 1, 1]);

    map
}
