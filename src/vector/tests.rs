// src/vector/tests.rs
#[cfg(test)]
mod tests {
    use crate::vector::{merge_intervals, sliding_window_maximum};

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
}
