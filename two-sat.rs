#[allow(unused)]
mod two_sat {
    use petgraph::graph::{Graph, NodeIndex};
    use petgraph::algo::kosaraju_scc;

    pub struct TwoSat {
        n: usize,
        graph: Graph<(), ()>,
        nodes: Vec<NodeIndex>,
        pub answer: Vec<bool>,
    }
    impl TwoSat {
        pub fn new(n: usize) -> TwoSat {
            let mut graph = Graph::<(), ()>::new();
            let nodes = (0..2*n).map(|_| graph.add_node(())).collect::<Vec<NodeIndex>>();
            TwoSat {
                n,
                graph,
                nodes,
                answer: vec![false; n],
            }
        }

        pub fn add_clause(&mut self, i: usize, f: bool, j: usize, g: bool) {
            self.graph.add_edge(self.nodes[2*i + if f {0} else {1}], self.nodes[2*j + if g {1} else {0}], ());
            self.graph.add_edge(self.nodes[2*j + if g {0} else {1}], self.nodes[2*i + if f {1} else {0}], ());
        }

        pub fn satisfiable(&mut self) -> bool {
            let scc = kosaraju_scc(&self.graph);
            let mut id_map = vec![0; 2*self.n];
            for (i, s) in scc.into_iter().enumerate() {
                for &v in &s {
                    if v.index() < 2*self.n {
                        id_map[v.index()] = i;
                    }
                }
            }

            for i in 0..self.n {
                if id_map[2*i] == id_map[2*i+1] {
                    return false;
                }
                self.answer[i] = id_map[2*i] < id_map[2*i + 1];
            }
            true
        }
    }
}
