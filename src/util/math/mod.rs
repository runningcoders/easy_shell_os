mod fraction;
mod matrix;
mod tow_d;

trait Point {
    type Vector: Vector;
}

trait Vector {
    type Point: Point;
}

trait Line {
    type Point: Point;
}

trait Ray: Line {}

trait LineSegment: Ray {}

trait Plane {
    type Point: Point;
}

trait Polyhedron {
    type Point: Point;
}
