/*
	heap
	This question requires you to implement a binary heap function
*/
use std::cmp::Ord;
use std::default::Default;

pub struct Heap<T>
where
    T: Default,
{
    count: usize,
    items: Vec<T>,
    comparator: fn(&T, &T) -> bool,
}

impl<T> Heap<T>
where
    T: Default + Clone + Copy,
{
    pub fn new(comparator: fn(&T, &T) -> bool) -> Self {
        Self {
            count: 0,
            items: vec![T::default()],
            comparator,
        }
    }

    pub fn len(&self) -> usize {
        self.count
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    fn parent_idx(&self, idx: usize) -> usize {
        idx / 2
    }

    fn children_present(&self, idx: usize) -> bool {
        self.left_child_idx(idx) <= self.count
    }

    fn left_child_idx(&self, idx: usize) -> usize {
        idx * 2
    }

    fn right_child_idx(&self, idx: usize) -> usize {
        self.left_child_idx(idx) + 1
    }

    fn smallest_child_idx(&self, idx: usize) -> usize {
        //TODO
        let len = self.items.len();
        if len == 0 {
            0
        }
        else {
            len - 1
        }
    }
}

impl<T> Heap<T>
where
    T: Default + Ord + Clone + Copy + std::fmt::Debug,
{
    /// Create a new MinHeap
    pub fn new_min() -> Self {
        Self::new(|a, b| a < b)
    }

    /// Create a new MaxHeap
    pub fn new_max() -> Self {
        Self::new(|a, b| a > b)
    }

    pub fn add(&mut self, value: T) {
        self.count += 1;
        self.items.resize(self.count + 1, value);
        if self.count == 1 {
            self.items[1] = value;
            return;
        }
        let mut parent = self.parent_idx(self.count);
        println!("start");
        while parent != 0 {
            self.add_helper(parent);
            println!("parent = {parent}, self.items = {:?}", self.items);
            parent = self.parent_idx(parent);
        }
    }

    fn pop(&mut self) -> Option<T> {
        if self.count == 0 {
            None
        }
        else {
            let value = self.items[1];
            self.items[1] = self.items[self.count];
            self.items[self.count] = value;
            self.count -= 1;
            self.add_helper(1);
            self.items.pop()
        }
    }

    fn add_helper(&mut self, parent: usize) {
        println!("parent = {}", parent);
        let (lidx, ridx) = (self.left_child_idx(parent), self.right_child_idx(parent));
        let value = self.items[parent];
        match ( lidx <= self.count && (self.comparator)(&value, &self.items[lidx]), ridx <= self.count && (self.comparator)(&value, &self.items[ridx]) ) {
            (true, true) => {
                println!("all right value < left and value < right");
            },
            (true, false) => {
                println!("path 2");
                if ridx <= self.count {
                    self.items[parent] = self.items[ridx];
                    self.items[ridx] = value;
                    self.add_helper(ridx);
                }
            }
            (false, true) => {
                println!("path 3");
                self.items[parent] = self.items[lidx];
                self.items[lidx] = value;
                self.add_helper(lidx);
            }
            (false, false) => {
                println!("path 4");
                if lidx > self.count {
                    println!("path 4.1");
                    return;
                }
                else if ridx > self.count {
                    println!("path 4.2");
                    self.items[parent] = self.items[lidx];
                    self.items[lidx] = value;
                    self.add_helper(lidx);
                }
                else {
                    println!("path 4.3");
                    if (self.comparator)(&self.items[lidx], &self.items[ridx]) {
                        if !(self.comparator)(&value, &self.items[lidx]) {
                            self.items[parent] = self.items[lidx];
                            self.items[lidx] = value;
                            self.add_helper(lidx);
                        }
                    }
                    else {
                        if !(self.comparator)(&value, &self.items[ridx]) {
                            self.items[parent] = self.items[ridx];
                            self.items[ridx] = value;
                            self.add_helper(ridx);
                        }
                    }
                }
            }
        }
    }

}

impl<T> Iterator for Heap<T>
where
    T: Default + Ord + Copy + Clone + std::fmt::Debug,
{
    type Item = T;

    fn next(&mut self) -> Option<T> {
        println!("pop self.items = {:?}", self.items);
        self.pop()
    }
}

pub struct MinHeap;

impl MinHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord + Copy + Clone + std::fmt::Debug,
    {
        Heap::new(|a, b| a < b)
    }
}

pub struct MaxHeap;

impl MaxHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord + Copy + Clone + std::fmt::Debug,
    {
        Heap::new(|a, b| a > b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_empty_heap() {
        let mut heap = MaxHeap::new::<i32>();
        assert_eq!(heap.next(), None);
    }

    #[test]
    fn test_min_heap() {
        let mut heap = MinHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(2));
        assert_eq!(heap.next(), Some(4));
        assert_eq!(heap.next(), Some(9));
        heap.add(1);
        assert_eq!(heap.next(), Some(1));
    }

    #[test]
    fn test_max_heap() {
        let mut heap = MaxHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(11));
        assert_eq!(heap.next(), Some(9));
        assert_eq!(heap.next(), Some(4));
        heap.add(1);
        assert_eq!(heap.next(), Some(2));
    }
}
