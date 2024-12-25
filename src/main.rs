mod geometry;
use crate::geometry::point::{p2p_dist, create_point};
use crate::geometry::types::XY;

fn main() {
    let p1: XY = create_point(4.0, 3.0);
    let p2: XY = create_point(6.0, 8.0);
    let dist: f64 = p2p_dist(p1, p2);
    println!("Distance from origin: {}", dist);
}
