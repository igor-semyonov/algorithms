#![allow(dead_code)]

fn main() {
    println!("Hello, world!");
}

fn insertion_sort<T>(input: &mut [T])
where
    T: std::fmt::Debug + Ord + Clone,
{
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

fn merge_sort<T>(input: &mut [T])
where
    T: std::fmt::Debug + Ord + Clone,
{
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    fn get_sorting_functions<T>()
    -> HashMap<String, fn(&mut [T])>
    where
        T: std::fmt::Debug + Ord + Clone,
    {
        let mut sorting_functions: HashMap<
            _,
            fn(&mut [T]),
        > = HashMap::new();
        sorting_functions.insert(
            String::from("insertion_sort"),
            insertion_sort,
        );
        sorting_functions.insert(
            String::from("merge_sort"),
            merge_sort,
        );
        sorting_functions
    }

    #[test]
    fn empty_list() {
        let sorting_functions = get_sorting_functions();
        for (sorting_function_name, sorting_function) in
            sorting_functions
        {
            let mut array: Vec<u8> = vec![];
            sorting_function(&mut array);
            println!(
                "Testing {}.",
                &sorting_function_name
            );
            assert!(array.len() == 0);
            println!(
                "Finished {}.",
                &sorting_function_name
            );
        }
    }

    #[test]
    fn singleton_list() {
        let sorting_functions = get_sorting_functions();
        for (sorting_function_name, sorting_function) in
            sorting_functions
        {
            let mut array: Vec<u8> = vec![0];
            sorting_function(&mut array);
            println!(
                "Testing {}.",
                &sorting_function_name
            );
            assert!(array.len() == 1);
            assert!(array[0] == 0);
            println!(
                "Finished {}.",
                &sorting_function_name
            );
        }
    }

    #[test]
    fn basic_list() {
        let sorting_functions = get_sorting_functions();
        for (sorting_function_name, sorting_function) in
            sorting_functions
        {
            let mut array: Vec<u8> = vec![8, 3, 11, 2, 15];
            sorting_function(&mut array);
            println!(
                "Testing {}.",
                &sorting_function_name
            );
            assert_eq!(
                &array,
                &[2, 3, 8, 11, 15]
            );
            println!(
                "Finished {}.",
                &sorting_function_name
            );
        }
    }

    #[test]
    fn inverted() {
        let sorting_functions = get_sorting_functions();
        for upper_bound in [1_000, 1_001] {
            for (sorting_function_name, sorting_function) in
                &sorting_functions
            {
                let mut array: Vec<u32> = (0..upper_bound)
                    .rev()
                    .collect();
                let mut correct = array.clone();
                correct.sort();
                sorting_function(&mut array);
                println!(
                    "Testing {}.",
                    &sorting_function_name
                );
                assert_eq!(
                    &array,
                    &correct
                );
                println!(
                    "Finished {}.",
                    &sorting_function_name
                );
            }
        }
    }

    #[test]
    fn already_sorted() {
        let sorting_functions = get_sorting_functions();
        for upper_bound in [1_000, 1_001] {
            for (sorting_function_name, sorting_function) in
                &sorting_functions
            {
                let mut array: Vec<u32> =
                    (0..upper_bound).collect();
                let correct = array.clone();
                sorting_function(&mut array);
                println!(
                    "Testing {}.",
                    &sorting_function_name
                );
                assert_eq!(
                    &array,
                    &correct
                );
                println!(
                    "Finished {}.",
                    &sorting_function_name
                );
            }
        }
    }
}
