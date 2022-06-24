use std::ops::{Index, Mul};


const MATRIX_SIZE: usize = 2;


#[derive(Debug, PartialEq, PartialOrd)]
pub struct Matrix2D {
    pub data: [[f32; MATRIX_SIZE]; MATRIX_SIZE]
}

impl Matrix2D {
    pub fn new(data: [[f32; MATRIX_SIZE]; MATRIX_SIZE]) -> Self {
        Matrix2D { data: data }
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
        Matrix2D { data: tmp }
    }

    pub fn transpose(&self) -> Matrix2D {
        let mut tmp: [[f32; MATRIX_SIZE]; MATRIX_SIZE] = [[0.0; MATRIX_SIZE]; MATRIX_SIZE];
        for (row_idx, row) in self.data.iter().enumerate() {
            for (col_idx, elem) in row.iter().enumerate() {
                tmp[col_idx][row_idx] = *elem;
            }
        }

        Matrix2D { data: tmp }
    }

    pub fn det(&self) -> f32 {
        self[(0, 0)] * self[(1, 1)] - self[(0, 1)] * self[(1, 0)]
    }
}

impl Index<(usize, usize)> for Matrix2D {
    type Output = f32;

    fn index(&self, idx_pair: (usize, usize)) -> &Self::Output {
        &self.data[idx_pair.0][idx_pair.1]
    }
}

// Implement multiplication by another MxN matrix
impl Mul<Matrix2D> for Matrix2D {
    type Output = Matrix2D;

    fn mul(self, rhs: Matrix2D) -> Self::Output {
        let mut tmp: [[f32; MATRIX_SIZE]; MATRIX_SIZE] = [[0.0; MATRIX_SIZE]; MATRIX_SIZE];
        for (row_idx, row) in self.data.iter().enumerate() {
            for (col_idx, col) in rhs.transpose().data.iter().enumerate() {
                for (r, c) in row.iter().zip(col.iter()) {
                    tmp[row_idx][col_idx] += r * c;
                }
            }
        }
        Matrix2D { data: tmp }
    }
}

#[cfg(test)]
mod tests {

    use super::Matrix2D;

    #[test]
    fn indexing() {

        let m = Matrix2D::new(
            [[-3.0, 5.0],
            [1.0, -2.0]]
        );
        assert_eq!(m[(0, 0)], -3.0);
        assert_eq!(m[(1, 0)], 1.0);
        assert_eq!(m[(0, 1)], 5.0);
        assert_eq!(m[(1, 1)], -2.0);
    }

    #[test]
    fn equality() {
        let m1 = Matrix2D::new(
            [[-3.0, 5.0],
            [1.0, -2.0]]
        );

        let m2 = Matrix2D::new(
            [[-3.0, 5.0],
            [1.0, -2.0]]
        );

        assert_eq!(m1, m2);

        let m3 = Matrix2D::new(
            [[3.0, 5.0],
            [1.0, 2.0]]
        );

        assert_ne!(m3, m2);
        assert_ne!(m3, m1);
    }

    #[test]
    fn transpose() {
        let m = Matrix2D::new(
            [[-3.0, 5.0],
            [1.0, -2.0]]
        );

        let m_t = m.transpose();

        assert_eq!(m_t[(0, 0)], -3.0);
        assert_eq!(m_t[(1, 0)], 5.0);
        assert_eq!(m_t[(0, 1)], 1.0);
        assert_eq!(m_t[(1, 1)], -2.0);
    }

    #[test]
    fn multiply() {
        let m1 = Matrix2D::new(
            [[-3.0, 5.0],
            [1.0, -2.0]]
        );

        let m2 = Matrix2D::new(
            [[-1.0, 2.0],
            [1.0, -2.0]]
        );

        let m3 = Matrix2D::new(
            [[8.0, -16.0],
            [-3.0, 6.0]]
        );

        assert_eq!(&(m1 * m2), &m3);
    }
}