use std::ops::{Index, Mul};
use approx::{RelativeEq, AbsDiffEq};

use crate::tuple::Tuple;

use super::matrix3d::Matrix3D;

const MATRIX_SIZE: usize = 4;


#[derive(Debug, PartialEq, PartialOrd)]
pub struct Matrix4D {
    pub data: [[f32; MATRIX_SIZE]; MATRIX_SIZE]
}

impl Matrix4D {
    pub fn new(data: [[f32; MATRIX_SIZE]; MATRIX_SIZE]) -> Self {
        Matrix4D { data: data }
    }

    // construct identity matrix
    pub fn identity() -> Self {
        let mut tmp: [[f32; MATRIX_SIZE]; MATRIX_SIZE] = [[0.0; MATRIX_SIZE]; MATRIX_SIZE];
        for row_idx in 0..MATRIX_SIZE {
            for col_idx in 0..MATRIX_SIZE {
                if row_idx == col_idx {
                    tmp[row_idx][col_idx] = 1.0;
                }
            }
        }
        Matrix4D { data: tmp }
    }

    pub fn transpose(&self) -> Matrix4D {
        let mut tmp: [[f32; MATRIX_SIZE]; MATRIX_SIZE] = [[0.0; MATRIX_SIZE]; MATRIX_SIZE];
        for (row_idx, row) in self.data.iter().enumerate() {
            for (col_idx, elem) in row.iter().enumerate() {
                tmp[col_idx][row_idx] = *elem;
            }
        }

        Matrix4D { data: tmp }
    }

    pub fn minor(&self, row_idx: usize, col_idx: usize) -> f32 {
        let submatrix = self.submatrix(row_idx, col_idx);
        submatrix.det()
    }

    pub fn cofactor(&self, row_idx: usize, col_idx: usize) -> f32 {
        let minor_value = self.minor(row_idx, col_idx);
        if (row_idx + col_idx) % 2 == 0 {
            minor_value
        } else {
            -minor_value
        }
    }

    pub fn det(&self) -> f32 {    
        let mut result: f32 = 0.0;
        for i in 0..MATRIX_SIZE {
            result += self.cofactor(0, i) * self[(0, i)];
        }
        result
    }

    pub fn is_invertible(&self) -> bool {
        self.det() != 0.0
    }

    pub fn inverse(&self) -> Option<Matrix4D> {
        if !self.is_invertible() {
            return None
        }

        let det = self.det();
        let mut tmp: [[f32; MATRIX_SIZE]; MATRIX_SIZE] = [[0.0; MATRIX_SIZE]; MATRIX_SIZE];
        for row_idx in 0..MATRIX_SIZE {
            for col_idx in 0..MATRIX_SIZE {
                let c = self.cofactor(row_idx, col_idx);
                tmp[col_idx][row_idx] = c / det;
            }
        }

        Some(Matrix4D { data: tmp })
    }
    
    pub fn submatrix(&self, row_idx_skip: usize, col_idx_skip: usize) -> Matrix3D {
        let mut tmp: [[f32; MATRIX_SIZE - 1]; MATRIX_SIZE - 1] = [[0.0; MATRIX_SIZE - 1]; MATRIX_SIZE - 1];
    
        let mut row_idx: usize = 0;
        let mut col_idx: usize = 0;
        for m in 0..MATRIX_SIZE {
            if m == row_idx_skip {
                continue;
            }
            for c in 0..MATRIX_SIZE {
                if c == col_idx_skip {
                    continue;
                }
                tmp[row_idx][col_idx] = self[(m, c)];
                col_idx += 1
            }
            row_idx += 1;
            col_idx = 0;
        }

        Matrix3D { data: tmp }
    }
}

impl Index<(usize, usize)> for Matrix4D {
    type Output = f32;

    fn index(&self, idx_pair: (usize, usize)) -> &Self::Output {
        &self.data[idx_pair.0][idx_pair.1]
    }
}

