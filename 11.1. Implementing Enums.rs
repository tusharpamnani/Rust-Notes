// Define an enum called Shape
enum Shape {
    Circle(f64),  // Variant with associated data (radius)
    Square(f64),  // Variant with associated data (side length)
    Rectangle(f64, f64),  // Variant with associated data (width, height)
}

// Define an enum called Direction
enum Direction {
    North,
    East,
    South,
    West,
}

fn main() {
    // Create instances of different shapes
    let circle = Shape::Circle(5.0);
    let square = Shape::Square(4.0);
    let rectangle = Shape::Rectangle(3.0, 6.0);
    // Create instances of diiferent directions
    let my_direction_north = Direction::North;
    let my_direction_south = Direction::South;
}
