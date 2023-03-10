use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize)]
#[derive(Debug)]
pub struct Graph {
    graph: Vec<Vec<bool>>,
}

impl Graph {
    pub fn new(sz: usize) -> Self {
        let mut graph: Vec<Vec<bool>> = Vec::new();

        graph.resize_with(sz, || { 
            let mut g: Vec<bool> = Vec::new();
            g.resize_with(sz, || { false } );
            g
         });

        Self {
            graph,
        }
    }

    pub fn connect(&mut self, v1: usize, v2: usize) {
        assert!(!self.graph.is_empty());
        assert!(v1 < self.graph[0].len() && v2 < self.graph[0].len());

        self.graph[v1][v2] = true;
    }

    pub fn connected(&self, v1: usize, v2: usize) -> bool {
        assert!(!self.graph.is_empty());
        assert!(v1 < self.graph[0].len() && v2 < self.graph[0].len());

        self.graph[v1][v2]
    }
}

