//! Vector-based Algorithm Exercises
//!
//! This module contains challenging algorithmic problems that use vectors
//! as their primary data structure.

/// # Sliding Window Maximum
///
/// ## Problem Statement
/// Given an array of integers and a window size k, find the maximum element
/// in each sliding window as it moves from left to right through the array.
///
/// ## Example
/// ```
/// use rust_ds_learning::vector::sliding_window_maximum;
/// let nums = vec![1, 3, -1, -3, 5, 3, 6, 7];
/// let result = sliding_window_maximum(&nums, 3);
/// assert_eq!(result, vec![3, 3, 5, 5, 6, 7]);
/// ```
///
/// ## Approach
/// Uses a deque (double-ended queue) to maintain indices of potential maximum elements.
/// The deque stores elements in decreasing order, with the front being the current maximum.
///
/// ## Complexity
/// - Time: O(n) where n is the length of the input array
/// - Space: O(k) where k is the window size
///
/// ## Key Points
/// 1. Maintains a monotonic decreasing queue
/// 2. Stores indices instead of values to handle window boundaries
/// 3. Efficiently removes elements outside the current window
///
/// ## Common Pitfalls
/// 1. Not handling empty input array
/// 2. Incorrect window size validation
/// 3. Not removing elements outside current window
/// 4. Not maintaining monotonic property of deque
pub fn sliding_window_maximum(nums: &[i32], window_size: usize) -> Vec<i32> {
    if nums.is_empty() || window_size == 0 {
        return vec![];
    }
    if window_size == 1 {
        return nums.to_vec();
    }

    let mut result = Vec::with_capacity(nums.len() - window_size + 1);
    let mut deque = std::collections::VecDeque::new();

    // Process first window
    for i in 0..window_size {
        // Remove all elements smaller than current element from back
        // This maintains monotonic decreasing property
        while let Some(&back) = deque.back() {
            if nums[back] <= nums[i] {
                deque.pop_back();
            } else {
                break;
            }
        }
        deque.push_back(i);
    }

    // Process remaining elements
    for i in window_size..nums.len() {
        // Add maximum from current window
        result.push(nums[deque[0]]);

        // Remove elements outside current window
        while let Some(&front) = deque.front() {
            if front <= i - window_size {
                deque.pop_front();
            } else {
                break;
            }
        }

        // Remove elements smaller than current element
        while let Some(&back) = deque.back() {
            if nums[back] <= nums[i] {
                deque.pop_back();
            } else {
                break;
            }
        }
        deque.push_back(i);
    }

    // Add maximum from last window
    if !deque.is_empty() {
        result.push(nums[deque[0]]);
    }

    result
}

/// # Merge Intervals
///
/// ## Problem Statement
/// Given a collection of intervals, merge all overlapping intervals into a
/// minimal set of non-overlapping intervals.
///
/// ## Example
/// ```
/// use rust_ds_learning::vector::merge_intervals;
/// let intervals = vec![(1, 3), (2, 6), (8, 10), (15, 18)];
/// let result = merge_intervals(&intervals);
/// assert_eq!(result, vec![(1, 6), (8, 10), (15, 18)]);
/// ```
///
/// ## Approach
/// 1. Sort intervals by start time
/// 2. Iterate through sorted intervals, merging when overlap is found
/// 3. An overlap occurs when current interval's start ≤ previous interval's end
///
/// ## Complexity
/// - Time: O(n log n) due to sorting
/// - Space: O(n) for storing result
///
/// ## Key Points
/// 1. Sorting is crucial for efficient merging
/// 2. Uses tuple comparison for clean implementation
/// 3. Handles various types of overlaps:
///    - Complete overlap: (1,5), (2,3)
///    - Partial overlap: (1,3), (2,4)
///    - Touch points: (1,2), (2,3)
///
/// ## Common Pitfalls
/// 1. Not handling empty input
/// 2. Incorrect overlap detection
/// 3. Not considering edge cases (single interval)
/// 4. Not handling negative intervals
pub fn merge_intervals(intervals: &[(i32, i32)]) -> Vec<(i32, i32)> {
    if intervals.is_empty() {
        return vec![];
    }

    let mut intervals = intervals.to_vec();
    // Sort by start time to ensure we can merge in one pass
    intervals.sort_unstable_by_key(|&(start, _)| start);

    let mut result = Vec::new();
    let mut current = intervals[0];

    for &interval in intervals.iter().skip(1) {
        if interval.0 <= current.1 {
            // Overlapping intervals - extend current interval
            current.1 = current.1.max(interval.1);
        } else {
            // Non-overlapping - add current to result and start new interval
            result.push(current);
            current = interval;
        }
    }
    // Don't forget to add the last interval
    result.push(current);

    result
}
