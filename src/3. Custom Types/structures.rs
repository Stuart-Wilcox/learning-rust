use std::fmt::{Display, Formatter, Result};

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

// A unit struct
struct Unit;

// A tuple struct
struct Pair(i32, f32);

// A struct with two fields
struct Point {
    x: f32,
    y: f32,
}

// Structs can be reused as fields of another struct
#[allow(dead_code)]
struct Rectangle {
    // A rectangle can be specified by where the top left and bottom right
    // corners are in space.
    top_left: Point,
    bottom_right: Point,
}

impl Display for Rectangle {
    fn fmt(&self, f: &mut Formatter) -> Result {
        return write!(f, "({}, {}), ({}, {})", self.bottom_right.x, self.bottom_right.y, self.top_left.x, self.top_left.y);
    }
}

fn rect_area(rect: Rectangle) -> f32 {
    let Rectangle { top_left, bottom_right } = rect;
    let length: f32 = top_left.x - bottom_right.x;
    let width: f32 = top_left.y - bottom_right.y;

    let area: f32 = length * width;
    if area < 0.0 {
        return area * -1f32; // make positive
    }

    return area;
}

fn square(point: Point, size: f32) -> Rectangle {
    let point2: Point = Point { x: point.x + size, y: point.y + size };
    return Rectangle { bottom_right: point, top_left: point2 };
}

fn main() {
    // Create struct with field init shorthand
    let name = String::from("Peter");
    let age = 27;
    let peter = Person { name, age };

    // Print debug struct
    println!("{:?}", peter);

    // Instantiate a `Point`
    let point: Point = Point { x: 10.3, y: 0.4 };

    // Access the fields of the point
    println!("point coordinates: ({}, {})", point.x, point.y);

    // Make a new point by using struct update syntax to use the fields of our
    // other one
    let bottom_right = Point { x: 5.2, y: 1.4 };

    // `bottom_right.y` will be the same as `point.y` because we used that field
    // from `point`
    println!("second point: ({}, {})", bottom_right.x, bottom_right.y);

    // Destructure the point using a `let` binding
    let Point { x: left_edge, y: top_edge } = point;

    let _rectangle = Rectangle {
        // struct instantiation is an expression too
        top_left: Point { x: left_edge, y: top_edge },
        bottom_right: bottom_right,
    };

    // Instantiate a unit struct
    let _unit = Unit;

    // Instantiate a tuple struct
    let pair = Pair(1, 0.1);

    // Access the fields of a tuple struct
    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    // Destructure a tuple struct
    let Pair(integer, decimal) = pair;

    println!("pair contains {:?} and {:?}", integer, decimal);

    println!("Area of rectangle is {}", rect_area(_rectangle));

    println!("Square: {}", square(point, 2.0));
}
