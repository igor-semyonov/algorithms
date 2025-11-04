#![allow(
    dead_code,
    unused_imports
)]

mod heap;
use crate::heap::{Heap, build_max_heap, max_heapify};

fn main() {
    let mut v = vec![8, 11, 3, 1, 5];
    quick_sort(&mut v);
    println!(
        "{:?}",
        v
    );
}

fn insertion_sort<T: Ord + Clone>(input: &mut [T]) {
    for i in 1..input.len() {
        for j in (1..=i).rev() {
            if input[j] < input[j - 1] {
                let tmp = input[j].clone();
                input[j] = input[j - 1].clone();
                input[j - 1] = tmp.clone();
            }
        }
    }
}

fn merge_sort<T: Ord + Clone>(input: &mut [T]) {
    let input_len = input.len();
    let input_ptr = input.as_mut_ptr();
    if input_len <= 1 {
        return;
    }
    let (left, right) = input.split_at_mut(input_len / 2);
    merge_sort(left);
    merge_sort(right);
    let input = unsafe {
        std::slice::from_raw_parts_mut(
            input_ptr, input_len,
        )
    };

    if left
        .last()
        .unwrap()
        <= right
            .first()
            .unwrap()
    {
        return;
    }
    let mut left_cursor = 0;
    let mut right_cursor = left.len();
    while left_cursor < right_cursor
        && right_cursor < input_len
    {
        if input[left_cursor] <= input[right_cursor] {
            left_cursor += 1;
        } else {
            let right_value = input[right_cursor].clone();
            for i in (left_cursor..right_cursor).rev() {
                input[i + 1] = input[i].clone();
            }
            input[left_cursor] = right_value.clone();
            left_cursor += 1;
            right_cursor += 1;
        }
    }
}

fn heap_sort<T: Ord + Clone>(input: &mut [T]) {
    build_max_heap(input);
    for idx in (1..input.len()).rev() {
        (
            input[0], input[idx],
        ) = (
            input[idx].clone(),
            input[0].clone(),
        );
        let (subslice, _) = input.split_at_mut(idx);
        max_heapify(
            subslice, 0,
        );
    }
}

fn quick_sort<T: Ord + Clone>(input: &mut [T]) {
    if input.len() >= 2 {
        let pivot_idx = quick_sort_partition(input);
        let (lower_half, tmp) =
            input.split_at_mut(pivot_idx);
        let (_, upper_half) = tmp.split_at_mut(1);
        quick_sort(lower_half);
        quick_sort(upper_half);
    }
}

fn quick_sort_partition<T: Ord + Clone>(
    input: &mut [T],
) -> usize {
    let old_pivot_idx = input.len() - 1;
    let pivot = input[old_pivot_idx].clone();
    let mut new_pivot_idx = 0;
    for idx in 0..input.len() {
        let value = input[idx].clone();
        if value < pivot {
            if idx != 0 {
                (
                    input[new_pivot_idx],
                    input[idx],
                ) = (
                    value,
                    input[new_pivot_idx].clone(),
                );
            }
            new_pivot_idx += 1;
        }
    }
    (
        input[old_pivot_idx],
        input[new_pivot_idx],
    ) = (
        input[new_pivot_idx].clone(),
        pivot,
    );
    new_pivot_idx
}

#[cfg(test)]
mod tests {
    use super::*;
    use paste::paste;
    macro_rules! create_sorting_tests {
        ($($test_name:ident: $sorting_function:path,)*) => {
        paste! {
                $(
    #[test]
    fn [<$test_name _empty_list>]() {
        let mut array: Vec<u8> = vec![];
        $sorting_function(&mut array);
        assert!(array.len() == 0);
    }

    #[test]
    fn [<$test_name _singleton_list>]() {
        let mut array: Vec<u8> = vec![0];
        $sorting_function(&mut array);
        assert!(array.len() == 1);
        assert!(array[0] == 0);
    }

    #[test]
    fn [<$test_name _basec_list>]() {
        let mut array: Vec<u8> = vec![8, 3, 11, 2, 15];
        $sorting_function(&mut array);
        assert_eq!(
            &array,
            &[2, 3, 8, 11, 15]
        );
    }

    #[test]
    fn [<$test_name _inverted>]() {
        for upper_bound in [1_000, 1_001] {
            let mut array: Vec<u32> = (0..upper_bound)
                .rev()
                .collect();
            let mut correct = array.clone();
            correct.sort();
            $sorting_function(&mut array);
            assert_eq!(
                &array,
                &correct
            );
        }
    }

                    #[test]
    fn [<$test_name _already_sorted>]() {
        for upper_bound in [1_000, 1_001] {
            let mut array: Vec<u32> =
                (0..upper_bound).collect();
            let correct = array.clone();
            $sorting_function(&mut array);
            assert_eq!(
                &array,
                &correct
            );
        }
    }
                )*
            }
        }
    }

    create_sorting_tests![
        insertion_sort: insertion_sort,
        merge_sort: merge_sort,
        heap_sort: heap_sort,
        quick_sort: quick_sort,
    ];
}
