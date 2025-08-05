use std::collections::HashSet;

use nalgebra::DMatrix;
use relations::{relation_matrix::RelationMatrix, relations::Relation};

fn main() {
    let matrix_a = DMatrix::from_row_slice(
        3, 
        2, 
        &[1u8,0,0,1,1,0]);
    let matrix_b = DMatrix::from_row_slice(
        2, 
        3, 
        &[1,1,0,0,1,1]);
let matrix_a = RelationMatrix(matrix_a);
let matrix_b = RelationMatrix(matrix_b);
println!("A = {}",matrix_a);
println!("B = {}",matrix_b.0.transpose());
let t = matrix_a*matrix_b;
println!("t {}",t);
//Power check Example 2.6
let matrix_a = 
    DMatrix::from_row_slice(
        3, 
        3, 
        &[0,0,1,1,0,0,1,1,0]);
let matrix_a = RelationMatrix(matrix_a);
for i in 0..6 {
    let pow = matrix_a.fast_pow(i);
    println!("A^{} = {}",i,pow);
}
//Composition of relation check Example 2.8
let matrix_r = 
DMatrix::from_row_slice(
    3, 
    3, 
    &[1,0,1,1,1,0,0,0,0]);
let matrix_s = 
DMatrix::from_row_slice(
    3, 
    3, 
    &[0,1,0,0,0,1,1,0,1]);
    let matrix_r = RelationMatrix(matrix_r);
    let matrix_s  = RelationMatrix(matrix_s);
    let rs = &matrix_r*&matrix_s;
    println!("rs {}",rs);
    let pow = matrix_r.fast_pow(2);
    println!("pow {}",pow);
// Example 3.4

let a: HashSet<u64> = [0,1,2,3].into_iter().collect();
let rel: Vec<(u64, u64)> = vec![
    (0,1), (0,2), (1,0),
    (1,3), (2,1), (2,3)
 ];
 let relation  = Relation{
    a: a.clone(),
    b: a.clone(),
    rel,
};
let matrix_r = relation.zero_one_matrix();
println!("r = {}",matrix_r);
let r4 = matrix_r.fast_pow(4);
println!("r^4 = {}",r4);
println!("Example transitive closure computation. Example 3.16");
let a: HashSet<u64> = (0..=3).into_iter().collect();
let rel: Vec<(u64, u64)> = vec![
    (0,1), (1,0),
    (1,2), (2,3)
 ];
 let relation  = Relation{
    a: a.clone(),
    b: a.clone(),
    rel,
};
let cl_trans = relation.get_transitive_closure();
println!("r = {:?}",relation.rel);
println!("cl = {:?}",cl_trans.rel);
println!("is transitive cl {}",cl_trans.is_transitive());
 
}
