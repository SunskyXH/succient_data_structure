use super::bit_vector::BitVector;
use std::collections::HashMap;

pub enum WaveletTreeNode {
    Empty,
    Node {
        bit_vector: BitVector,
        left: Box<WaveletTreeNode>,
        right: Box<WaveletTreeNode>,
        codex: Box<HashMap<char, Vec<u8>>>,
    },
}

impl WaveletTreeNode {
    pub fn new(t: &str) -> Self {
        let alphabet = get_alphabet(t);
        let codex = construct_codex(alphabet.as_str());
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
                codex: Box::new(codex.clone()),
            }
        }

        create_node(&t.bytes().collect::<Vec<_>>(), &codex, 0, max_depth)
    }

    /// Return `T[index]`
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
                    ..
                } => {
                    let bit = bit_vector[i];
                    s.push(bit);
                    i = bit_vector.rank(bit, i).unwrap() - 1;
                    current_node = if bit == 0 { left } else { right };
                }
            }
        }
        BitVector::new(s)
    }

    /// Return the number of `char` in `T[0..index]`
    pub fn rank(&self, char: &char, index: usize) -> Option<usize> {
        let mut current_node = self;
        let mut i = index;
        let mut count = 0;
        let codex = match self {
            WaveletTreeNode::Empty => return None,
            WaveletTreeNode::Node { codex, .. } => codex,
        };
        let t = codex.get(char).unwrap();
        let mut d = 0;
        loop {
            match current_node {
                WaveletTreeNode::Empty => break,
                WaveletTreeNode::Node {
                    bit_vector,
                    left,
                    right,
                    ..
                } => {
                    let bit = t[d];
                    count = match bit_vector.rank(bit, i) {
                        None => return None,
                        Some(c) => c,
                    };
                    i = count - 1;
                    current_node = if bit == 0 { left } else { right };
                    d += 1;
                }
            }
        }
        Some(count)
    }

    /// Return the index of the `i`-th `char` in `T`
    pub fn select(&self, char: &char, index: usize) -> Option<usize> {
        let mut current_node = self;
        let mut i = index;
        let codex = match self {
            WaveletTreeNode::Empty => {
                return None;
            }
            WaveletTreeNode::Node { codex, .. } => codex,
        };
        let coding_sequence = codex.get(char);
        match coding_sequence {
            None => {
                return None;
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
                            ..
                        } => {
                            stack.push(current_node);
                            let bit = t[depth];
                            i = match bit_vector.rank(bit, i) {
                                None => return None,
                                Some(c) => c,
                            };
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
                    i = match bit_vector.select(bit, i) {
                        None => return None,
                        Some(c) => c + 1,
                    };
                }
                // convert to 0-based index
                Some(i - 1)
            }
        }
    }

    /// Return the codex of the wavelet tree
    pub fn get_codex(&self) -> Option<&HashMap<char, Vec<u8>>> {
        match self {
            WaveletTreeNode::Empty => None,
            WaveletTreeNode::Node { codex, .. } => Some(codex),
        }
    }
}

/// Get alphabet from given string.
pub fn get_alphabet(s: &str) -> String {
    let mut alphabet: Vec<char> = Vec::new();
    let mut has_dollar_sign = false;

    for c in s.chars() {
        if c == '$' {
            has_dollar_sign = true;
        } else if c.is_alphabetic() && !alphabet.contains(&c) {
            alphabet.push(c);
        }
    }

    alphabet.sort();
    let mut result: String = alphabet.into_iter().collect();
    if has_dollar_sign {
        result.insert(0, '$');
    }
    result
}

/// Construct dict by given string.
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::ops::Deref;
    const T: &str = "abcafcgbagcb$";

    fn create_tree(t: &str) -> WaveletTreeNode {
        let root = WaveletTreeNode::new(t);
        match root {
            WaveletTreeNode::Empty => panic!("empty wavelet tree"),
            WaveletTreeNode::Node { .. } => root,
        }
    }

    #[test]
    fn access_works() {
        let root = create_tree(T);
        let codex = root.get_codex().unwrap();

        for i in 0..T.len() {
            assert_eq!(
                root.access(i).deref(),
                codex.get(&T.chars().nth(i).unwrap()).unwrap()
            );
        }
    }

    #[test]
    fn rank_works() {
        let root = create_tree(T);

        let a = 'a';
        assert_eq!(root.rank(&a, 0), Some(1));
        assert_eq!(root.rank(&a, 1), Some(1));
        assert_eq!(root.rank(&a, 2), Some(1));
        assert_eq!(root.rank(&a, 3), Some(2));
        assert_eq!(root.rank(&a, 4), Some(2));
        assert_eq!(root.rank(&a, 5), Some(2));
        assert_eq!(root.rank(&a, 6), Some(2));
        assert_eq!(root.rank(&a, 7), Some(2));
        assert_eq!(root.rank(&a, 8), Some(3));
        assert_eq!(root.rank(&a, 9), Some(3));
        assert_eq!(root.rank(&a, 10), Some(3));
        assert_eq!(root.rank(&a, 11), Some(3));
        assert_eq!(root.rank(&a, 12), Some(3));
        assert_eq!(root.rank(&a, 13), None);
    }

    #[test]
    fn select_works() {
        let root = create_tree(T);
        let a = 'a';
        assert_eq!(root.select(&a, 1), Some(0));
        assert_eq!(root.select(&a, 2), Some(3));
        assert_eq!(root.select(&a, 3), Some(8));
        assert_eq!(root.select(&a, 4), None);
    }
}
