mod succinct;

use crate::succinct::wavelet_tree::{construct_codex, WaveletTreeNode};
use std::ops::Deref;

fn main() {
    let t = "abcafcgbagcb$";
    let codex = construct_codex(t);
    let root = WaveletTreeNode::new(t);

    // the T[4] is 'f'
    assert_eq!(root.access(4).deref(), codex.get("f").unwrap());
    // the T[0..8] contains 3 'a's
    assert_eq!(root.rank("a", 8), 3);
    // the 3rd 'a' is at T[8]
    assert_eq!(root.select("a", 3), 8);
}
