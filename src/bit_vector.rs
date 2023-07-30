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

    /// Return number of `target` in `B[0..index]`
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

        self.iter().position(|&x| {
            if x == target {
                count += 1;
            }
            count == index
        })
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rank_works() {
        let bv = BitVector::new(vec![0, 1, 0, 1, 1, 0, 1, 0, 1]);
        assert_eq!(bv.rank(0, 0), Some(1));
        assert_eq!(bv.rank(0, 1), Some(1));
        assert_eq!(bv.rank(0, 2), Some(2));
        assert_eq!(bv.rank(0, 3), Some(2));
        assert_eq!(bv.rank(0, 4), Some(2));
        assert_eq!(bv.rank(0, 5), Some(3));
        assert_eq!(bv.rank(0, 6), Some(3));
        assert_eq!(bv.rank(0, 7), Some(4));
        assert_eq!(bv.rank(0, 8), Some(4));
        assert_eq!(bv.rank(0, 9), None);
        assert_eq!(bv.rank(1, 0), Some(0));
        assert_eq!(bv.rank(1, 1), Some(1));
        assert_eq!(bv.rank(1, 2), Some(1));
        assert_eq!(bv.rank(1, 3), Some(2));
        assert_eq!(bv.rank(1, 4), Some(3));
        assert_eq!(bv.rank(1, 5), Some(3));
        assert_eq!(bv.rank(1, 6), Some(4));
        assert_eq!(bv.rank(1, 7), Some(4));
        assert_eq!(bv.rank(1, 8), Some(5));
        assert_eq!(bv.rank(1, 9), None);
    }

    #[test]
    fn select_works() {
        let bv = BitVector::new(vec![0, 1, 0, 1, 1, 0, 1, 0, 1]);
        assert_eq!(bv.select(0, 1), Some(0));
        assert_eq!(bv.select(0, 2), Some(2));
        assert_eq!(bv.select(0, 3), Some(5));
        assert_eq!(bv.select(0, 4), Some(7));
        assert_eq!(bv.select(0, 5), None);
        assert_eq!(bv.select(1, 1), Some(1));
        assert_eq!(bv.select(1, 2), Some(3));
        assert_eq!(bv.select(1, 3), Some(4));
        assert_eq!(bv.select(1, 4), Some(6));
        assert_eq!(bv.select(1, 5), Some(8));
        assert_eq!(bv.select(1, 6), None);
    }
}
