use crate::Type::{Dir, Fil};
use helper::{aoc1, aoc2, InputReader};
use id_tree::InsertBehavior::{AsRoot, UnderNode};
use id_tree::{Node, NodeId, Tree, TreeBuilder};
use regex::Regex;
use std::cmp::min;

fn main() {
    aoc1!(Container, "day7", 95437);
    aoc2!(Container, "day7", 95437, 1491614, 24933642);
}

#[derive(Debug, PartialEq)]
enum Type {
    Dir,
    Fil,
}

#[derive(Debug)]
struct Leaf {
    name: String,
    size: usize,
    kind: Type,
}

struct Container {
    tree: Tree<Leaf>,
    position: NodeId,
    root_id: NodeId,
    regex_move: Regex,
    regex_dir: Regex,
    regex_file: Regex,
}

impl Default for Container {
    fn default() -> Self {
        let mut tree = TreeBuilder::new().build();
        let root_id: NodeId = tree
            .insert(
                Node::new(Leaf {
                    name: "/".to_string(),
                    size: 0,
                    kind: Dir,
                }),
                AsRoot,
            )
            .unwrap();
        Self {
            tree,
            position: root_id.clone(),
            root_id,
            regex_move: Regex::new(r"^\$ cd (.*)$").unwrap(),
            regex_dir: Regex::new(r"^dir (.*)$").unwrap(),
            regex_file: Regex::new(r"^([0-9]+) (.*)$").unwrap(),
        }
    }
}

impl InputReader for Container {
    fn on_start(&mut self) {}

    fn add_line(&mut self, line: &str) {
        if line == "$ cd /" {
            // self.position =tbd;
            return;
        }
        if line == "$ ls" {
            return;
        }
        if let Some(m) = self.regex_move.captures(line) {
            let node = self.tree.get(&self.position).unwrap();
            if &m[1] == ".." {
                self.position = node.parent().unwrap().clone();
            } else {
                for c in node.children() {
                    let children = self.tree.get(c).unwrap().data();
                    if children.name == m[1].to_string() {
                        self.position = c.clone();
                    }
                }
            }
        }
        if let Some(dir) = self.regex_dir.captures(line) {
            self.tree
                .insert(
                    Node::new(Leaf {
                        name: dir[1].to_string(),
                        size: 0,
                        kind: Dir,
                    }),
                    UnderNode(&self.position),
                )
                .unwrap();
        }
        if let Some(file) = self.regex_file.captures(line) {
            let file_size: usize = file[1].parse().unwrap();
            let new_node = self
                .tree
                .insert(
                    Node::new(Leaf {
                        name: file[2].to_string(),
                        size: file_size,
                        kind: Fil,
                    }),
                    UnderNode(&self.position),
                )
                .unwrap();

            let ancestors = self.tree.ancestor_ids(&new_node).unwrap();

            let nodes: Vec<NodeId> = ancestors.into_iter().map(|a| a.clone()).collect();

            for n in nodes {
                let current = self.tree.get_mut(&n).unwrap();
                let data = current.data_mut();
                data.size += file_size;
            }
        }
    }

    fn star1(self) -> String {
        let mut sum_size = 0;

        for node in self.tree.traverse_level_order(&self.root_id).unwrap() {
            let data = node.data();
            if data.kind == Dir && data.size < 100000 {
                sum_size += data.size
            }
        }

        sum_size.to_string()
    }

    fn star2(self) -> String {
        let root = self.tree.get(&self.root_id).unwrap();
        let data = root.data();

        let free = 70_000_000 - data.size;
        let to_remove = 30_000_000 - free;

        let mut smallest = 70_000_000;

        for node in self.tree.traverse_level_order(&self.root_id).unwrap() {
            let data = node.data();
            if data.kind == Dir && data.size > to_remove {
                smallest = min(smallest, data.size);
            }
        }

        smallest.to_string()
    }
}