// Implement multiplication by another 4D matrix
impl Mul<Matrix4D> for Matrix4D {
    type Output = Matrix4D;

    fn mul(self, rhs: Matrix4D) -> Self::Output {
        let mut tmp: [[f32; MATRIX_SIZE]; MATRIX_SIZE] = [[0.0; MATRIX_SIZE]; MATRIX_SIZE];
        for (row_idx, row) in self.data.iter().enumerate() {
            for (col_idx, col) in rhs.transpose().data.iter().enumerate() {
                for (r, c) in row.iter().zip(col.iter()) {
                    tmp[row_idx][col_idx] += r * c;
                }
            }
        }
        Matrix4D { data: tmp }
    }
}

// Implement multiplication by tuple of 4 elements
impl Mul<Tuple> for Matrix4D {
    type Output = Tuple;

    fn mul(self, rhs: Tuple) -> Self::Output {
        let mut tmp: [f32; 4] = [0.0; 4];
        for (row_idx, row) in self.data.iter().enumerate() {
            for (r, c) in row.iter().zip(rhs.into_iter()) {
                tmp[row_idx] += r * c;
            }
        }
        Tuple::from_array(tmp)
    }
}

impl AbsDiffEq for Matrix4D {
    type Epsilon = f32;

    fn default_epsilon() -> Self::Epsilon {
        f32::default_epsilon()
    }

    fn abs_diff_eq(&self, other: &Self, epsilon: f32) -> bool {
        let mut result = true;
        for row_idx in 0..MATRIX_SIZE {
            for col_idx in 0..MATRIX_SIZE {
                result = result && f32::abs_diff_eq(&self[(row_idx, col_idx)], &other[(row_idx, col_idx)], epsilon);
            }
        }

        result
    }
}

impl RelativeEq for Matrix4D {

    fn default_max_relative() -> f32 {
        f32::default_max_relative()
    }

