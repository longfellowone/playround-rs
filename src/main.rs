#![allow(dead_code, unused_imports, unused_variables)]
use itertools::Itertools; // https://docs.rs/itertools/0.10.1/itertools/trait.Itertools.html

fn main() {
    let vertices = vertices_from_str("100 100 200 200 300 300");
    let length = total_length_between(&vertices);

    println!("{:?}", length)
}

#[derive(Debug, PartialEq)]
struct Vertex {
    x: f64,
    y: f64,
}

fn vertices_from_str(s: &str) -> Vec<Vertex> {
    s.split_whitespace()
        .filter_map(|s| s.parse::<f64>().ok())
        .tuples()
        .map(|p: (_, _)| Vertex { x: p.0, y: p.1 })
        .collect()
}

fn total_length_between(p: &[Vertex]) -> f64 {
    p.windows(2)
        .fold(0.0, |total, p| total + length_between(&p[0], &p[1]))
}

fn length_between(p0: &Vertex, p1: &Vertex) -> f64 {
    ((p1.x - p0.x).powf(2.0) + (p1.y - p0.y).powf(2.0)).sqrt()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vertices_from_str() {
        let str = "100 100 200 200 300 300";

        let p0 = Vertex { x: 100.0, y: 100.0 };
        let p1 = Vertex { x: 200.0, y: 200.0 };
        let p2 = Vertex { x: 300.0, y: 300.0 };

        let points = vec![p0, p1, p2];

        assert_eq!(vertices_from_str(str), points)
    }

    #[test]
    fn test_total_length_between() {
        let p0 = Vertex { x: 100.0, y: 100.0 };
        let p1 = Vertex { x: 200.0, y: 200.0 };
        let p2 = Vertex { x: 300.0, y: 300.0 };

        let points = vec![p0, p1, p2];

        assert_eq!(total_length_between(&points), 282.842712474619);
    }

    #[test]
    fn test_calculate_length_between_two_points() {
        let p0 = Vertex { x: 100.0, y: 100.0 };
        let p1 = Vertex { x: 200.0, y: 200.0 };

        assert_eq!(length_between(&p0, &p1), 141.4213562373095)
    }
}
