use point::Point;

fn main() {
    let p1: Point = Point::new(1.0, 1.0, 1.0);
    println!("p1: {:#?}", p1);
    println!("p1 magnitude: {}", p1.magnitude());

    let p2 = p1.translate(Point::new(1.0, 1.0, 1.0));
    println!("p2: {:#?}", p2);

    let p3 = p2.rotate_z(90.0);
    println!("p3: {:#?}", p3);

    println!("p3 matrix:\n{:#?}", p3.clone().to_matrix());

    let p4 = p3.rotate_y(45.0);
    println!("p4: {:#?}", p4);
}
