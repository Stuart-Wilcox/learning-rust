use matrix::Matrix;

fn main() {
    // create two matrices
    let mut m1: Matrix = Matrix::new_empty(2, 4);
    let mut m2: Matrix = Matrix::new_empty(2, 4);

    // set some values
    m1[0][0] = 100.2;
    m2[0][0] = 1.5;

    // print the matrices and their addition
    println!("m1: \n{:#?}", m1);
    println!("m2: \n{:#?}", m2);
    println!("m1+m2: \n{:#?}", m1.clone() + m2.clone());
    println!("m1-m2: \n{:#?}", m1.clone() - m2.clone());

    // check equivalencies
    println!("m1==m2: {}\n", m1 == m2);
    m2[0][0] = 100.2;
    println!("m1==m2: {}\n", m1 == m2);


    // create two more matrices using the constructor from vectors
    let m3 = Matrix::from_vector(
        vec![
            vec![1.0,2.0],
            vec![8.0,9.0],
        ]
    );

    let m4 = Matrix::from_vector(
        vec![
            vec![4.0, 5.0],
            vec![0.0, 1.0],
        ]
    );

    println!("m3*m4: \n{:#?}", m3 * m4);
}
