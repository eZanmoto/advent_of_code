use std::cmp;
use std::fmt;
use std::fs::File;
use std::io;
use std::io::prelude::*;

pub fn run() -> Result<Option<i32>, Error> {
    run_on_parsed_file("inputs/day_3_1.txt", closest_intersection)
}

pub fn run_on_parsed_file(path: &str, f: fn(&str, &str) -> Option<i32>) -> Result<Option<i32>, Error> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let paths: Vec<&str> = contents.split("\n").collect();
    if paths.len() != 3 {
        return Err(Error::FileFormat(format!("incorrect number of lines: expected 3, got {}", paths.len())));
    }
    if paths[2].len() != 0 {
        return Err(Error::FileFormat(format!("file should end with an empty line")));
    }

    Ok(f(paths[0], paths[1]))
}

#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    FileFormat(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Io(ref err) => write!(f, "IO error: {}", err),
            Error::FileFormat(ref err) => write!(f, "file format error: {}", err),
        }
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::Io(err)
    }
}

fn closest_intersection(a: &str, b: &str) -> Option<i32> {
    let paths = [a, b];
    let paths_pts = paths.iter().map(|p| parse_pts(p));
    let paths_lines: Vec<Vec<Ln>> = paths_pts.map(|p| pts_to_lines(p)).collect();

    let mut closest: Option<i32> = None;
    for a in &paths_lines[0] {
        for b in &paths_lines[1] {
            // TODO Mixing results and control flow seems like it could affect
            // the readability of the following `if` statement; see if there is
            // an alternative approach to destructuring an `Option` in a guard
            // in a readable way.

            let mut dist =
                if let Some(poi) = Ln::man_poi(a, b) {
                    poi.x.abs() + poi.y.abs()
                } else {
                    continue
                };

            let new_closest = match closest {
                None => dist,
                Some(v) => cmp::min(dist, v),
            };

            closest = Some(new_closest);
        }
    }

    closest
}

pub fn parse_pts(path: &str) -> Vec<Pt> {
    let mut pts = vec![Pt::new(0, 0)];

    for vec in path.split(",") {
        let (dir, mag) = vec.split_at(1);
        let mag = mag.parse::<i32>().unwrap();
        let (x, y) = match dir {
            "U" => (0, mag),
            "R" => (mag, 0),
            "D" => (0, -mag),
            "L" => (-mag, 0),
            _ => panic!("unexpected direction: {}", dir),
        };
        let pt = Pt::add(&pts[pts.len()-1], &Pt::new(x, y));
        pts.push(pt);
    }

    pts
}

#[derive(Clone, Copy)]
pub struct Pt {
    pub x: i32,
    pub y: i32,
}

impl Pt {
    pub fn new(x: i32, y: i32) -> Pt {
        Pt{x: x, y: y}
    }

    pub fn add(a: &Pt, b: &Pt) -> Pt {
        Pt{x: a.x + b.x, y: a.y + b.y}
    }
}

impl fmt::Display for Pt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

pub fn pts_to_lines(pts: Vec<Pt>) -> Vec<Ln> {
    (0..pts.len()-1)
        .map(|i| Ln::new(pts[i], pts[i+1]))
        .collect()
}

pub struct Ln {
    pub src: Pt,
    pub tgt: Pt,
}

impl Ln {
    pub fn new(src: Pt, tgt: Pt) -> Ln {
        Ln{src: src, tgt: tgt}
    }

    pub fn man_poi(a: &Ln, b: &Ln) -> Option<Pt> {
        let (vert, horz) =
            if a.src.x == a.tgt.x {
                (a, b)
            } else {
                (b, a)
            };

        // TODO Return `None` if `vert` isn't vertical or `horz` isn't
        // horizontal.

        let vx = vert.src.x;
        let hy = horz.src.y;
        let bw = |n, a, b| (a < n) == (n < b);

        if bw(vx, horz.src.x, horz.tgt.x) && bw(hy, vert.src.y, vert.tgt.y) {
            Some(Pt::new(vx, hy))
        } else {
            None
        }
    }
}

impl fmt::Display for Ln {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{} -> {}]", self.src, self.tgt)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn samples() {
        let test_cases = [
            (
                (
                    "R8,U5,L5,D3",
                    "U7,R6,D4,L4",
                ),
                Some(6),
            ),
            (
                (
                    "R75,D30,R83,U83,L12,D49,R71,U7,L72",
                    "U62,R66,U55,R34,D71,R55,D58,R83",
                ),
                Some(159),
            ),
            (
                (
                    "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51",
                    "U98,R91,D20,R16,D67,R40,U7,R15,U6,R7",
                ),
                Some(135),
            ),
        ];
        for &(src, tgt) in &test_cases {
            let x: Option<i32> = tgt;
            assert_eq!(closest_intersection(src.0, src.1), x);
        }
    }
}
