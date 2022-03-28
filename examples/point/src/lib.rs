use std::fmt::{Debug,Formatter,Result};
use std::f32::consts::PI;
use matrix::Matrix;

pub struct Point {
    x: f32,
    y: f32,
    z: f32,
}

impl Debug for Point {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        return write!(f, "({}, {}, {})", self.x, self.y,self.z);
    }
}

impl Copy for Point { }

impl Clone for Point {
    fn clone(&self) -> Point {
        return Point {
            x: self.x,
            y: self.y,
            z: self.z,
        };
    }
}

impl Point {
    pub fn new(x: f32, y: f32, z: f32) -> Point {
        return Point {
            x,
            y,
            z,
        };
    }

    pub fn from_point(point: Point) -> Point {
        return Point {
            x: point.x,
            y: point.y,
            z: point.z,
        };
    }

    pub fn to_matrix(self) -> Matrix {
        return Matrix::from_vector(
            vec![
                vec![self.x],
                vec![self.y],
                vec![self.z],
            ]
        );
    }

    pub fn rotate_x(self, angle: f32) -> Point {
        let rot: Matrix = get_x_rotation_matrix(angle);
        let pt: Matrix = matrix_from_point(self);

        let new_pt: Matrix = rot*pt;
        return Point { x: new_pt[0][0], y: new_pt[1][0], z: new_pt[2][0] };
    }

    pub fn rotate_y(self, angle: f32) -> Point {
        let rot: Matrix = get_y_rotation_matrix(angle);
        let pt: Matrix = matrix_from_point(self);

        let new_pt: Matrix = rot * pt;
        return Point { x: new_pt[0][0], y: new_pt[1][0], z: new_pt[2][0] };
    }

    pub fn rotate_z(self, angle: f32) -> Point {
        let rot: Matrix = get_z_rotation_matrix(angle);
        let pt: Matrix = matrix_from_point(self);

        let new_pt:Matrix = rot * pt;
        return Point {x: new_pt[0][0], y: new_pt[1][0], z: new_pt[2][0] };
    }

    pub fn translate(self, point: Point) -> Point {
        return Point {
            x: self.x + point.x,
            y: self.y + point.y,
            z: self.z + point.z,
        };
    }

    pub fn magnitude(self) -> f32 {
        let magnitude_square = {
            (self.x * self.x) +
            (self.y * self.y) +
            (self.z * self.z)
        };

        return magnitude_square.sqrt();
    }
}

fn get_x_rotation_matrix(angle: f32) -> Matrix {
    let angle_rad = (angle * PI) / 180.0;

    let m = vec![
        vec![1.0, 0.0, 0.0],
        vec![0.0, angle_rad.cos(), -1.0 * angle_rad.sin()],
        vec![0.0, angle_rad.sin(), angle_rad.cos()],
    ];

    return Matrix::from_vector(m);
}

fn get_y_rotation_matrix(angle: f32) -> Matrix {
    let angle_rad = (angle * PI) / 180.0;

    let m = vec![
        vec![angle_rad.cos(), 0.0, angle_rad.sin()],
        vec![0.0, 1.0, 0.0],
        vec![-1.0 * angle_rad.sin(), 0.0, angle_rad.cos()],
    ];

    return Matrix::from_vector(m);
}

fn get_z_rotation_matrix(angle: f32) -> Matrix {
    let angle_rad = (angle * PI) / 180.0;

    let m = vec![
        vec![angle_rad.cos(), -1.0 * angle_rad.sin(), 0.0],
        vec![angle_rad.sin(), angle_rad.cos(), 0.0],
        vec![0.0, 0.0, 1.0],
    ];

    return Matrix::from_vector(m);
}

fn matrix_from_point(point: Point) -> Matrix {
    return Matrix::from_vector(
        vec![
            vec![point.x],
            vec![point.y],
            vec![point.z],
        ]
    );
}
