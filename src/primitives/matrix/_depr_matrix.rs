// Attempt to implement Matrix using const generics. There are some limitations const generic expressions (even with nightly compiler features enabled), so some recursive functionality is not working. 
// However I want to keep this implementation alive to get back to it later

// use std::ops::{Index, Mul};
// use num::Integer;

// use crate::tuple::Tuple;


// #[derive(Debug, PartialEq, PartialOrd)]
// pub struct Matrix<const M: usize, const N: usize> {
//     pub data: [[f64; N]; M]
// }

// impl<const M: usize, const N: usize> Matrix<M, N> {
//     pub fn new(data: [[f64; N]; M]) -> Self {
//         Matrix { data: data }
//     }

//     // construct identity matrix
//     pub fn identity() -> Self {
//         let mut tmp: [[f64; N]; M] = [[0.0; N]; M];
//         for row_idx in 0..M {
//             for col_idx in 0..N {
//                 if row_idx == col_idx {
//                     tmp[row_idx][col_idx] = 1.0;
//                 }
//             }
//         }
//         Matrix { data: tmp }
//     }

//     pub fn transpose(&self) -> Matrix<N, M> {
//         let mut tmp: [[f64; M]; N] = [[0.0; M]; N];
//         for (row_idx, row) in self.data.iter().enumerate() {
//             for (col_idx, elem) in row.iter().enumerate() {
//                 tmp[col_idx][row_idx] = *elem;
//             }
//         }

//         Matrix { data: tmp }
//     }

//     pub fn minor(&self, row_idx: usize, col_idx: usize) -> f64 
//     where
//         [(); N - 1]:,
//         [(); M - 1]:,
//         [(); N - 1 - 1]:,
//         [(); M - 1 - 1]:,
//     {
//         // for now
//         assert!(N == 3 && M == 3);

//         let submatrix = self.submatrix(row_idx, col_idx);
//         submatrix.det()
//     }

//     pub fn cofactor(&self, row_idx: usize, col_idx: usize) -> f64 
//     where
//         [(); N - 1]:,
//         [(); M - 1]:,
//         [(); N - 1 - 1]:,
//         [(); M - 1 - 1]:,
//     {
//         // for now
//         assert!(N == 3 && M == 3);

//         let minor_value = self.minor(row_idx, col_idx);
//         if (row_idx + col_idx) % 2 == 0 {
//             minor_value
//         } else {
//             -minor_value
//         }
//     }

//     pub fn det(&self) -> f64 
//     where
//         [(); N - 1]:,
//         [(); M - 1]:,
//     {    
//         let mut result: f64 = 0.0;
//         for i in 0..N {
//             result += self.cofactor(0, i);
//         }
//         result
//     }

//     fn det2(&self) -> f64 {
//         assert!(N == 2 && M == 2);
//         self[(0, 0)] * self[(1, 1)] - self[(0, 1)] * self[(1, 0)]
//     }
    
//     pub fn submatrix(&self, row_idx_skip: usize, col_idx_skip: usize) -> Matrix<{ M-1 }, { N-1 }> {

//         assert!(N > 1 && M > 1);

//         let mut tmp: [[f64; N-1]; M-1] = [[0.0; N-1]; M-1];
    
//         let mut row_idx: usize = 0;
//         let mut col_idx: usize = 0;
//         for m in 0..M {
//             if m == row_idx_skip {
//                 continue;
//             }
//             for c in 0..N {
//                 if c == col_idx_skip {
//                     continue;
//                 }
//                 tmp[row_idx][col_idx] = self[(m, c)];
//                 col_idx += 1
//             }
//             row_idx += 1;
//             col_idx = 0;
//         }

//         Matrix { data: tmp }
//     }
// }

// impl<const M: usize, const N: usize> Index<(usize, usize)> for Matrix<M, N> {
//     type Output = f64;

//     fn index(&self, idx_pair: (usize, usize)) -> &Self::Output {
//         &self.data[idx_pair.0][idx_pair.1]
//     }
// }

// // Implement multiplication by another MxN matrix
// impl<const M: usize, const N: usize> Mul<Matrix<N, M>> for Matrix<M, N> {
//     type Output = Matrix<N, N>;

//     fn mul(self, rhs: Matrix<N, M>) -> Self::Output {
//         let mut tmp: [[f64; N]; N] = [[0.0; N]; N];
//         for (row_idx, row) in self.data.iter().enumerate() {
//             for (col_idx, col) in rhs.transpose().data.iter().enumerate() {
//                 for (r, c) in row.iter().zip(col.iter()) {
//                     tmp[row_idx][col_idx] += r * c;
//                 }
//             }
//         }
//         Matrix { data: tmp }
//     }
// }

