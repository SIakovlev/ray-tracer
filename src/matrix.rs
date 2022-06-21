use std::ops::{Index, Mul};


#[derive(Debug, PartialEq, PartialOrd)]
struct Matrix<const M: usize, const N: usize> {
    data: [[f32; N]; M]
}

impl<const M: usize, const N: usize> Matrix<M, N> {
    fn new(data: [[f32; N]; M]) -> Self {
        Matrix { data: data }
    }

    fn transpose(&self) -> Matrix<N, M> {
        let mut tmp: [[f32; M]; N] = [[0.0; M]; N];
        for (row_idx, row) in self.data.iter().enumerate() {
            for (col_idx, elem) in row.iter().enumerate() {
                tmp[col_idx][row_idx] = *elem;
            }
        }

        Matrix { data: tmp }
    }
}

impl<const M: usize, const N: usize> Index<(usize, usize)> for Matrix<M, N> {
    type Output = f32;

    fn index(&self, idx_pair: (usize, usize)) -> &Self::Output {
        &self.data[idx_pair.0][idx_pair.1]
    }
}

impl<const M: usize, const N: usize> Mul<Matrix<N, M>> for Matrix<M, N> {
    type Output = Matrix<N, N>;

    fn mul(self, rhs: Matrix<N, M>) -> Self::Output {
        let mut tmp: [[f32; N]; N] = [[0.0; N]; N];
        for (row_idx, row) in self.data.iter().enumerate() {
            for (col_idx, col) in rhs.transpose().data.iter().enumerate() {
                for (r, c) in row.iter().zip(col.iter()) {
                    tmp[row_idx][col_idx] += r * c;
                }
            }
        }
        Matrix { data: tmp }
    }
}

#[cfg(test)]
mod tests {

    use super::Matrix;

    #[test]
    fn indexing() {

        let m = Matrix::<2, 2>::new(
            [[-3.0, 5.0],
            [1.0, -2.0]]
        );
        assert_eq!(m[(0, 0)], -3.0);
        assert_eq!(m[(1, 0)], 1.0);
        assert_eq!(m[(0, 1)], 5.0);
        assert_eq!(m[(1, 1)], -2.0);

        let m = Matrix::<3, 3>::new(
            [[-3.0, 5.0, 0.0],
            [1.0, -2.0, -7.0],
            [0.0, 1.0, 1.0]]
        );

        assert_eq!(m[(0, 0)], -3.0);
        assert_eq!(m[(1, 1)], -2.0);
        assert_eq!(m[(2, 2)], 1.0);

        let m = Matrix::<4, 4>::new(
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
        let m1 = Matrix::<4, 4>::new(
            [[1.0, 2.0, 3.0, 4.0], 
            [5.5, 6.5, 7.5, 8.5],
            [9.0, 10.0, 11.0, 12.0],
            [13.5, 14.5, 15.5, 16.5]]
        );

        let m2 = Matrix::<4, 4>::new(
            [[1.0, 2.0, 3.0, 4.0], 
            [5.5, 6.5, 7.5, 8.5],
            [9.0, 10.0, 11.0, 12.0],
            [13.5, 14.5, 15.5, 16.5]]
        );

        assert_eq!(m1, m2);

        let m3= Matrix::<4, 4>::new(
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
        let m = Matrix::<2, 2>::new(
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
        let m1 = Matrix::<4, 4>::new(
            [[1.0, 2.0, 3.0, 4.0], 
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 8.0, 7.0, 6.0],
            [5.0, 4.0, 3.0, 2.0]]
        );

        let m2 = Matrix::<4, 4>::new(
            [[-2.0, 1.0, 2.0, 3.0], 
            [3.0, 2.0, 1.0, -1.0],
            [4.0, 3.0, 6.0, 5.0],
            [1.0, 2.0, 7.0, 8.0]]
        );

        let m3 = Matrix::<4, 4>::new(
            [[20.0, 22.0, 50.0, 48.0], 
            [44.0, 54.0, 114.0, 108.0],
            [40.0, 58.0, 110.0, 102.0],
            [16.0, 26.0, 46.0, 42.0]]
        );

        assert_eq!(&(m1 * m2), &m3);
    }

}