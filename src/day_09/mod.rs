//! --- Day 9: All in a Single Night ---
//! Every year, Santa manages to deliver all of his presents in a single night.
//! 
//! This year, however, he has some new locations to visit; his elves have provided him
//! the distances between every pair of locations. He can start and end at any two
//! (different) locations he wants, but he must visit each location exactly once. What is
//! the shortest distance he can travel to achieve this?
//! 
//! For example, given the following distances:
//! 
//! ```
//! London to Dublin = 464
//! London to Belfast = 518
//! Dublin to Belfast = 141
//! ```
//! The possible routes are therefore:
//! 
//! ```
//! Dublin -> London -> Belfast = 982
//! London -> Dublin -> Belfast = 605
//! London -> Belfast -> Dublin = 659
//! Dublin -> Belfast -> London = 659
//! Belfast -> Dublin -> London = 605
//! Belfast -> London -> Dublin = 982
//! ```
//! The shortest of these is `London -> Dublin -> Belfast = 605`, and so the answer is
//! `605` in this example.
//! 
//! What is the distance of the shortest route?
//!
//! --- Part Two ---
//! The next year, just to show off, Santa decides to take the route with the longest
//! distance instead.
//! 
//! He can still start and end at any two (different) locations he wants, and he still
//! must visit each location exactly once.
//! 
//! For example, given the distances above, the longest route would be `982` via
//! (for example) `Dublin -> London -> Belfast`.
//! 
//! What is the distance of the longest route?

use std::collections::HashMap;

#[derive(Debug)]
struct Edge {
    from: String,
    to: String,
    distance: usize,
}


#[derive(Clone, Debug, Eq, PartialEq)]
struct Route {
    stops: Vec<String>,
    distance: usize,
}


impl Edge {
    pub fn permutations(&self) -> [(&str, &str); 2] {
        [(self.from.as_str(), self.to.as_str()), 
         (self.to.as_str(), self.from.as_str())]
    }
}

impl TryFrom<&String> for Edge {
    type Error = String;
    fn try_from(value: &String) -> Result<Self, Self::Error> {
        let splits: Vec<&str> = value.split(" = ").collect();
        if splits.len() != 2 { return Err(String::from("invalid input for edge")) }
        let (lhs, rhs) = (splits[0], splits[1]);
        let lhs_splits: Vec<&str> = lhs.split(" to ").collect();
        if lhs_splits.len() != 2 { return Err(format!("invalid lhs for edge: {}", lhs)) }
        let (from, to) = (lhs_splits[0].to_owned(), lhs_splits[1].to_owned());
        let distance: usize = rhs.parse().map_err(|e| String::from("invalid distance"))?;
        Ok(Self { from, to, distance })
    }
}


fn edges_from_input(input: &[String]) -> Result<Vec<Edge>, String> {
    let out: Result<Vec<Edge>, String> = input
        .into_iter()
        .map(|s| s.try_into())
        .collect();
    out
}


fn backtrack(
    paths: &mut Vec<Route>,
    route: &mut Route,
    graph: &HashMap<String, Vec<(String, usize)>>,
)
    -> Result<(), String>
{
    if route.stops.is_empty() {
        return Err(String::from("backtrack should always receive a non-empty path"))
    }
    if route.stops.len() == graph.len() {
        paths.push(route.clone());
        return Ok(())
    }
    let current = route.stops.last().unwrap();
    for (v, dist) in graph.get(current).expect("all vertices should have a key in graph") {
        if route.stops.contains(v) { continue }
        route.stops.push(v.to_owned());
        route.distance += dist;
        backtrack(paths, route, graph)?;
        route.stops.pop();
        route.distance -= dist;
    }
    Ok(())
}