    fn relative_eq(&self, other: &Self, epsilon: f32, max_relative: f32) -> bool {
        let mut result = true;
        for row_idx in 0..MATRIX_SIZE {
            for col_idx in 0..MATRIX_SIZE {
                result = result && f32::relative_eq(&self[(row_idx, col_idx)], &other[(row_idx, col_idx)], epsilon, max_relative);
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;

    use super::Matrix4D;

    #[test]
    fn indexing() {
        let m = Matrix4D::new(
            [[1.0, 2.0, 3.0, 4.0], 
            [5.5, 6.5, 7.5, 8.5],
            [9.0, 10.0, 11.0, 12.0],
            [13.5, 14.5, 15.5, 16.5]]
        );

        assert_eq!(m[(0, 0)], 1.0);
        assert_eq!(m[(1, 0)], 5.5);
        assert_eq!(m[(2, 0)], 9.0);
        assert_eq!(m[(3, 0)], 13.5);
        assert_eq!(m[(0, 1)], 2.0);
        assert_eq!(m[(0, 2)], 3.0);
        assert_eq!(m[(0, 3)], 4.0);
        assert_eq!(m[(1, 1)], 6.5);
        assert_eq!(m[(1, 2)], 7.5);
        assert_eq!(m[(2, 2)], 11.0);
        assert_eq!(m[(3, 0)], 13.5);
        assert_eq!(m[(3, 2)], 15.5);
        assert_eq!(m[(3, 3)], 16.5);
    }

    #[test]
    fn equality() {
        let m1 = Matrix4D::new(
            [[1.0, 2.0, 3.0, 4.0], 
            [5.5, 6.5, 7.5, 8.5],
            [9.0, 10.0, 11.0, 12.0],
            [13.5, 14.5, 15.5, 16.5]]
        );

        let m2 = Matrix4D::new(
            [[1.0, 2.0, 3.0, 4.0], 
            [5.5, 6.5, 7.5, 8.5],
            [9.0, 10.0, 11.0, 12.0],
            [13.5, 14.5, 15.5, 16.5]]
        );

        assert_eq!(m1, m2);

        let m3= Matrix4D::new(
            [[1.0, 2.0, 3.0, 4.0], 
            [5.5, 6.5, 7.5, 8.5],
            [9.0, 10.0, 11.0, 12.0],
            [13.5, 14.5, 15.5, 16.0]]
        );

        assert_ne!(m3, m2);
        assert_ne!(m3, m1);
    }

    #[test]
    fn transpose() {
        let m = Matrix4D::new(
            [[1.0, 2.0, 3.0, 4.0], 
            [5.5, 6.5, 7.5, 8.5],
            [9.0, 10.0, 11.0, 12.0],
            [13.5, 14.5, 15.5, 16.5]]
        );

        let m_t = m.transpose();

        assert_eq!(m_t[(0, 0)], 1.0);
        assert_eq!(m_t[(1, 0)], 2.0);
        assert_eq!(m_t[(0, 1)], 5.5);
        assert_eq!(m_t[(1, 1)], 6.5);
        assert_eq!(m_t[(2, 2)], 11.0);
        assert_eq!(m_t[(2, 0)], 3.0);
        assert_eq!(m_t[(0, 2)], 9.0);
        assert_eq!(m_t[(3, 3)], 16.5);
    }

    #[test]
    fn multiply() {
        let m1 = Matrix4D::new(
            [[1.0, 2.0, 3.0, 4.0], 
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 8.0, 7.0, 6.0],
            [5.0, 4.0, 3.0, 2.0]]
        );

        let m2 = Matrix4D::new(
            [[-2.0, 1.0, 2.0, 3.0], 
            [3.0, 2.0, 1.0, -1.0],
            [4.0, 3.0, 6.0, 5.0],
            [1.0, 2.0, 7.0, 8.0]]
        );

        let m3 = Matrix4D::new(
            [[20.0, 22.0, 50.0, 48.0], 
            [44.0, 54.0, 114.0, 108.0],
            [40.0, 58.0, 110.0, 102.0],
            [16.0, 26.0, 46.0, 42.0]]
        );

        assert_eq!(&(m1 * m2), &m3);
    }

    #[test]
    fn multiply_by_tuple() {
        use crate::tuple::Tuple;

        let m1 = Matrix4D::new(
            [[1.0, 2.0, 3.0, 4.0], 
            [2.0, 4.0, 4.0, 2.0],
            [8.0, 6.0, 4.0, 1.0],
            [0.0, 0.0, 0.0, 1.0]]
        );

        let t1 = Tuple::new(1.0, 2.0, 3.0, 1.0);

        let t = Tuple::new(18.0, 24.0, 33.0, 1.0);

        assert_eq!(&(m1 * t1), &t);
    }

    #[test]
    fn multiply_by_identity() {
        use crate::tuple::Tuple;

        let m1 = Matrix4D::new(
            [[1.0, 2.0, 3.0, 4.0], 
            [2.0, 4.0, 4.0, 2.0],
            [8.0, 6.0, 4.0, 1.0],
            [0.0, 0.0, 0.0, 1.0]]
        );
        
        let m_id = Matrix4D::identity();

        let m = Matrix4D::new(
            [[1.0, 2.0, 3.0, 4.0], 
            [2.0, 4.0, 4.0, 2.0],
            [8.0, 6.0, 4.0, 1.0],
            [0.0, 0.0, 0.0, 1.0]]
        );

        assert_eq!(&(m1 * m_id), &m);

        let m_id = Matrix4D::identity();

        let t1 = Tuple::new(1.0, 2.0, 3.0, 1.0);

        let t = Tuple::new(1.0, 2.0, 3.0, 1.0);

        assert_eq!(&(m_id * t1), &t);
    }

    #[test]
    fn submatrix() {
        use super::Matrix3D;

        let m1 = Matrix4D::new(
            [[-6.0, 1.0, 1.0, 6.0], 
            [-8.0, 5.0, 8.0, 6.0],
            [-1.0, 0.0, 8.0, 2.0],
            [-7.0, 1.0, -1.0, 1.0]]
        );

        let m2 = m1.submatrix(2, 1);

        let m = Matrix3D::new(
            [[-6.0, 1.0, 6.0],
            [-8.0, 8.0, 6.0],
            [-7.0, -1.0, 1.0]]
        );

        assert_eq!(&m2, &m);
    }

    #[test]
    fn minor() {

        let m1 = Matrix4D::new(
            [[-2.0, -8.0, 3.0, 5.0],
            [-3.0, 1.0, 7.0, 3.0],
            [1.0, 2.0, -9.0, 6.0],
            [-6.0, 7.0, 7.0, -9.0]]
        );

        assert_eq!(m1.minor(0, 0), 690.0);
        assert_eq!(m1.minor(0, 1), -447.0)
    }

    #[test]
    fn cofactor() {

        let m1 = Matrix4D::new(
            [[-2.0, -8.0, 3.0, 5.0],
            [-3.0, 1.0, 7.0, 3.0],
            [1.0, 2.0, -9.0, 6.0],
            [-6.0, 7.0, 7.0, -9.0]]
        );

        assert_eq!(m1.cofactor(0, 0), 690.0);
        assert_eq!(m1.cofactor(0, 1), 447.0);
        assert_eq!(m1.cofactor(0, 2), 210.0);
        assert_eq!(m1.cofactor(0, 3), 51.0);
    }

    #[test]
    fn determinant() {

        let m1 = Matrix4D::new(
            [[-2.0, -8.0, 3.0, 5.0],
            [-3.0, 1.0, 7.0, 3.0],
            [1.0, 2.0, -9.0, 6.0],
            [-6.0, 7.0, 7.0, -9.0]]
        );

        assert_eq!(m1.det(), -4071.0);
    }

    #[test]
    fn invertibility() {
        let m1 = Matrix4D::new(
            [[-2.0, -8.0, 3.0, 5.0],
            [-3.0, 1.0, 7.0, 3.0],
            [1.0, 2.0, -9.0, 6.0],
            [-6.0, 7.0, 7.0, -9.0]]
        );

        assert!(m1.is_invertible());

        let m2 = Matrix4D::new(
            [[-2.0, -8.0, 3.0, 5.0],
            [-3.0, 1.0, 7.0, 3.0],
            [1.0, 2.0, -9.0, 6.0],
            [0.0, 0.0, 0.0, 0.0]]
        );

        assert!(!m2.is_invertible());
    }

    #[test]
    fn inverse() {
        let m1 = Matrix4D::new(
            [[8.0, -5.0, 9.0, 2.0],
            [7.0, 5.0, 6.0, 1.0],
            [-6.0, 0.0, 9.0, 6.0],
            [-3.0, 0.0, -9.0, -4.0]]
        );

        let m1_inv = Matrix4D::new(
            [[-0.15384616, -0.15384616, -0.2820513, -0.53846157], 
            [-0.07692308, 0.12307692, 0.025641026, 0.03076923], 
            [0.35897437, 0.35897437, 0.43589744, 0.9230769], 
            [-0.6923077, -0.6923077, -0.7692308, -1.9230769]]
        );

        assert_relative_eq!(&m1.inverse().unwrap(), &m1_inv);

        let m1 = Matrix4D::new(
            [[9.0, 3.0, 0.0, 9.0],
            [-5.0, -2.0, -6.0, -3.0],
            [-4.0, 9.0, 6.0, 4.0],
            [-7.0, 6.0, 6.0, 2.0]]
        );

        let m1_inv = Matrix4D::new(
            [[-0.04074074, -0.07777778, 0.14444445, -0.22222222], 
            [-0.07777778, 0.033333335, 0.36666667, -0.33333334], 
            [-0.029012345, -0.14629629, -0.10925926, 0.12962963], 
            [0.17777778, 0.06666667, -0.26666668, 0.33333334]]
        );

        assert_eq!(&m1.inverse().unwrap(), &m1_inv);
    }

}