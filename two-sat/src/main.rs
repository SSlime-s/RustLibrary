// -*- coding:utf-8-unix -*-

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


use std::collections::{HashMap};

use proconio::input;

const MAX: usize = 2_000_000;

struct LinearPrime {
    pub primes: Vec<usize>,
    pub divisor: Vec<usize>,
}
impl LinearPrime {
    pub fn new(n: usize) -> LinearPrime {
        let mut divisor = vec![0; n+1];
        let mut primes = Vec::new();
        divisor[1] = 1;
        for i in 2..=n {
            if divisor[i] == 0 {
                divisor[i] = i;
                primes.push(i);
            }
            for &p in &primes {
                if p * i > n || p > divisor[i] {
                    break;
                }
                divisor[p * i] = p;
            }
        }
        LinearPrime {
            primes,
            divisor,
        }
    }
}

fn main() {
    // ABC 210 F Coprime Solitaire
    // https://atcoder.jp/contests/abc210/tasks/abc210_f
    input! {
        n: usize,
        ab: [(usize, usize); n],
    }
    let lp = LinearPrime::new(MAX);
    let mut map = HashMap::new();
    let mut cnt = 0;
    for (i, &(a, b)) in (&ab).into_iter().enumerate() {
        let mut a = a;
        let mut b = b;
        while a > 1 {
            let p = lp.divisor[a];
            map.entry(p).or_insert_with(|| Vec::new()).push((i, true));
            cnt += 1;
            while a % p == 0 {
                a /= p;
            }
        }
        while b > 1 {
            let p = lp.divisor[b];
            map.entry(p).or_insert_with(|| Vec::new()).push((i, false));
            cnt += 1;
            while b % p == 0 {
                b /= p;
            }
        }
    }
    let mut ts = two_sat::TwoSat::new(n + cnt);
    let mut done = n-1;
    for (_, v) in map {
        for &(i, f) in &v {
            if v.first().unwrap() != &(i, f) {
                ts.add_clause(done, false, done+1, true);
                ts.add_clause(done, false, i, !f);
            }
            ts.add_clause(i, !f, done+1, true);
            done += 1;
        }
    }
    println!("{}", if ts.satisfiable() {"Yes"} else {"No"});
}
