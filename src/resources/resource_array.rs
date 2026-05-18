use std::{
    array::IntoIter,
    ops::{Add, AddAssign, Index, IndexMut, Sub, SubAssign},
};

use crate::resources::{RESOURCE_COUNT, ResourceType};
#[derive(Clone, Copy, Debug)]
pub struct ResValArray(pub [usize; RESOURCE_COUNT]);
impl ResValArray {
    pub fn new() -> Self {
        Self([0; RESOURCE_COUNT])
    }
    pub fn gte_all(self, other: Self) -> bool {
        let mut greater = true;
        for i in 0..RESOURCE_COUNT {
            if self[i] < other[i] {
                greater = false;
                break;
            }
        }
        greater
    }
    pub fn add_cost(mut self, amt: usize, res_type: ResourceType) -> Self {
        self[res_type as usize] = amt;
        self
    }
    pub fn get_val(self, idx: usize) -> usize {
        if idx > RESOURCE_COUNT {
            return 0;
        }
        return self[idx];
    }
}

impl Add for ResValArray {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut result = [0; RESOURCE_COUNT];
        for i in 0..RESOURCE_COUNT {
            result[i] = self.0[i].saturating_add(rhs.0[i]);
        }
        Self(result)
    }
}

impl Sub for ResValArray {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut result = [0; RESOURCE_COUNT];
        for i in 0..RESOURCE_COUNT {
            result[i] = self.0[i].saturating_sub(rhs.0[i]);
        }
        Self(result)
    }
}

impl IndexMut<usize> for ResValArray {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut usize {
        &mut self.0[index]
    }
}

impl Index<usize> for ResValArray {
    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }

    type Output = usize;
}
impl AddAssign for ResValArray {
    fn add_assign(&mut self, rhs: Self) {
        for i in 0..RESOURCE_COUNT {
            self.0[i] = self.0[i].saturating_add(rhs.0[i]);
        }
    }
}
impl SubAssign for ResValArray {
    fn sub_assign(&mut self, rhs: Self) {
        for i in 0..RESOURCE_COUNT {
            self.0[i] = self.0[i].saturating_sub(rhs.0[i]);
        }
    }
}
impl IntoIterator for ResValArray {
    type IntoIter = IntoIter<usize, RESOURCE_COUNT>;
    type Item = usize;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}
