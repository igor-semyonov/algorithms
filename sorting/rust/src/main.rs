#![allow(dead_code)]

fn main() {
    println!("Hello, world!");
}

fn insertion_sort<T>(input: &mut [T])
where
    T: std::fmt::Debug + Ord + Clone,
{
    for i in 1..input.len() {
        for j in (1..i).rev() {
            if input[j] < input[j - 1] {
                let tmp = input[j].clone();
                input[j] = input[j - 1].clone();
                input[j - 1] = tmp.clone();
            }
        }
    }
}

fn merge_sort<T>(input: &mut [T])
where
    T: std::fmt::Debug + Ord + Clone,
{
    let input_len = input.len();
    if input_len <= 1 {
        return;
    }
    let (left, right) = input.split_at_mut(input_len / 2);
    merge_sort(left);
    merge_sort(right);

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
    let mut right_cursor = 0;
    while left_cursor < left.len()
        && right_cursor < right.len()
    {
        if left[left_cursor] <= right[right_cursor] {
            left_cursor += 1;
        } else {
            let right_value = right[right_cursor].clone();
            right[right_cursor] = left
                .last()
                .unwrap()
                .clone();
            for i in (left_cursor..left.len() - 1).rev() {
                left[i + 1] = left[i].clone();
            }
            left[left_cursor] = right_value.clone();
            left_cursor += 1;
            right_cursor += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_sorting_functions<T>() -> Vec<fn(&mut [T])>
    where
        T: std::fmt::Debug + Ord + Clone,
    {
        vec![insertion_sort, merge_sort]
    }

    #[test]
    fn empty_list() {
        let sorting_functions = get_sorting_functions();
        for sorting_function in sorting_functions {
            let mut array: Vec<u8> = vec![];
            sorting_function(&mut array);
            assert!(array.len() == 0);
        }
    }

    #[test]
    fn singleton_list() {
        let sorting_functions = get_sorting_functions();
        for sorting_function in sorting_functions {
            let mut array: Vec<u8> = vec![0];
            sorting_function(&mut array);
            assert!(array.len() == 1);
            assert!(array[0] == 0);
        }
    }

    #[test]
    fn basic_list() {
        let sorting_functions = get_sorting_functions();
        for sorting_function in sorting_functions {
            let mut array: Vec<u8> = vec![8, 3, 11, 2, 15];
            sorting_function(&mut array);
            assert_eq!(
                &array,
                &[2, 3, 8, 11, 15]
            );
        }
    }

    #[test]
    fn inverted() {
        let sorting_functions = get_sorting_functions();
        for sorting_function in sorting_functions {
            let mut array: Vec<u32> = (0..1_000)
                .rev()
                .collect();
            let mut correct = array.clone();
            correct.sort();
            sorting_function(&mut array);
            assert_eq!(
                &array,
                &correct
            );
        }
    }

    #[test]
    fn already_sorted() {
        let sorting_functions = get_sorting_functions();
        for sorting_function in sorting_functions {
            let mut array: Vec<u32> = (0..1_000).collect();
            let correct = array.clone();
            sorting_function(&mut array);
            assert_eq!(
                &array,
                &correct
            );
        }
    }
}
