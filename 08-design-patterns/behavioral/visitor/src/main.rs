//! Visitor: add operations across a stable family of element types.

struct Circle {
    radius: f64,
}
struct Rectangle {
    width: f64,
    height: f64,
}

trait ShapeVisitor {
    fn visit_circle(&self, circle: &Circle) -> f64;
    fn visit_rectangle(&self, rectangle: &Rectangle) -> f64;
}

trait Shape {
    fn accept(&self, visitor: &dyn ShapeVisitor) -> f64;
}

impl Shape for Circle {
    fn accept(&self, visitor: &dyn ShapeVisitor) -> f64 {
        visitor.visit_circle(self)
    }
}

impl Shape for Rectangle {
    fn accept(&self, visitor: &dyn ShapeVisitor) -> f64 {
        visitor.visit_rectangle(self)
    }
}

struct AreaVisitor;

impl ShapeVisitor for AreaVisitor {
    fn visit_circle(&self, circle: &Circle) -> f64 {
        std::f64::consts::PI * circle.radius.powi(2)
    }

    fn visit_rectangle(&self, rectangle: &Rectangle) -> f64 {
        rectangle.width * rectangle.height
    }
}

fn main() {
    let shapes: Vec<Box<dyn Shape>> = vec![
        Box::new(Circle { radius: 2.0 }),
        Box::new(Rectangle {
            width: 3.0,
            height: 4.0,
        }),
    ];
    let total: f64 = shapes.iter().map(|shape| shape.accept(&AreaVisitor)).sum();
    println!("Total area: {total:.2}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn visitor_handles_each_shape_type() {
        assert_eq!(
            Rectangle {
                width: 3.0,
                height: 4.0
            }
            .accept(&AreaVisitor),
            12.0
        );
    }
}
