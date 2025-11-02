pub fn build_max_heap<T: Ord + Clone>(input: &mut [T]) {
    for idx in (0..input.len() >> 1).rev() {
        max_heapify(
            input, idx,
        );
    }
}
pub fn max_heapify<T: Ord + Clone>(
    input: &mut [T],
    idx: usize,
) {
    let left = (idx << 1) + 1;
    let right = left + 1;
    let mut largest = idx;
    if left < input.len() && input[left] > input[largest] {
        largest = left;
    }
    if right < input.len() && input[right] > input[largest]
    {
        largest = right;
    }
    if largest != idx {
        (
            input[idx],
            input[largest],
        ) = (
            input[largest].clone(),
            input[idx].clone(),
        );
        max_heapify(
            input, largest,
        );
    }
}
fn get_parent_idx(idx: usize) -> usize {
    (idx + 1 >> 1) - 1
}
fn get_children_idx(
    idx: usize,
) -> (
    usize,
    usize,
) {
    let child1_idx = (idx + 1 << 1) - 1;
    let child2_idx = child1_idx + 1;
    (
        child1_idx, child2_idx,
    )
}

#[derive(Debug, Clone)]
pub struct Heap<T: Ord + Clone + std::fmt::Debug> {
    data: Vec<T>,
}
impl<T: Ord + Clone + std::fmt::Debug> From<Vec<T>>
    for Heap<T>
{
    fn from(v: Vec<T>) -> Self {
        let mut heap = Heap::new();
        heap.data = v;
        heap
    }
}
impl<T: Ord + Clone + std::fmt::Debug> Heap<T> {
    /// Make a new heap.
    pub fn new() -> Self {
        let data: Vec<T> = vec![];
        Heap {
            data,
        }
    }

    /// Returns the height of the heap. This is defined to
    /// be -1 when there are 0 elements. For non-zero size,
    /// it is defined as the floor of the base
    /// 2 logorithm of the size of the heap as returned by
    /// [`Heap::size`].
    pub fn height(&self) -> i32 {
        core::mem::size_of_val(&self.size()) as i32 * 8
            - self
                .size()
                .leading_zeros() as i32
            - 1
    }

    /// Returns the number of elements.
    pub fn size(&self) -> usize {
        self.data
            .len()
    }

    /// Apply [`Heap::max_heapify`] until the heap has been
    /// fully transformed to a max heap.
    pub fn build_max_heap(&mut self) {
        build_max_heap(
            self.data
                .as_mut_slice(),
        );
    }

    /// Apply [`max_heapify`] starting
    /// at a given index of the underlying array.
    fn max_heapify(&mut self, idx: usize) {
        // println!(
        //     "Current: {:?}\tIndex: {}",
        //     self.data, idx
        // );
        max_heapify(
            self.data
                .as_mut_slice(),
            idx,
        );
    }

    /// Push an item to the heap then sift it up.
    pub fn push(&mut self, value: T) {
        self.data
            .push(value);
        let mut node_idx = self.size() - 1;
        loop {
            if node_idx == 0 {
                break;
            }
            let parent_idx = get_parent_idx(node_idx);
            if self[parent_idx] < self[node_idx] {
                (
                    self[node_idx],
                    self[parent_idx],
                ) = (
                    self[parent_idx].clone(),
                    self[node_idx].clone(),
                );
                node_idx = parent_idx;
            } else {
                break;
            }
        }
    }

