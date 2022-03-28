use std::ops::{Index, IndexMut, Add, Sub, Mul};
use std::fmt::{Debug,Formatter,Result};
use std::cmp::{PartialEq, Eq};
use std::clone::Clone;

// definition for Matrix struct
pub struct Matrix {
    // size
    n: usize,
    m: usize,

    // contents
    rows: Vec<Vec<f32>>,
}

// add debug print for fun
impl Debug for Matrix {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let mut out: String = format!("");
        for row in &self.rows {
            out = format!("{}[", out);
            for cell in row {
                out = format!("{}  {:#?}", out, cell);
            }
            out = format!("{}  ]\n", out);
        }
        return write!(f, "{}", out);
    }
}

// implement two constructors for Matrix
// the first one makes a new matrix of size x*y with empty slots
// the second makes a new matrix from the given vector of vectors of items
impl Matrix {
    pub fn new_empty(n: usize, m: usize) -> Matrix {
        let mut rows = Vec::new();

        for _i in 0..n {
            let mut row = Vec::new();
            for _j in 0..m {
                row.push(0.0f32);
            }
            rows.push(row);
        }

        let matrix: Matrix = Matrix { n, m, rows };
        return matrix;
    }

    pub fn from_vector(rows: Vec<Vec<f32>>) -> Matrix {
        let mut copy_rows: Vec<Vec<f32>> = Vec::new();
        let mut m = 0;
        let n = rows.capacity();
        for row in rows {
            let row_length = row.capacity();

            if m == 0 {
                m = row_length;
            }

            if m != row_length {
                panic!("Supplied rows are all different lengths! Rows must all be same length");
            }

            let mut copy_row = Vec::new();
            for cell in row {
                copy_row.push(cell);
            }
            copy_rows.push(copy_row);
        }

        let matrix: Matrix = Matrix { n, m, rows: copy_rows };
        return matrix;
    }

    // implement other generic methods of Matrix
    pub fn size(&self) -> (usize, usize) {
        return (self.n, self.m);
    }
}

// implement indexing for matrix
impl Index<usize> for Matrix {
    type Output = Vec<f32>;

    fn index(&self, index: usize) -> &Self::Output {
        return &self.rows[index];
    } 
}

// implement mutable indexing for matrix
impl IndexMut<usize> for Matrix {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        return &mut self.rows[index];
    } 
}

// implement add (+ operator) for Matrix
impl Add<Matrix> for Matrix {
    type Output = Matrix;

    fn add(self, rhs: Self) -> Self {
        // size must be the same in order to add
        if self.n == rhs.n && self.m == rhs.m {
            let mut rows: Vec<Vec<f32>> = Vec::new();

            for i in 0..self.n {
                let mut row: Vec<f32> = Vec::new();

                for j in 0..self.m {
                    row.push(self[i][j] + rhs[i][j]);
                }

                rows.push(row);
            }

            return Matrix::from_vector(rows);
        }

        panic!("LHS and RHS Matrix must be same size");
    }
}

// implement subtract (- operator) for Matrix
impl Sub<Matrix> for Matrix {
    type Output = Matrix;

    fn sub(self, rhs: Self) -> Self {
        // size must be same in order to subtract
        if self.n == rhs.n && self.m == rhs.m {
            let mut rows: Vec<Vec<f32>> = Vec::new();

            for i in 0..self.n {
                let mut row: Vec<f32> = Vec::new();

                for j in 0..self.m {
                    row.push(self[i][j] - rhs[i][j]);
                }

                rows.push(row);
            }

            return Matrix::from_vector(rows);
        }

        panic!("LHS and RHS Matrix must be same size");        
    }
}

// implement multiple (* operator) for Matrix
impl Mul<Matrix> for Matrix {
    type Output = Matrix;

    fn mul(self, rhs: Self) -> Self {
        // size must match to multiply
        if self.m == rhs.n {
            let mut rows: Vec<Vec<f32>> = Vec::new();

            // initialize all matrix cells to zero
            for _i in 0..self.n {
                let mut row: Vec<f32> = Vec::new();
                for _j in 0..rhs.m {
                    row.push(0.0f32);
                }
                rows.push(row);
            }

            // perform multiplication
            for i in 0..self.n {
                for j in 0..rhs.m {
                    for k in 0..rhs.n {
                        rows[i][j] += self[i][k] * rhs[k][j];
                    }
                }
            }

            return Matrix::from_vector(rows);
        }

        panic!("LHS and RHS Matrix must match size to multiply");
    }
}

impl Mul<f32> for Matrix {
    type Output = Matrix;

    fn mul(self, rhs: f32) -> Self::Output {
        // linear multiply each cell by input
        let mut rows: Vec<Vec<f32>> = Vec::new();

        for i in 0..self.n {
            let mut row: Vec<f32> = Vec::new();

            for j in 0..self.m {
                row.push(self[i][j] * rhs);
            }

            rows.push(row);
        }

        return Matrix::from_vector(rows);
    }
}

// implement equality for Matrix
impl PartialEq for Matrix {
    fn eq(&self, other: &Self) -> bool {
        // compare sizes
        if self.n == other.n && self.m == other.m {
            // compare each cell
            for i in 0..self.n {
                for j in 0..self.m {
                    if self[i][j] != other[i][j] {
                        return false;
                    }
                }
            }

            return true;
        }

        return false;
    }
}

// needed to complete equality for Matrix
impl Eq for Matrix { }

impl Clone for Matrix {
    fn clone(&self) -> Self {
        let mut matrix = Matrix::new_empty(self.n, self.m);
        for i in 0..self.n {
            for j in 0..self.m {
                matrix[i][j] = self[i][j];
            }
        }

        return matrix;
    }
}
