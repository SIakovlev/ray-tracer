use std::ops::{Index, Mul};
use super::matrix2d::Matrix2D;

const MATRIX_SIZE: usize = 3;


#[derive(Debug, PartialEq, PartialOrd)]
pub struct Matrix3D {
    pub data: [[f32; MATRIX_SIZE]; MATRIX_SIZE]
}

impl Matrix3D {
    pub fn new(data: [[f32; MATRIX_SIZE]; MATRIX_SIZE]) -> Self {
        Matrix3D { data: data }
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
        Matrix3D { data: tmp }
    }

    pub fn transpose(&self) -> Matrix3D {
        let mut tmp: [[f32; MATRIX_SIZE]; MATRIX_SIZE] = [[0.0; MATRIX_SIZE]; MATRIX_SIZE];
        for (row_idx, row) in self.data.iter().enumerate() {
            for (col_idx, elem) in row.iter().enumerate() {
                tmp[col_idx][row_idx] = *elem;
            }
        }

        Matrix3D { data: tmp }
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
    
    pub fn submatrix(&self, row_idx_skip: usize, col_idx_skip: usize) -> Matrix2D {

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

        Matrix2D { data: tmp }
    }
}

impl Index<(usize, usize)> for Matrix3D {
    type Output = f32;

    fn index(&self, idx_pair: (usize, usize)) -> &Self::Output {
        &self.data[idx_pair.0][idx_pair.1]
    }
}

// Implement multiplication by another 3D matrix
impl Mul<Matrix3D> for Matrix3D {
    type Output = Matrix3D;

    fn mul(self, rhs: Matrix3D) -> Self::Output {
        let mut tmp: [[f32; MATRIX_SIZE]; MATRIX_SIZE] = [[0.0; MATRIX_SIZE]; MATRIX_SIZE];

        for (row_idx, row) in self.data.iter().enumerate() {
            for (col_idx, col) in rhs.transpose().data.iter().enumerate() {
                for (r, c) in row.iter().zip(col.iter()) {
                    tmp[row_idx][col_idx] += r * c;
                }
            }
        }
        Matrix3D { data: tmp }
    }
}

#[cfg(test)]
mod tests {

    use super::Matrix3D;

    #[test]
    fn indexing() {
        let m = Matrix3D::new(
            [[-3.0, 5.0, 0.0],
            [1.0, -2.0, -7.0],
            [0.0, 1.0, 1.0]]
        );

        assert_eq!(m[(0, 0)], -3.0);
        assert_eq!(m[(1, 1)], -2.0);
        assert_eq!(m[(2, 2)], 1.0);
    }

    #[test]
    fn equality() {
        let m1 = Matrix3D::new(
            [[-3.0, 5.0, 0.0],
            [1.0, -2.0, -7.0],
            [0.0, 1.0, 1.0]]
        );

        let m2 = Matrix3D::new(
            [[-3.0, 5.0, 0.0],
            [1.0, -2.0, -7.0],
            [0.0, 1.0, 1.0]]
        );

        assert_eq!(m1, m2);

        let m3 = Matrix3D::new(
            [[-3.0, 5.0, 0.0],
            [1.0, -2.0, -7.0],
            [0.0, 1.0, -1.0]]
        );

        assert_ne!(m3, m2);
        assert_ne!(m3, m1);
    }

    #[test]
    fn transpose() {
        let m = Matrix3D::new(
            [[-3.0, 5.0, 0.0],
            [1.0, -2.0, -7.0],
            [0.0, 1.0, -1.0]]
        );

        let m_t = m.transpose();

        assert_eq!(m_t[(0, 0)], -3.0);
        assert_eq!(m_t[(1, 0)], 5.0);
        assert_eq!(m_t[(0, 1)], 1.0);
        assert_eq!(m_t[(1, 1)], -2.0);
        assert_eq!(m_t[(2, 2)], -1.0);
    }

    #[test]
    fn multiply() {
        // let m1 = Matrix::<4, 4>::new(
        //     [[1.0, 2.0, 3.0, 4.0], 
        //     [5.0, 6.0, 7.0, 8.0],
        //     [9.0, 8.0, 7.0, 6.0],
        //     [5.0, 4.0, 3.0, 2.0]]
        // );

        // let m2 = Matrix::<4, 4>::new(
        //     [[-2.0, 1.0, 2.0, 3.0], 
        //     [3.0, 2.0, 1.0, -1.0],
        //     [4.0, 3.0, 6.0, 5.0],
        //     [1.0, 2.0, 7.0, 8.0]]
        // );

        // let m3 = Matrix::<4, 4>::new(
        //     [[20.0, 22.0, 50.0, 48.0], 
        //     [44.0, 54.0, 114.0, 108.0],
        //     [40.0, 58.0, 110.0, 102.0],
        //     [16.0, 26.0, 46.0, 42.0]]
        // );

        // assert_eq!(&(m1 * m2), &m3);
    }

    #[test]
    fn multiply_by_identity() {

        let m1 = Matrix3D::new(
            [[-3.0, 5.0, 0.0],
            [1.0, -2.0, -7.0],
            [0.0, 1.0, -1.0]]
        );
        
        let m_id = Matrix3D::identity();

        let m = Matrix3D::new(
            [[-3.0, 5.0, 0.0],
            [1.0, -2.0, -7.0],
            [0.0, 1.0, -1.0]]
        );

        assert_eq!(&(m1 * m_id), &m);
    }

    #[test]
    fn submatrix() {
        use super::Matrix2D;

        let m1 = Matrix3D::new(
            [[1.0, 5.0, 0.0], 
            [-3.0, 2.0, 7.0],
            [0.0, 6.0, -3.0]]
        );

        let m2 = m1.submatrix(0, 2);

        let m = Matrix2D::new(
            [[-3.0, 2.0],
            [0.0, 6.0]]
        );

        assert_eq!(&m2, &m);
    }

    #[test]
    fn minor() {
        let m1 = Matrix3D::new(
            [[3.0, 5.0, 0.0], 
            [2.0, -1.0, -7.0],
            [6.0, -1.0, 5.0]]
        );

        assert_eq!(m1.minor(0, 0), -12.0);
        assert_eq!(m1.minor(1, 0), 25.0);
    }

    #[test]
    fn cofactor() {
        let m1 = Matrix3D::new(
            [[3.0, 5.0, 0.0], 
            [2.0, -1.0, -7.0],
            [6.0, -1.0, 5.0]]
        );

        assert_eq!(m1.minor(0, 0), -12.0);
        assert_eq!(m1.cofactor(0, 0), -12.0);
        assert_eq!(m1.cofactor(1, 0), -25.0);

        let m1 = Matrix3D::new(
            [[1.0, 2.0, 6.0], 
            [-5.0, 8.0, -4.0],
            [2.0, 6.0, 4.0]]
        );

        assert_eq!(m1.cofactor(0, 0), 56.0);
        assert_eq!(m1.cofactor(0, 1), 12.0);
        assert_eq!(m1.cofactor(0, 2), -46.0);
        assert_eq!(m1.det(), -196.0);
    }

}