use std::hash;

use day_3_1;
use day_6_1;

pub fn run() -> Result<Option<i32>, day_3_1::Error> {
    day_6_1::run_on_input(orbit_tfs)
}

fn orbit_tfs(links: &[&str]) -> Option<i32> {
    let g = day_6_1::graph_from_links(links);

    graph_dist(&g, &"COM", &(&"YOU", &"SAN"))
}

fn graph_dist<T: Eq + hash::Hash>(g: &day_6_1::Graph<T>, origin: &T, tgts: &(&T, &T)) -> Option<i32> {
    fn dist<T: Eq + hash::Hash>(g: &day_6_1::Graph<T>, origin: &T, tgts: &(&T, &T)) -> (Option<i32>, Option<i32>) {
        if origin == tgts.0 {
            (Some(0), None)
        } else if origin == tgts.1 {
            (None, Some(0))
        } else {
            match g.get(origin) {
                None => (None, None),
                Some(nbrs) => {
                    let mut d: (Option<i32>, Option<i32>) = (None, None);

                    // NOTE An optimisation here is to break the loop once both
                    // elements of `d` are non-`None`.
                    for nbr in nbrs {
                        let nbr_d = dist(&g, &nbr, &tgts);
                        match nbr_d {
                            (Some(_), Some(_)) => {
                                return nbr_d;
                            },
                            (Some(n), None) => {
                                d.0 = Some(n + 1);
                            },
                            (None, Some(n)) => {
                                d.1 = Some(n + 1);
                            },
                            _ => {},
                        };
                    }

                    d
                }
            }
        }
    }

    match dist(&g, &origin, &tgts) {
        (Some(n), Some(m)) => Some((n + m) - 2),
        _ => None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn samples() {
        let test_cases = [
            (
                [
                    "COM)B",
                    "B)C",
                    "C)D",
                    "D)E",
                    "E)F",
                    "B)G",
                    "G)H",
                    "D)I",
                    "E)J",
                    "J)K",
                    "K)L",
                    "K)YOU",
                    "I)SAN",
                ],
                Some(4),
            ),
        ];
        for &(ref src, tgt) in &test_cases {
            assert_eq!(orbit_tfs(src), tgt);
        }
    }
}
