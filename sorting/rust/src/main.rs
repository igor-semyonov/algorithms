fn main() {
    println!("Hello, world!");
}

fn insertion_sort<T: Ord + Copy>(input: &mut [T]) {
    for i in 1..input.len() {
        for j in (1..i).rev() {
            if input[j] < input[j - 1] {
                let tmp = input[j];
                input[j] = input[j - 1];
                input[j - 1] = tmp;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_list() {
        let mut array: Vec<u8> = vec![];
        insertion_sort(&mut array);
        assert!(array.len() == 0);
    }

    #[test]
    fn singleton_list() {
        let mut array: Vec<u8> = vec![0];
        insertion_sort(&mut array);
        assert!(array.len() == 1);
        assert!(array[0] == 0);
    }

    #[test]
    fn basic_list() {
        let mut array: Vec<u8> = vec![8, 3, 11, 2, 15];
        insertion_sort(&mut array);
        assert_eq!(
            &array,
            &[2, 3, 8, 11, 15]
        );
    }
}
