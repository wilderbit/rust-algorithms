// Dynamic Array
use std::mem::MaybeUninit;

pub struct DArr<T, const N: usize> {
    cap: usize,
    len: usize,
    pointer: Box<[MaybeUninit<T>; 1000]>,
}

impl<T: Copy, const N: usize> DArr<T, N> {
    pub fn new() -> Self {
        Self::new_with_capacity(16)
    }

    pub fn new_with_capacity(cap: usize) -> Self {
        let arr = [MaybeUninit::<T>::uninit(); 1000];
        let boxed = Box::new(arr);
        Self {
            cap: 1000,
            len: 0,
            pointer: boxed,
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn insert(&mut self, element: T) {
        // check if there is a capacity
        // if capacity is full, resize the array
        // insert the element
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        None
    }

    pub fn resize(&mut self) {}

    pub fn capacity(&self) -> usize {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let _arr: DArr<i32, 10> = DArr::new();
    }
}
