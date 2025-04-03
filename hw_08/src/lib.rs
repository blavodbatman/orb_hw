#[allow(dead_code)]
trait Shape {
    fn get_area(&self) -> f64;
    fn get_perimeter(&self) -> f64;
}

#[allow(dead_code)]
struct Triangle {
    sides_lens: [f64; 3],
}

impl Shape for Triangle {
    fn get_area(&self) -> f64 {
        let p = self.get_perimeter() / 2.;
        (p * (p - self.sides_lens[0]) * (p - self.sides_lens[1]) * (p - self.sides_lens[2])).sqrt()
    }

    fn get_perimeter(&self) -> f64 {
        self.sides_lens[0] + self.sides_lens[1] + self.sides_lens[2]
    }
}

#[allow(dead_code)]
struct Rectangle {
    width: f64,
    height: f64,
}

impl Shape for Rectangle {
    fn get_area(&self) -> f64 {
        self.width * self.height
    }

    fn get_perimeter(&self) -> f64 {
        2. * self.width + 2. * self.height
    }
}

#[allow(dead_code)]
struct Circle {
    radius: f64,
}

impl Shape for Circle {
    fn get_area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }

    fn get_perimeter(&self) -> f64 {
        2. * std::f64::consts::PI * self.radius
    }
}

#[allow(dead_code)]
fn perimeter_by_area(shape: impl Shape) -> f64 {
    shape.get_perimeter() / shape.get_area()
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::relative_eq;

    #[test]
	#[allow(unused_must_use)]
    fn test() {
        relative_eq!(
            perimeter_by_area(Triangle {
                sides_lens: [3.0, 4.0, 5.0]
            }),
            2.0
        );
        relative_eq!(perimeter_by_area(Circle { radius: 2.0 }), 1.0);
        relative_eq!(
            perimeter_by_area(Rectangle {
                width: 2.0,
                height: 3.0,
            }),
            1.6666
        );
    }
}
