use std::{fmt::Display, ops::{BitAnd, BitOr, Mul}};

use itertools::Itertools;
use nalgebra::DMatrix;
#[derive(Debug,Clone,PartialEq, Eq)]
pub struct RelationMatrix(pub DMatrix<u8>);

impl BitAnd for RelationMatrix {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        &self & &rhs
}
}
impl<'a, 'b> BitAnd<&'b RelationMatrix> for &'a RelationMatrix {
    type Output = RelationMatrix;

    fn bitand(self, rhs: &'b RelationMatrix) -> RelationMatrix {
        
        assert_eq!(self.0.shape(),rhs.0.shape(),"Dimension of the two matrices differ!");
        RelationMatrix(
        DMatrix::from_fn(self.0.nrows(), self.0.ncols(), 
            |i, j| self.0[(i,j)] & rhs.0[(i,j)])
        )    
    }
}
impl BitOr for RelationMatrix {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        &self | &rhs
    }
}
impl<'a, 'b> BitOr<&'b RelationMatrix> for &'a RelationMatrix {
    type Output = RelationMatrix;

    fn bitor(self, rhs: &'b RelationMatrix) -> RelationMatrix {
        
        assert_eq!(self.0.shape(),rhs.0.shape(),"Dimension of the two matrices differ!");

        RelationMatrix(
        DMatrix::from_fn(self.0.nrows(), self.0.ncols(), 
            |i, j| self.0[(i,j)] | rhs.0[(i,j)])
        )    
    }
}
impl Mul for RelationMatrix {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        &self*&rhs
    }
}
impl<'a, 'b> Mul<&'b RelationMatrix> for &'a RelationMatrix {
    type Output = RelationMatrix;
        fn mul(self, rhs: &'b RelationMatrix) -> Self::Output {
        assert_eq!(self.0.ncols(),rhs.0.nrows(),"({:?})-matrix and ({:?})-matrix can't be multiplied!", self.0.shape(),rhs.0.shape());
        let v  =self.0.row_iter()
            .map(|r| 
                    rhs.0.column_iter()
                        .map(|c|
                            scalar_multiplication(r, c)
                        ).collect_vec()
                ).concat();
        let mat = DMatrix::from_vec(
            self.0.nrows(), 
            rhs.0.ncols(), 
            v)
            .transpose();
        RelationMatrix(mat)
    }
    

}

impl RelationMatrix {
    pub fn fast_pow(&self, mut exp: u64) -> Self {
        let mut base = self.clone();
    let mut result = RelationMatrix(DMatrix::identity(self.0.nrows(), self.0.ncols()));
     while exp > 0 {
        if exp % 2 == 1 {
            result = &result*&base;
        }
        base = &base*&base;
        exp /= 2;
    }
    result 
}
}
impl Display for RelationMatrix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
 write!(f,"{}", self.0)
    }
}
pub fn scalar_multiplication(
    row:nalgebra::Matrix<u8, nalgebra::Const<1>, nalgebra::Dyn, nalgebra::ViewStorage<'_, u8, nalgebra::Const<1>, nalgebra::Dyn, nalgebra::Const<1>, nalgebra::Dyn>>,
    col:nalgebra::Matrix<u8, nalgebra::Dyn, nalgebra::Const<1>, nalgebra::ViewStorage<'_, u8, nalgebra::Dyn, nalgebra::Const<1>, nalgebra::Const<1>, nalgebra::Dyn>>)-> u8 {
    row.iter().zip(col.iter()).map(|(a,b)| a&b).fold(0u8, |acc,x| acc|x)
 
    }
