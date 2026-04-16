use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Clone, Debug)]
pub struct Node {
    pub id: String,
}

pub struct GraphRouter {
    pub graph: HashMap<String, Vec<(String, f64)>>, // neighbor, cost
}

impl GraphRouter {
    pub fn new() -> Self {
        Self {
            graph: HashMap::new(),
        }
    }

    pub fn add_link(&mut self, from: &str, to: &str, cost: f64) {
        self.graph
            .entry(from.to_string())
            .or_default()
            .push((to.to_string(), cost));
    }

    pub fn neighbors(&self, node: &str) -> Option<&Vec<(String, f64)>> {
        self.graph.get(node)
    }

    // Simple BFS fallback (not weighted optimal yet)
    pub fn find_path(&self, start: &str, goal: &str) -> Option<Vec<String>> {
        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();
        let mut parent: HashMap<String, String> = HashMap::new();

        queue.push_back(start.to_string());
        visited.insert(start.to_string());

        while let Some(node) = queue.pop_front() {
            if node == goal {
                let mut path = vec![goal.to_string()];
                let mut current = goal.to_string();

                while let Some(p) = parent.get(&current) {
                    path.push(p.clone());
                    current = p.clone();
                }

                path.reverse();
                return Some(path);
            }

            if let Some(neigh) = self.graph.get(&node) {
                for (n, _) in neigh {
                    if !visited.contains(n) {
                        visited.insert(n.clone());
                        parent.insert(n.clone(), node.clone());
                        queue.push_back(n.clone());
                    }
                }
            }
        }

        None
    }
}
