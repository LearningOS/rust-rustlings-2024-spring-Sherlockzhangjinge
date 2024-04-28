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
    T: Default,
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

    pub fn add(&mut self, value: T)
    where T:Clone {
        self.items.push(value); // Add the new element at the end
        self.count += 1; // Increment the count
    
        // Perform bubble-up to restore the heap property
        let mut current_idx = self.count; // Index of the newly added element
        let mut current_value = self.items[current_idx].clone(); // Clone the value for temporary storage
        while current_idx > 1 && (self.comparator)(&current_value, &self.items[self.parent_idx(current_idx)]) {
            let parent_idx = self.parent_idx(current_idx);
            self.items[current_idx] = self.items[parent_idx].clone(); // Move parent down
            current_idx = parent_idx; // Move up to parent index
        }
        self.items[current_idx] = current_value; // Insert the new value at the correct position
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
        let left_child_idx = self.left_child_idx(idx);
        let right_child_idx = self.right_child_idx(idx);
    
        if right_child_idx <= self.count && (self.comparator)(&self.items[right_child_idx], &self.items[left_child_idx]) {
            right_child_idx // Right child is smaller
        } else {
            left_child_idx // Left child is smaller or there's only a left child
        }
    }
    
}

impl<T> Heap<T>
where
    T: Default + Ord,
{
    /// Create a new MinHeap
    pub fn new_min() -> Self {
        Self::new(|a, b| a < b)
    }

    /// Create a new MaxHeap
    pub fn new_max() -> Self {
        Self::new(|a, b| a > b)
    }
}

impl<T> Iterator for Heap<T>
where
    T: Default+Clone,
{
    type Item = T;

    fn next(&mut self) -> Option<T> {
        if self.is_empty() {
            return None; // If the heap is empty, return None
        }
    
        let root_value = self.items[1].clone(); // Get the root value
        self.items.swap(1, self.count); // Swap root with last element
        self.items.pop(); // Remove the last element
        self.count -= 1; // Decrement the count
    
        // Perform bubble-down to restore the heap property
        let mut current_idx = 1; // Start at the root
        while self.children_present(current_idx) {
            let smallest_child_idx = self.smallest_child_idx(current_idx);
            if (self.comparator)(&self.items[smallest_child_idx], &self.items[current_idx]) {
                self.items.swap(current_idx, smallest_child_idx); // Swap with smallest child
                current_idx = smallest_child_idx; // Move down to smallest child index
            } else {
                break; // Heap property satisfied
            }
        }
    
        Some(root_value) // Return the removed root value
    }
    
}

pub struct MinHeap;

impl MinHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord,
    {
        Heap::new(|a, b| a < b)
    }
}

pub struct MaxHeap;

impl MaxHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord,
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