    /// Pop the max value and then sift down to maintain the
    /// heap.
    pub fn pop(&mut self) -> Option<T> {
        if self.size() == 0 {
            return None;
        }
        let size = self.size();
        let last_idx = size - 1;
        let new_size = last_idx;
        (
            self[0],
            self[last_idx],
        ) = (
            self[last_idx].clone(),
            self[0].clone(),
        );
        let mut node_idx = 0;
        while node_idx < new_size {
            let (child1_idx, child2_idx) =
                get_children_idx(node_idx);
            if child1_idx >= new_size {
                break;
            } else if child2_idx >= new_size {
                if self[node_idx] < self[child1_idx] {
                    (
                        self[node_idx],
                        self[child1_idx],
                    ) = (
                        self[child1_idx].clone(),
                        self[node_idx].clone(),
                    );
                }
                break;
            } else {
                let (max_child_idx, max_child_value) =
                    match self[child1_idx]
                        >= self[child2_idx]
                    {
                        true => (
                            child1_idx,
                            self[child1_idx].clone(),
                        ),
                        false => (
                            child2_idx,
                            self[child2_idx].clone(),
                        ),
                    };
                if self[node_idx] < max_child_value {
                    (
                        self[node_idx],
                        self[max_child_idx],
                    ) = (
                        max_child_value,
                        self[node_idx].clone(),
                    );
                    node_idx = max_child_idx;
                } else {
                    break;
                }
            }
        }
        self.data
            .pop()
    }

    pub fn peak(&self) -> Option<&T> {
        self.data
            .first()
    }

    /// Check that the heap satisfies the property that each
    /// element is less than or equal to its parent
    pub fn is_valid(&self) -> bool {
        for node_idx in 1..self.size() {
            let node_value = &self[node_idx];
            let parent_idx = get_parent_idx(node_idx);
            let parent_value = &self[parent_idx];
            if node_value > parent_value {
                eprintln!(
                    "Invalid binary heap!\n   Node index {:4} has value {:5?}\n Parent index {:4} has value {:5?}",
                    node_idx,
                    node_value,
                    parent_idx,
                    parent_value
                );
                return false;
            }
        }
        true
    }
}

impl<T: Ord + Clone + std::fmt::Debug>
    std::ops::Index<usize> for Heap<T>
{
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl<T: Ord + Clone + std::fmt::Debug>
    std::ops::IndexMut<usize> for Heap<T>
{
    fn index_mut(
        &mut self,
        index: usize,
    ) -> &mut Self::Output {
        &mut self.data[index]
    }
}

#[cfg(test)]
mod tests {
    use crate::heap::Heap;

    #[test]
    fn size_and_height() {
        let mut heap: Heap<i32> = vec![].into();
        assert_eq!(
            heap.size(),
            0
        );
        assert_eq!(
            heap.height(),
            -1
        );
        heap.push(1);
        assert_eq!(
            heap.size(),
            1
        );
        assert_eq!(
            heap.height(),
            0
        );
        heap.push(1);
        assert_eq!(
            heap.size(),
            2
        );
        assert_eq!(
            heap.height(),
            1
        );
        heap.push(1);
        assert_eq!(
            heap.size(),
            3
        );
        assert_eq!(
            heap.height(),
            1
        );
        heap.push(1);
        assert_eq!(
            heap.size(),
            4
        );
        assert_eq!(
            heap.height(),
            2
        );
        heap.push(1);
    }

    #[test]
    fn pushing_popping_and_peaking() {
        let mut heap: Heap<i32> = Heap::new();
        assert!(heap.is_valid());
        for _v in vec![5, 3, 8, 11] {
            assert!(heap.is_valid());
            assert!(
                heap.peak()
                    == heap
                        .data
                        .iter()
                        .max()
            )
        }
        while let Some(val) = heap.pop() {
            let peaked = heap
                .peak()
                .unwrap()
                .clone();
            let max = heap
                .data
                .iter()
                .max()
                .unwrap()
                .clone();
            assert!(heap.is_valid());
            assert!(max <= val);
            assert!(peaked == max);
        }
        assert!(heap.is_valid());
        assert!(heap.peak().is_none());
    }

    #[test]
    fn build_max_heap_empty() {
        let mut heap: Heap<i32> = vec![].into();
        heap.build_max_heap();
        assert_eq!(
            heap.size(),
            0
        );
    }

    #[test]
    fn build_max_heap_singleton() {
        let mut heap: Heap<_> = vec![5].into();
        heap.build_max_heap();
        assert_eq!(
            heap.size(),
            1
        );
        assert_eq!(
            heap[0],
            5
        );
    }

    #[test]
    fn build_max_heap() {
        let mut heap: Heap<_> =
            vec![5, 10, 15, 3, 8, 21].into();
        heap.build_max_heap();
        assert_eq!(
            heap[0],
            21
        );
    }
}
