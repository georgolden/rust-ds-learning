#[cfg(test)]
mod tests {
    use crate::array::find_element_arr;

    mod find_element_arr_tests {
        use super::*;

        #[test]
        fn test_empty_arr() {
            assert_eq!(find_element_arr(&[], 0), -1);
        }
        #[test]
        fn test_no_element() {
            assert_eq!(find_element_arr(&[1, 2, 3], 4), -1);
        }
        #[test]
        fn test_typical_case() {
            assert_eq!(find_element_arr(&[1, 2, 3], 2), 1);
        }
        #[test]
        fn test_one_element_arr() {
            assert_eq!(find_element_arr(&[1], 1), 0);
        }
    }
}
