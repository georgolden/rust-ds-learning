#[derive(Debug)]
pub enum MatrixError {
    InvalidCreation {
        expected: usize,
        actual: usize,
    },
    DimensionMismatch {
        operation: &'static str,
        left_dims: (usize, usize),
        right_dims: (usize, usize),
    },
    IndexOutOfBounds {
        row: usize,
        col: usize,
        rows: usize,
        cols: usize,
    },
    ElementNotFound {
        el: f64,
    }
}

impl std::fmt::Display for MatrixError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MatrixError::InvalidCreation { expected, actual } => {
                write!(f, "Invalid dimensions: expected {} elements, got {}", 
                    expected, actual)
            }
            MatrixError::DimensionMismatch { operation, left_dims, right_dims } => {
                write!(f, "Cannot {} matrices: left matrix is {:?}, right matrix is {:?}",
                    operation, left_dims, right_dims)
            }
            MatrixError::IndexOutOfBounds { row, col, rows, cols } => {
                write!(f, "Index out of bounds: tried to access ({}, {}) in a {}x{} matrix",
                    row, col, rows, cols)
            }
            MatrixError::ElementNotFound { el } => {
                write!(f, "Element ({}) not found", el)
            }
        }
    }
}

impl std::error::Error for MatrixError {}

#[derive(Debug, Clone, PartialEq)]
pub struct Matrix {
    rows: usize,
    cols: usize,
    data: Vec<f64>,
}

impl Matrix {
    pub fn zeros(rows: usize, cols: usize) -> Self {
        Self {
            rows,
            cols,
            data: vec![0.0; rows * cols],
        }
    }

    pub fn from_vec(rows: usize, cols: usize, data: Vec<f64>) -> Result<Self, MatrixError> {
        let expected = rows * cols;
        if data.len() != expected {
            return Err(MatrixError::InvalidCreation {
                expected,
                actual: data.len(),
            });
        }
        Ok(Self { rows, cols, data })
    }

    #[inline]
    pub fn rows(&self) -> usize {
        self.rows
    }

    #[inline]
    pub fn cols(&self) -> usize {
        self.cols
    }

    pub fn get(&self, row: usize, col: usize) -> Result<f64, MatrixError> {
        if row >= self.rows || col >= self.cols {
            return Err(MatrixError::IndexOutOfBounds {
                row,
                col,
                rows: self.rows,
                cols: self.cols,
            });
        }
        Ok(self.data[row * self.cols + col])
    }

    pub fn set(&mut self, row: usize, col: usize, value: f64) -> Result<(), MatrixError> {
        if row >= self.rows || col >= self.cols {
            return Err(MatrixError::IndexOutOfBounds {
                row,
                col,
                rows: self.rows,
                cols: self.cols,
            });
        }
        self.data[row * self.cols + col] = value;
        Ok(())
    }

    pub fn transpose(&self) -> Self {
        let mut result = Self::zeros(self.cols, self.rows);
        for i in 0..self.rows {
            for j in 0..self.cols {
                result.data[j * self.rows + i] = self.data[i * self.cols + j];
            }
        }
        result
    }

    pub fn find_position(&self, val: f64) -> Result<(usize, usize), MatrixError> {
        for i in 0..self.rows {
            for j in 0..self.cols {
                if self.data[i*self.cols + j] == val {
                    return Ok((i, j));
                }
            }
        }
        return Err(MatrixError::ElementNotFound { 
            el: val
        });
    }

}

impl std::ops::Add for &Matrix {
    type Output = Result<Matrix, MatrixError>;

    fn add(self, rhs: &Matrix) -> Self::Output {
        if self.rows != rhs.rows || self.cols != rhs.cols {
            return Err(MatrixError::DimensionMismatch {
                operation: "addition",
                left_dims: (self.rows, self.cols),
                right_dims: (rhs.rows, rhs.cols),
            });
        }

        let mut result = Matrix::zeros(self.rows, self.cols);
        for i in 0..self.data.len() {
            result.data[i] = self.data[i] + rhs.data[i];
        }
        Ok(result)
    }
}

impl std::ops::Mul for &Matrix {
    type Output = Result<Matrix, MatrixError>;

