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

    /// Return number of `target` in bit_vector[0..i]
    pub fn rank(&self, target: u8, index: usize) -> usize {
        let mut count = 0;
        for j in 0..=index {
            if self[j] == target {
                count += 1;
            }
        }
        count
    }

    // Return position of `i-th` 1 from the start
    pub fn select(&self, target: u8, index: usize) -> usize {
        if index < 1 {
            panic!("i must be greater than 0");
        }
        let mut count = 0;
        for j in 0..self.data.len() {
            if self[j] == target {
                count += 1;
            }
            if count == index {
                return j;
            }
        }
        panic!("No {}-th 1", index);
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
