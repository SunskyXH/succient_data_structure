use std::ops::Deref;
use std::ops::{Index, IndexMut};
#[derive(Debug)]
pub struct BitVector {
    data: Vec<u8>,
}

impl BitVector {
    pub fn new(data: Vec<u8>) -> BitVector {
        BitVector { data }
    }

    /// Return number of `target` in bit_vector[0..index]
    pub fn rank(&self, target: u8, index: usize) -> Option<usize> {
        if index >= self.len() {
            return None;
        }
        Some(
            self.iter()
                .take(index + 1)
                .filter(|&&x| x == target)
                .count(),
        )
    }

    /// Return the index of `index`-th `target` from the start
    pub fn select(&self, target: u8, index: usize) -> Option<usize> {
        if index < 1 {
            return None;
        }
        let mut count = 0;
        for (i, &x) in self.iter().enumerate() {
            if x == target {
                count += 1;
                if count == index {
                    return Some(i);
                }
            }
        }
        None
    }
}

impl Index<usize> for BitVector {
    type Output = u8;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl IndexMut<usize> for BitVector {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}

impl PartialEq<Vec<u8>> for BitVector {
    fn eq(&self, other: &Vec<u8>) -> bool {
        &self.data == other
    }
}

impl Deref for BitVector {
    type Target = Vec<u8>;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}
