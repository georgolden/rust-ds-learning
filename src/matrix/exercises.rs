use thiserror::Error;
use crate::matrix::matrix::{Matrix, MatrixError};

#[derive(Error, Debug)]
pub enum SearchError {
    #[error("Matrix must be square, got dimensions {rows}x{cols}")]
    NotSquareMatrix {
        rows: usize,
        cols: usize,
    },
    #[error("Element {el} not found in sorted matrix")]
    ElementNotFound {
        el: f64,
    },
    #[error(transparent)]
    Matrix(#[from] MatrixError),
}

/// Finds position of a value in a sorted square matrix (Young tableau).
/// A Young tableau is a square matrix where:
/// 1. Elements are sorted in ascending order from left to right in each row
/// 2. Elements are sorted in ascending order from top to bottom in each column
/// 
/// Example of a valid sorted square matrix:
/// ```text
/// [1.0, 2.0, 3.0]
/// [4.0, 5.0, 6.0]
/// [7.0, 8.0, 9.0]
/// ```
pub fn find_postition_sorted_square_matrix(m: &Matrix, val: f64) -> Result<(usize, usize), SearchError> {
    if m.rows != m.cols {
        return Err(SearchError::NotSquareMatrix {
            rows: m.rows,
            cols: m.cols,
        });
    }

    let n = m.rows;
    if n == 0 {
        return Err(SearchError::ElementNotFound { el: val });
    }

    let mut i: usize = 0;
    let mut j: usize = n - 1;

    while i < n && j < n {
        let current = m.get(i, j)?;
        
        if current == val {
            return Ok((i, j));
        }
        
        if current > val {
            if j == 0 { break; }  // prevent underflow
            j -= 1;
        } else {
            i += 1;
        }
    }

    Err(SearchError::ElementNotFound { el: val })
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_find_position_sorted_typical() {
        // Test case for a typical 3x3 sorted matrix
        let matrix = Matrix::from_vec(3, 3, vec![
            1.0, 2.0, 3.0,
            4.0, 5.0, 6.0,
            7.0, 8.0, 9.0
        ]).unwrap();

        assert_eq!(find_postition_sorted_square_matrix(&matrix, 5.0).unwrap(), (1, 1));
        assert_eq!(find_postition_sorted_square_matrix(&matrix, 1.0).unwrap(), (0, 0)); // First element
        assert_eq!(find_postition_sorted_square_matrix(&matrix, 9.0).unwrap(), (2, 2)); // Last element
    }

    #[test]
    fn test_find_position_sorted_edge_cases() {
        // Test 1x1 matrix
        let matrix = Matrix::from_vec(1, 1, vec![1.0]).unwrap();
        assert_eq!(find_postition_sorted_square_matrix(&matrix, 1.0).unwrap(), (0, 0));

        // Test empty matrix
        let matrix = Matrix::zeros(0, 0);
        assert!(matches!(
            find_postition_sorted_square_matrix(&matrix, 1.0),
            Err(SearchError::ElementNotFound { el: 1.0 })
        ));

        // Test 2x2 matrix corner cases
        let matrix = Matrix::from_vec(2, 2, vec![
            1.0, 2.0,
            3.0, 4.0
        ]).unwrap();
        assert_eq!(find_postition_sorted_square_matrix(&matrix, 1.0).unwrap(), (0, 0)); // top-left
        assert_eq!(find_postition_sorted_square_matrix(&matrix, 4.0).unwrap(), (1, 1)); // bottom-right
    }

    #[test]
    fn test_non_square_matrix_error() {
        let matrix = Matrix::from_vec(2, 3, vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0]).unwrap();
        
        assert!(matches!(
            find_postition_sorted_square_matrix(&matrix, 5.0),
            Err(SearchError::NotSquareMatrix { rows: 2, cols: 3 })
        ));
    }

    #[test]
    fn test_find_position_sorted_not_found() {
        let matrix = Matrix::from_vec(2, 2, vec![
            1.0, 2.0,
            3.0, 4.0
        ]).unwrap();

        // Test value smaller than minimum
        assert!(matches!(
            find_postition_sorted_square_matrix(&matrix, 0.0),
            Err(SearchError::ElementNotFound { el: 0.0 })
        ));

        // Test value larger than maximum
        assert!(matches!(
            find_postition_sorted_square_matrix(&matrix, 5.0),
            Err(SearchError::ElementNotFound { el: 5.0 })
        ));

        // Test value between existing elements
        assert!(matches!(
            find_postition_sorted_square_matrix(&matrix, 1.5),
            Err(SearchError::ElementNotFound { el: 1.5 })
        ));
    }

    #[test]
    fn test_find_position_sorted_floating_point() {
        let matrix = Matrix::from_vec(2, 2, vec![
            1.1, 1.2,
            1.3, 1.4
        ]).unwrap();

        assert_eq!(find_postition_sorted_square_matrix(&matrix, 1.2).unwrap(), (0, 1));
        
        assert!(matches!(
            find_postition_sorted_square_matrix(&matrix, 1.25),
            Err(SearchError::ElementNotFound { el: 1.25 })
        ));
    }
}