    fn mul(self, rhs: &Matrix) -> Self::Output {
        if self.cols != rhs.rows {
            return Err(MatrixError::DimensionMismatch {
                operation: "multiplication",
                left_dims: (self.rows, self.cols),
                right_dims: (rhs.rows, rhs.cols),
            });
        }

        let mut result = Matrix::zeros(self.rows, rhs.cols);
        for i in 0..self.rows {
            for j in 0..rhs.cols {
                let mut sum = 0.0;
                for k in 0..self.cols {
                    sum += self.data[i * self.cols + k] * rhs.data[k * rhs.cols + j];
                }
                result.data[i * rhs.cols + j] = sum;
            }
        }
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EPSILON: f64 = 1e-10;

    fn approx_eq(a: f64, b: f64, epsilon: f64) -> bool {
        (a - b).abs() < epsilon
    }

    // Helper function to compare vectors of f64 with approximate equality
    fn vec_approx_eq(a: &[f64], b: &[f64]) -> bool {
        if a.len() != b.len() {
            return false;
        }
        a.iter().zip(b.iter()).all(|(&x, &y)| approx_eq(x, y, EPSILON))
    }

    #[test]
    fn test_creation() {
        let matrix = Matrix::zeros(2, 3);
        assert_eq!(matrix.rows(), 2);
        assert_eq!(matrix.cols(), 3);
        assert!(matrix.data.iter().all(|&x| approx_eq(x, 0.0, EPSILON)));
    }

    #[test]
    fn test_from_vec() {
        let data = vec![1.0, 2.0, 3.0, 4.0];
        let matrix = Matrix::from_vec(2, 2, data.clone()).unwrap();
        assert!(vec_approx_eq(&matrix.data, &data));

        let result = Matrix::from_vec(2, 3, data);
        assert!(matches!(
            result,
            Err(MatrixError::InvalidCreation { expected: 6, actual: 4 })
        ));
    }

    #[test]
    fn test_get_set() {
        let mut matrix = Matrix::zeros(2, 2);
        assert!(matrix.set(0, 1, 5.0).is_ok());
        assert!(approx_eq(matrix.get(0, 1).unwrap(), 5.0, EPSILON));

        assert!(matches!(
            matrix.get(2, 0),
            Err(MatrixError::IndexOutOfBounds { row: 2, col: 0, .. })
        ));
    }

    #[test]
    fn test_add() {
        let m1 = Matrix::from_vec(2, 2, vec![1.0, 2.0, 3.0, 4.0]).unwrap();
        let m2 = Matrix::from_vec(2, 2, vec![5.0, 6.0, 7.0, 8.0]).unwrap();
        let sum = (&m1 + &m2).unwrap();
        assert!(vec_approx_eq(&sum.data, &[6.0, 8.0, 10.0, 12.0]));

        let m3 = Matrix::zeros(2, 3);
        assert!(matches!(
            &m1 + &m3,
            Err(MatrixError::DimensionMismatch { operation: "addition", .. })
        ));
    }

    #[test]
    fn test_mul() {
        let m1 = Matrix::from_vec(2, 3, vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0]).unwrap();
        let m2 = Matrix::from_vec(3, 2, vec![7.0, 8.0, 9.0, 10.0, 11.0, 12.0]).unwrap();
        let product = (&m1 * &m2).unwrap();
        assert!(vec_approx_eq(&product.data, &[58.0, 64.0, 139.0, 154.0]));

        let m3 = Matrix::zeros(2, 2);
        assert!(matches!(
            &m1 * &m3,
            Err(MatrixError::DimensionMismatch { operation: "multiplication", .. })
        ));
    }

    #[test]
    fn test_transpose() {
        let matrix = Matrix::from_vec(2, 3, vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0]).unwrap();
        let transposed = matrix.transpose();
        assert_eq!(transposed.rows(), 3);
        assert_eq!(transposed.cols(), 2);
        assert_eq!(transposed.data, vec![1.0, 4.0, 2.0, 5.0, 3.0, 6.0]);
    }

    #[test]
    fn test_find_position() {
        // Test case 1: Finding element in a 1x1 matrix
        let matrix = Matrix::from_vec(1, 1, vec![5.0]).unwrap();
        assert_eq!(matrix.find_position(5.0).unwrap(), (0, 0));

        // Test case 2: Finding element in first row
        let matrix = Matrix::from_vec(2, 3, vec![1.0, 2.0, 3.0, 
                                                4.0, 5.0, 6.0]).unwrap();
        assert_eq!(matrix.find_position(2.0).unwrap(), (0, 1));

        // Test case 3: Finding element in last row
        assert_eq!(matrix.find_position(5.0).unwrap(), (1, 1));

        // Test case 4: Finding element that doesn't exist
        assert!(matches!(
            matrix.find_position(7.0),
            Err(MatrixError::ElementNotFound { el: 7.0 })
        ));

        // Test case 5: Finding element in a matrix with duplicate values (should return first occurrence)
        let matrix = Matrix::from_vec(2, 2, vec![1.0, 2.0, 
                                                2.0, 3.0]).unwrap();
        assert_eq!(matrix.find_position(2.0).unwrap(), (0, 1));

        // Test case 6: Finding element in empty matrix
        let matrix = Matrix::zeros(0, 0);
        assert!(matches!(
            matrix.find_position(1.0),
            Err(MatrixError::ElementNotFound { el: 1.0 })
        ));

        // Test case 7: Finding with floating point comparison
        let matrix = Matrix::from_vec(2, 2, vec![1.1, 1.2, 
                                                1.3, 1.4]).unwrap();
        assert_eq!(matrix.find_position(1.2).unwrap(), (0, 1));
    }

    #[test]
    fn test_find_position_with_approximate_values() {
        // This test specifically checks floating point comparison issues
        let matrix = Matrix::from_vec(2, 2, vec![
            0.1 + 0.2,           0.4, 
            0.5,                 0.6
        ]).unwrap();

        // 0.1 + 0.2 is not exactly equal to 0.3 in floating point arithmetic
        // This test will fail with direct comparison
        // You might want to modify find_position to use approx_eq if this is important
        // for your use case
        assert!(matches!(
            matrix.find_position(0.3),
            Err(MatrixError::ElementNotFound { el: 0.3 })
        ));
    }
}