// // Implement multiplication by tuple of 4 elements
// impl<const M: usize> Mul<Tuple> for Matrix<M, 4> {
//     type Output = Tuple;

//     fn mul(self, rhs: Tuple) -> Self::Output {
//         let mut tmp: [f64; 4] = [0.0; 4];
//         for (row_idx, row) in self.data.iter().enumerate() {
//             for (r, c) in row.iter().zip(rhs.into_iter()) {
//                 tmp[row_idx] += r * c;
//             }
//         }
//         Tuple::from_array(tmp)
//     }
// }

// #[cfg(test)]
// mod tests {

//     use super::Matrix;

//     #[test]
//     fn indexing() {

//         let m = Matrix::<2, 2>::new(
//             [[-3.0, 5.0],
//             [1.0, -2.0]]
//         );
//         assert_eq!(m[(0, 0)], -3.0);
//         assert_eq!(m[(1, 0)], 1.0);
//         assert_eq!(m[(0, 1)], 5.0);
//         assert_eq!(m[(1, 1)], -2.0);

//         let m = Matrix::<3, 3>::new(
//             [[-3.0, 5.0, 0.0],
//             [1.0, -2.0, -7.0],
//             [0.0, 1.0, 1.0]]
//         );

//         assert_eq!(m[(0, 0)], -3.0);
//         assert_eq!(m[(1, 1)], -2.0);
//         assert_eq!(m[(2, 2)], 1.0);

//         let m = Matrix::<4, 4>::new(
//             [[1.0, 2.0, 3.0, 4.0], 
//             [5.5, 6.5, 7.5, 8.5],
//             [9.0, 10.0, 11.0, 12.0],
//             [13.5, 14.5, 15.5, 16.5]]
//         );

//         assert_eq!(m[(0, 0)], 1.0);
//         assert_eq!(m[(1, 0)], 5.5);
//         assert_eq!(m[(2, 0)], 9.0);
//         assert_eq!(m[(3, 0)], 13.5);
//         assert_eq!(m[(0, 1)], 2.0);
//         assert_eq!(m[(0, 2)], 3.0);
//         assert_eq!(m[(0, 3)], 4.0);
//         assert_eq!(m[(1, 1)], 6.5);
//         assert_eq!(m[(1, 2)], 7.5);
//         assert_eq!(m[(2, 2)], 11.0);
//         assert_eq!(m[(3, 0)], 13.5);
//         assert_eq!(m[(3, 2)], 15.5);
//         assert_eq!(m[(3, 3)], 16.5);
//     }

//     #[test]
//     fn equality() {
//         let m1 = Matrix::<4, 4>::new(
//             [[1.0, 2.0, 3.0, 4.0], 
//             [5.5, 6.5, 7.5, 8.5],
//             [9.0, 10.0, 11.0, 12.0],
//             [13.5, 14.5, 15.5, 16.5]]
//         );

//         let m2 = Matrix::<4, 4>::new(
//             [[1.0, 2.0, 3.0, 4.0], 
//             [5.5, 6.5, 7.5, 8.5],
//             [9.0, 10.0, 11.0, 12.0],
//             [13.5, 14.5, 15.5, 16.5]]
//         );

//         assert_eq!(m1, m2);

//         let m3= Matrix::<4, 4>::new(
//             [[1.0, 2.0, 3.0, 4.0], 
//             [5.5, 6.5, 7.5, 8.5],
//             [9.0, 10.0, 11.0, 12.0],
//             [13.5, 14.5, 15.5, 16.0]]
//         );

//         assert_ne!(m3, m2);
//         assert_ne!(m3, m1);
//     }

//     #[test]
//     fn transpose() {
//         let m = Matrix::<2, 2>::new(
//             [[-3.0, 5.0],
//             [1.0, -2.0]]
//         );

//         let m_t = m.transpose();

//         assert_eq!(m_t[(0, 0)], -3.0);
//         assert_eq!(m_t[(1, 0)], 5.0);
//         assert_eq!(m_t[(0, 1)], 1.0);
//         assert_eq!(m_t[(1, 1)], -2.0);
//     }

//     #[test]
//     fn multiply() {
//         let m1 = Matrix::<4, 4>::new(
//             [[1.0, 2.0, 3.0, 4.0], 
//             [5.0, 6.0, 7.0, 8.0],
//             [9.0, 8.0, 7.0, 6.0],
//             [5.0, 4.0, 3.0, 2.0]]
//         );

