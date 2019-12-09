use std::cmp;

use day_3_1;

pub fn run() -> Result<Option<i32>, day_3_1::Error> {
    day_3_1::run_on_parsed_file("inputs/day_3_1.txt", shortest_intersection)
}

fn shortest_intersection(a: &str, b: &str) -> Option<i32> {
    let paths = [a, b];
    let paths_pts = paths.iter().map(|p| day_3_1::parse_pts(p));
    let paths_lns: Vec<Vec<day_3_1::Ln>> = paths_pts.map(|p| day_3_1::pts_to_lines(p)).collect();

    let mut shortest: Option<i32> = None;

    let mut a_steps = 0;
    for a in &paths_lns[0] {
        let mut b_steps = 0;

        for b in &paths_lns[1] {
            let b_len = Pt::sub(&b.tgt, &b.src).man_mag();
            let mut poi =
                if let Some(v) = day_3_1::Ln::man_poi(&a, &b) {
                    if v.x == 0 && v.y == 0 {
                        b_steps += b_len;
                        continue
                    } else {
                        v
                    }
                } else {
                    b_steps += b_len;
                    continue
                };

            let dist =
                a_steps + Pt::sub(&poi, &a.src).man_mag() +
                b_steps + Pt::sub(&poi, &b.src).man_mag();

            let new_shortest = match shortest {
                None => dist,
                Some(v) => cmp::min(dist, v),
            };
            shortest = Some(new_shortest);

            b_steps += b_len;
        }

        a_steps += Pt::sub(&a.tgt, &a.src).man_mag();
    }

    shortest
}

type Pt = day_3_1::Pt;

impl Pt {
    fn sub(a: &Pt, b: &Pt) -> Pt {
        day_3_1::Pt::new(a.x - b.x, a.y - b.y)
    }

    fn man_mag(&self) -> i32 {
        self.x.abs() + self.y.abs()
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
                Some(30),
            ),
            (
                (
                    "R75,D30,R83,U83,L12,D49,R71,U7,L72",
                    "U62,R66,U55,R34,D71,R55,D58,R83",
                ),
                Some(610),
            ),
            (
                (
                    "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51",
                    "U98,R91,D20,R16,D67,R40,U7,R15,U6,R7",
                ),
                Some(410),
            ),
        ];
        for &(src, tgt) in &test_cases {
            let x: Option<i32> = tgt;
            assert_eq!(shortest_intersection(src.0, src.1), x);
        }
    }
}
