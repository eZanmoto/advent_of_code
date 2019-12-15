use std::collections;
use std::fs;
use std::hash;
use std::io::prelude::*;

use day_3_1;

pub fn run() -> Result<i32, day_3_1::Error> {
    let mut file = fs::File::open("inputs/day_6_1.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut links: Vec<&str> = contents.split("\n").collect();
    if links.len() == 0 {
        return Err(day_3_1::Error::FileFormat(format!("dev err: file has 0 lines")));
    }
    if links[links.len()-1].len() != 0 {
        return Err(day_3_1::Error::FileFormat(format!("file should end with an empty line")));
    }
    // We remove the final, empty, line.
    links.pop();

    Ok(num_orbits(links.as_slice()))
}

fn num_orbits(links: &[&str]) -> i32 {
    let pairs = links
        .iter()
        .enumerate()
        .map(|(i, link)| {
            let nodes: Vec<&str> = link.split(")").collect();
            if nodes.len() != 2 {
                panic!("line {}: link contained {} nodes: {}", i, nodes.len(), link)
            }
            nodes
        })
        .map(|nodes: Vec<&str>| (nodes[0], nodes[1]))
        .collect();

    let g = graph_from_pairs(pairs);

    count_leaf_depths(&g, &"COM")
}

type Graph<T> = collections::HashMap<T, Vec<T>>;

// TODO Implement From<Vec<(T, T)>> for Graph<T>.
fn graph_from_pairs<T: Eq + hash::Hash>(pairs: Vec<(T, T)>) -> Graph<T> {
    let mut g: Graph<T> = collections::HashMap::new();
    for (k, v) in pairs {
        let es: &mut Vec<T> = g.entry(k).or_insert(Vec::new());
        es.push(v);
    }
    g
}

fn count_leaf_depths(g: &Graph<&str>, origin: &str) -> i32 {
    fn count (g: &Graph<&str>, origin: &str, depth: i32) -> i32 {
        match g.get(origin) {
            None => depth,
            Some(nbrs) => {
                let nbr_depths: i32 = nbrs
                    .iter()
                    .map(|nbr: &&str| count(g, nbr, depth + 1))
                    .sum();

                depth + nbr_depths
            },
        }
    }

    count(g, origin, 0)
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
                ],
                42,
            ),
        ];
        for &(ref src, tgt) in &test_cases {
            assert_eq!(num_orbits(src), tgt);
        }
    }
}