//         let m2 = Matrix::<4, 4>::new(
//             [[-2.0, 1.0, 2.0, 3.0], 
//             [3.0, 2.0, 1.0, -1.0],
//             [4.0, 3.0, 6.0, 5.0],
//             [1.0, 2.0, 7.0, 8.0]]
//         );

//         let m3 = Matrix::<4, 4>::new(
//             [[20.0, 22.0, 50.0, 48.0], 
//             [44.0, 54.0, 114.0, 108.0],
//             [40.0, 58.0, 110.0, 102.0],
//             [16.0, 26.0, 46.0, 42.0]]
//         );

//         assert_eq!(&(m1 * m2), &m3);
//     }

//     #[test]
//     fn multiply_by_tuple() {
//         use crate::tuple::Tuple;

//         let m1 = Matrix::<4, 4>::new(
//             [[1.0, 2.0, 3.0, 4.0], 
//             [2.0, 4.0, 4.0, 2.0],
//             [8.0, 6.0, 4.0, 1.0],
//             [0.0, 0.0, 0.0, 1.0]]
//         );

//         let t1 = Tuple::new(1.0, 2.0, 3.0, 1.0);

//         let t = Tuple::new(18.0, 24.0, 33.0, 1.0);

//         assert_eq!(&(m1 * t1), &t);
//     }

//     #[test]
//     fn multiply_by_identity() {
//         use crate::tuple::Tuple;

//         let m1 = Matrix::<4, 4>::new(
//             [[1.0, 2.0, 3.0, 4.0], 
//             [2.0, 4.0, 4.0, 2.0],
//             [8.0, 6.0, 4.0, 1.0],
//             [0.0, 0.0, 0.0, 1.0]]
//         );
        
//         let m_id = Matrix::<4, 4>::identity();

//         let m = Matrix::<4, 4>::new(
//             [[1.0, 2.0, 3.0, 4.0], 
//             [2.0, 4.0, 4.0, 2.0],
//             [8.0, 6.0, 4.0, 1.0],
//             [0.0, 0.0, 0.0, 1.0]]
//         );

//         assert_eq!(&(m1 * m_id), &m);

//         let m_id = Matrix::<4, 4>::identity();

//         let t1 = Tuple::new(1.0, 2.0, 3.0, 1.0);

//         let t = Tuple::new(1.0, 2.0, 3.0, 1.0);

//         assert_eq!(&(m_id * t1), &t);
//     }

//     #[test]
//     fn submatrix() {

//         let m1 = Matrix::<3, 3>::new(
//             [[1.0, 5.0, 0.0], 
//             [-3.0, 2.0, 7.0],
//             [0.0, 6.0, -3.0]]
//         );

//         let m2 = m1.submatrix(0, 2);

//         let m = Matrix::<2, 2>::new(
//             [[-3.0, 2.0],
//             [0.0, 6.0]]
//         );

//         assert_eq!(&m2, &m);

//         let m1 = Matrix::<4, 4>::new(
//             [[-6.0, 1.0, 1.0, 6.0], 
//             [-8.0, 5.0, 8.0, 6.0],
//             [-1.0, 0.0, 8.0, 2.0],
//             [-7.0, 1.0, -1.0, 1.0]]
//         );

//         let m2 = m1.submatrix(2, 1);

//         let m = Matrix::<3, 3>::new(
//             [[-6.0, 1.0, 6.0],
//             [-8.0, 8.0, 6.0],
//             [-7.0, -1.0, 1.0]]
//         );

//         assert_eq!(&m2, &m);

//     }

//     #[test]
//     fn minor() {

//         let m1 = Matrix::<3, 3>::new(
//             [[3.0, 5.0, 0.0], 
//             [2.0, -1.0, -7.0],
//             [6.0, -1.0, 5.0]]
//         );

//         assert_eq!(m1.minor(0, 0), -12.0);
//         assert_eq!(m1.minor(1, 0), 25.0);
//     }

//     #[test]
//     fn cofactor() {

//         let m1 = Matrix::<3, 3>::new(
//             [[3.0, 5.0, 0.0], 
//             [2.0, -1.0, -7.0],
//             [6.0, -1.0, 5.0]]
//         );

//         assert_eq!(m1.minor(0, 0), -12.0);
//         assert_eq!(m1.cofactor(0, 0), -12.0);
//         assert_eq!(m1.cofactor(1, 0), -25.0);
//     }

// }