// src/vector/tests.rs
#[cfg(test)]
mod tests {
    use crate::vector::{
        max_product, max_product_functional, merge_intervals, sliding_window_maximum,
    };

    mod sliding_window_tests {
        use super::*;

        #[test]
        fn test_empty_vector() {
            assert_eq!(sliding_window_maximum(&[], 1), Vec::<i32>::new());
        }

        #[test]
        fn test_window_size_one() {
            assert_eq!(sliding_window_maximum(&[1, 2, 3], 1), vec![1, 2, 3]);
        }

        #[test]
        fn test_typical_case() {
            let nums = vec![1, 3, -1, -3, 5, 3, 6, 7];
            assert_eq!(sliding_window_maximum(&nums, 3), vec![3, 3, 5, 5, 6, 7]);
        }

        #[test]
        fn test_minimal_window() {
            let nums = vec![1, -1];
            assert_eq!(sliding_window_maximum(&nums, 1), vec![1, -1]);
        }

        #[test]
        fn test_window_equals_array_size() {
            let nums = vec![1, 2, 3, 4, 5];
            assert_eq!(sliding_window_maximum(&nums, 5), vec![5]);
        }

        #[test]
        fn test_decreasing_sequence() {
            let nums = vec![5, 4, 3, 2, 1];
            assert_eq!(sliding_window_maximum(&nums, 3), vec![5, 4, 3]);
        }

        #[test]
        fn test_increasing_sequence() {
            let nums = vec![1, 2, 3, 4, 5];
            assert_eq!(sliding_window_maximum(&nums, 3), vec![3, 4, 5]);
        }

        #[test]
        fn test_all_same_numbers() {
            let nums = vec![1, 1, 1, 1];
            assert_eq!(sliding_window_maximum(&nums, 2), vec![1, 1, 1]);
        }

        #[test]
        fn test_negative_numbers() {
            let nums = vec![-7, -8, 7, 5, -7, 3];
            assert_eq!(sliding_window_maximum(&nums, 2), vec![-7, 7, 7, 5, 3]);
        }
    }

    mod merge_intervals_tests {
        use super::*;

        #[test]
        fn test_empty_intervals() {
            assert_eq!(merge_intervals(&[]), Vec::<(i32, i32)>::new());
        }

        #[test]
        fn test_single_interval() {
            assert_eq!(merge_intervals(&[(1, 3)]), vec![(1, 3)]);
        }

        #[test]
        fn test_typical_case() {
            let intervals = vec![(1, 3), (2, 6), (8, 10), (15, 18)];
            assert_eq!(merge_intervals(&intervals), vec![(1, 6), (8, 10), (15, 18)]);
        }

        #[test]
        fn test_touching_intervals() {
            let intervals = vec![(1, 4), (4, 5)];
            assert_eq!(merge_intervals(&intervals), vec![(1, 5)]);
        }

        #[test]
        fn test_completely_overlapping() {
            let intervals = vec![(1, 5), (2, 3)];
            assert_eq!(merge_intervals(&intervals), vec![(1, 5)]);
        }

        #[test]
        fn test_complex_overlapping() {
            let intervals = vec![(1, 4), (0, 2), (3, 5), (6, 7), (4, 6)];
            assert_eq!(merge_intervals(&intervals), vec![(0, 7)]);
        }

        #[test]
        fn test_no_overlapping() {
            let intervals = vec![(1, 2), (3, 4), (5, 6)];
            assert_eq!(merge_intervals(&intervals), vec![(1, 2), (3, 4), (5, 6)]);
        }

        #[test]
        fn test_negative_intervals() {
            let intervals = vec![(-5, -3), (-2, 0), (-1, 1)];
            assert_eq!(merge_intervals(&intervals), vec![(-5, -3), (-2, 1)]);
        }

        #[test]
        fn test_unsorted_input() {
            let intervals = vec![(4, 6), (1, 3), (2, 5)];
            assert_eq!(merge_intervals(&intervals), vec![(1, 6)]);
        }
    }

    mod max_product_tests {
        use super::*;

        #[test]
        fn test_empty_vec() {
            assert_eq!(max_product(&vec![]), 0);
        }

        #[test]
        fn test_single_element() {
            assert_eq!(max_product(&vec![5]), 5);
            assert_eq!(max_product(&vec![-3]), -3);
            assert_eq!(max_product(&vec![0]), 0);
        }

        #[test]
        fn test_two_elements() {
            assert_eq!(max_product(&vec![2, 3]), 6);
            assert_eq!(max_product(&vec![-2, -3]), 6);
            assert_eq!(max_product(&vec![-2, 3]), 3);
        }

        #[test]
        fn test_three_elements() {
            assert_eq!(max_product(&vec![2, 3, -2]), 6);
            assert_eq!(max_product(&vec![-2, 3, -4]), 24);
            assert_eq!(max_product(&vec![-2, 0, -1]), 0);
        }

        #[test]
        fn test_with_zeros() {
            assert_eq!(max_product(&vec![2, 0, 3]), 3);
            assert_eq!(max_product(&vec![0, 0, 0]), 0);
            assert_eq!(max_product(&vec![1, 0, -2]), 1);
        }

        #[test]
        fn test_all_negative() {
            assert_eq!(max_product(&vec![-1, -2, -3]), 6);
            assert_eq!(max_product(&vec![-1, -2, -3, -4]), 24);
            assert_eq!(max_product(&vec![-2, -3]), 6);
        }

        #[test]
        fn test_mixed_numbers() {
            assert_eq!(max_product(&vec![2, 3, -2, 4]), 6);
            assert_eq!(max_product(&vec![-2, 3, -4, 5, -2]), 120);
            assert_eq!(max_product(&vec![2, -5, -2, -4, 3]), 24);
        }

        #[test]
        fn test_alternating_signs() {
            assert_eq!(max_product(&vec![1, -2, 3, -4, 5]), 120);
            assert_eq!(max_product(&vec![-1, 2, -3, 4, -5]), 120);
        }

        #[test]
        fn test_sequence_with_different_max_positions() {
            assert_eq!(max_product(&vec![6, 2, -1, 1, 1]), 12);
            assert_eq!(max_product(&vec![1, 2, 6, 2, 1]), 24);
            assert_eq!(max_product(&vec![1, 1, -1, 2, 6]), 12);
        }
    }

    mod max_product_functional_tests {
        use super::*;

        #[test]
        fn test_both_implementations() {
            let test_cases = vec![
                (vec![], 0),
                (vec![1], 1),
                (vec![2, 3], 6),
                (vec![-2, 3, -4], 24),
                (vec![2, 3, -2, 4], 6),
                (vec![-2, 0, -1], 0),
            ];

            for (input, expected) in test_cases {
                assert_eq!(max_product_functional(&input), expected);
            }
        }
    }
}
