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
    fn new() -> Self {
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

    /// Add an item to the heap then reapply
    /// [`Heap::build_max_heap`].
    fn push(&mut self, value: T) {
        self.data
            .push(value);
        self.build_max_heap();
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
