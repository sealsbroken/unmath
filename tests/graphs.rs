// SPDX-License-Identifier: LGPL-3.0-or-later
//! Structural checks for the published acyclic correction paths.
use std::collections::{BTreeMap, BTreeSet};
fn node(s: &str) -> String {
    s.trim()
        .chars()
        .take_while(|c| c.is_ascii_alphanumeric() || *c == '_')
        .collect()
}
fn reachable(start: &str, edges: &BTreeMap<String, Vec<String>>) -> BTreeSet<String> {
    let mut pending = vec![start.to_owned()];
    let mut seen = BTreeSet::new();
    while let Some(n) = pending.pop() {
        if seen.insert(n.clone()) {
            pending.extend(edges.get(&n).into_iter().flatten().cloned());
        }
    }
    seen
}
fn cycle(n: &str, edges: &BTreeMap<String, Vec<String>>, path: &mut BTreeSet<String>) -> bool {
    if !path.insert(n.into()) {
        return true;
    }
    let result = edges
        .get(n)
        .into_iter()
        .flatten()
        .any(|next| cycle(next, edges, path));
    path.remove(n);
    result
}
#[test]
fn every_question_graph_is_acyclic_with_reachable_nodes_and_success_rest_exits() {
    let dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("docs/wiki");
    let mut lessons = 0;
    for file in std::fs::read_dir(dir).unwrap() {
        let path = file.unwrap().path();
        if !path
            .file_name()
            .unwrap()
            .to_string_lossy()
            .starts_with("Lesson-")
        {
            continue;
        }
        lessons += 1;
        let page = std::fs::read_to_string(&path).unwrap();
        let graphs: Vec<_> = page.split("```mermaid\n").skip(1).collect();
        assert!(!graphs.is_empty(), "a rendered graph");
        for block in graphs {
            let graph = block.split("```").next().unwrap();
            let mut edges: BTreeMap<String, Vec<String>> = BTreeMap::new();
            let mut nodes = BTreeSet::new();
            for line in graph.lines() {
                if let Some((from, to)) = line.split_once("-->") {
                    let from = node(from);
                    let to = to.trim();
                    let to = if let Some(label) = to.strip_prefix('|') {
                        label.split_once('|').expect("closed edge label").1
                    } else {
                        to
                    };
                    let to = node(to);
                    assert!(
                        !from.is_empty() && !to.is_empty(),
                        "{}: {line}",
                        path.display()
                    );
                    nodes.insert(from.clone());
                    nodes.insert(to.clone());
                    edges.entry(from).or_default().push(to);
                }
            }
            assert!(nodes.contains("Choose"));
            assert!(
                !cycle("Choose", &edges, &mut BTreeSet::new()),
                "{} contains a directed cycle",
                path.display()
            );
            let visited = reachable("Choose", &edges);
            assert_eq!(visited, nodes, "unreachable nodes in {}", path.display());
            let success = ["Picnic", "Feast", "Done"]
                .into_iter()
                .find(|n| nodes.contains(*n))
                .expect("a success exit");
            assert!(nodes.contains("Rest"));
            for n in nodes
                .iter()
                .filter(|n| *n != "Rest" && n.as_str() != success)
            {
                let paths = reachable(n, &edges);
                assert!(
                    paths.contains("Rest") || paths.contains(success) || paths.contains("Continue"),
                    "dead end at {n} in {}",
                    path.display()
                );
            }
            assert!(!edges.contains_key("Rest"));
            assert!(!edges.contains_key(success));
        }
    }
    assert_eq!(lessons, 10);
}