fn get_hamiltonian_paths(graph: &HashMap<String, Vec<(String, usize)>>)
    -> Result<Vec<Route>, String>
{
    let mut paths = vec![];
    for start in graph.keys() {
        let mut route = Route { stops: vec![start.to_owned()], distance: 0 };
        backtrack(&mut paths, &mut route, graph)?;
    }
    return Ok(paths)
}


fn graph_from_edges(edges: &[Edge]) -> HashMap<String, Vec<(String, usize)>> {
    let mut out = HashMap::<String, Vec<(String, usize)>>::new();
    for e in edges {
        out.entry(e.from.clone())
            .and_modify(|v| v.push((e.to.clone(), e.distance)))
            .or_insert(vec![(e.to.clone(), e.distance)]);
        out.entry(e.to.clone())
            .and_modify(|v| v.push((e.from.clone(), e.distance)))
            .or_insert(vec![(e.from.clone(), e.distance)]);
    }
    out
}


fn part_1(input: &[String]) -> Result<Option<usize>, String> {
    let edges = edges_from_input(input)?;
    let graph = graph_from_edges(&edges);
    let paths = get_hamiltonian_paths(&graph)?;
    let min_route = paths.iter().min_by(|a, b| a.distance.cmp(&b.distance));
    match min_route {
        Some(route) => Ok(Some(route.distance)),
        None => Ok(None)
    }
}


fn part_2(input: &[String]) -> Result<Option<usize>, String> {
    let edges = edges_from_input(input)?;
    let graph = graph_from_edges(&edges);
    let paths = get_hamiltonian_paths(&graph)?;
    let max_route = paths.iter().max_by(|a, b| a.distance.cmp(&b.distance));
    match max_route {
        Some(route) => Ok(Some(route.distance)),
        None => Ok(None)
    }
}


#[cfg(test)]
mod tests {
    use std::{path::Path, fs::File, io::{BufReader, BufRead}};

    fn load_input(fname: &str) -> std::io::Result<Vec<String>> {
        let path = Path::new(file!())
            .parent()
            .expect("every file has a parent")
            .join(fname);
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        reader.lines().collect()
    }

    #[test]
    fn get_routes() {
        use super::Route;
        let input = load_input("input_sm.txt").unwrap();
        let edges = super::edges_from_input(&input).unwrap();
        let graph = super::graph_from_edges(&edges);
        let mut output = super::get_hamiltonian_paths(&graph).unwrap();
        output.sort_unstable_by(
            |a, b| match a.distance.cmp(&b.distance) {
                std::cmp::Ordering::Equal => a.stops.cmp(&b.stops),
                other => other,
            });
        let expected = vec![
            Route {
                stops: vec!["Belfast".into(), "Dublin".into(), "London".into()],
                distance: 605,
            },
            Route {
                stops: vec!["London".into(), "Dublin".into(), "Belfast".into()],
                distance: 605,
            },
            Route {
                stops: vec!["Dublin".into(), "Belfast".into(), "London".into()],
                distance: 659,
            },
            Route {
                stops: vec!["London".into(), "Belfast".into(), "Dublin".into()],
                distance: 659,
            },
            Route {
                stops: vec!["Belfast".into(), "London".into(), "Dublin".into()],
                distance: 982,
            },
            Route {
                stops: vec!["Dublin".into(), "London".into(), "Belfast".into()],
                distance: 982,
            },
        ];
        assert_eq!(output, expected);
    }

    #[test]
    fn part_1_sm() {
        let input = load_input("input_sm.txt").unwrap();
        let output = super::part_1(&input)
            .unwrap()
            .unwrap();
        assert_eq!(output, 605);
    }

    #[test]
    fn part_1() {
        let input = load_input("input.txt").unwrap();
        let output = super::part_1(&input)
            .unwrap()
            .unwrap();
        assert_eq!(output, 141);
    }

    #[test]
    fn part_2() {
        let input = load_input("input.txt").unwrap();
        let output = super::part_2(&input)
            .unwrap()
            .unwrap();
        assert_eq!(output, 736);
    }
}
