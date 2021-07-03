#[derive(Default, Debug)]
pub struct Polygon<T> {
    pub vertexes: Vec<T>,
}

trait Coordinates {}

#[derive(Default)]
struct CartesianCoord {
    x: f64,
    y: f64,
}
impl Coordinates for CartesianCoord {}

#[derive(Default)]
struct PolarCoord {
    r: f64,
    theta: f64,
}
impl Coordinates for PolarCoord {}

fn main() {
    let vertexes = vec![
        CartesianCoord { x: 0.0, y: 0.0 },
        CartesianCoord { x: 50.0, y: 0.0 },
        CartesianCoord { x: 30.0, y: 20.0 },
    ];
    let poly = Polygon {
        vertexes,
        ..Default::default()
    };
    println!("{}", poly.vertexes[2].x);
}
