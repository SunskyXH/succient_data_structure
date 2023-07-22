mod succinct;

use crate::succinct::wavelet_tree::{construct_codex, WaveletTreeNode};
use std::ops::Deref;

fn main() {
    let t = "abcafcgbagcb$";
    let codex = construct_codex("$abcdefg");
    let root = WaveletTreeNode::new(t, &codex);

    for i in 0..t.len() {
        assert_eq!(
            root.access(i).deref(),
            codex.get(&t.chars().nth(i).unwrap()).unwrap()
        );
    }
    // the T[0..8] contains 3 'a's
    assert_eq!(root.rank(&"a".chars().next().unwrap(), 8), 3);
    // the 3rd 'a' is at T[8]
    assert_eq!(root.select(&"a".chars().next().unwrap(), 3), 8);
}
