// -*- coding:utf-8-unix -*-

#[allow(dead_code)]
mod max_flow {
    struct _Edge {
        to: usize,
        rev: usize,
        cap: u64,
    }
    pub struct MfGraph {
        n: usize,
        pos: Vec<(usize, usize)>,
        g: Vec<Vec<_Edge>>
    }
    impl MfGraph {
        pub fn new(n: usize) -> MfGraph {
            MfGraph {
                n,
                pos: Vec::new(),
                g: (0..n).map(|_| Vec::new()).collect(),
            }
        }
        pub fn add_edge(&mut self, from: usize, to: usize, cap: u64) {
            let rev = self.g[to].len();
            self.g[from].push(_Edge {to, cap, rev});
            let rev = self.g[from].len() - 1;
            self.g[to].push(_Edge {to: from, cap: 0, rev});
        }
        pub fn flow(&mut self, s: usize, t: usize) -> u64 {
            self.flow_with_cap(s, t, u64::max_value())
        }
        pub fn flow_with_cap(&mut self, s: usize, t: usize, limit: u64) -> u64 {
            assert!(s < self.n && t < self.n);
            let n = self.n;
            let mut solver = FlowSolver {
                graph: self,
                s,
                t,
                limit,
                level: vec![0; n],
                iter: vec![0; n],
            };
            let mut flow = 0;
            while flow < limit {
                solver.bfs();
                if solver.level[t] == -1 {
                    break;
                }
                solver.iter.iter_mut().for_each(|e| *e = 0);
                while flow < limit {
                    let f = solver.dfs(s, limit - flow);
                    if f == 0 {
                        break;
                    }
                    flow += f;
                }
            }
            flow
        }
        pub fn min_cut(&mut self, s: usize) -> Vec<bool> {
            let mut visited = vec![false; self.n];
            let mut que = std::collections::VecDeque::new();
            que.push_back(s);
            while let Some(p) = que.pop_front() {
                visited[p] = true;
                for e in &self.g[p] {
                    if e.cap != 0 && !visited[e.to] {
                        visited[e.to] = true;
                        que.push_back(e.to);
                    }
                }
            }
            visited
        }
    }
    struct FlowSolver<'a> {
        graph: &'a mut MfGraph,
        s: usize,
        t: usize,
        limit: u64,
        level: Vec<i32>,
        iter: Vec<usize>,
    }
    impl FlowSolver<'_> {
        fn bfs(&mut self) {
            self.level.iter_mut().for_each(|e| *e = -1);
            self.level[self.s] = 0;
            let mut que = std::collections::VecDeque::<usize>::new();
            que.push_back(self.s);
            while let Some(now) = que.pop_front() {
                for &_Edge {to, cap, ..} in &self.graph.g[now] {
                    if cap <= 0 || self.level[to] >= 0 {continue;}
                    self.level[to] = self.level[now] + 1;
                    if self.t == to {
                        return;
                    }
                    que.push_back(to);
                }
            }
        }
        fn dfs(&mut self, v: usize, up: u64) -> u64 {
            if v == self.t {
                return up;
            }
            for i in self.iter[v]..self.graph.g[v].len() {
                self.iter[v] = i;
                let &_Edge {
                    to,
                    rev,
                    cap
                } = &self.graph.g[v][i];
                if cap <= 0 || self.level[v] >= self.level[to] {
                    continue;
                }
                let d = self.dfs(to, up.min(cap));
                if d <= 0 {
                    continue;
                }
                self.graph.g[v][i].cap -= d;
                self.graph.g[to][rev].cap += d;
                return d;
            }
            self.iter[v] = self.graph.g[v].len();
            0
        }
    }
}

use proconio::input;

fn main() {
    // 典型90 040 Get More Money
    // https://atcoder.jp/contests/typical90/tasks/typical90_an
    input! {
        n: usize,
        w: u64,
        a: [u64; n],
    }
    let mut flow = max_flow::MfGraph::new(n+2);
    for i in 1..=n {
        input! {
            k: usize,
            c: [usize; k],
        }
        flow.add_edge(0, i, a[i-1]);
        flow.add_edge(i, n+1, w);
        for c in c {
            flow.add_edge(c, i, u64::max_value())
        }
    }
    println!("{}", a.into_iter().sum::<u64>() - flow.flow(0, n+1));
}
