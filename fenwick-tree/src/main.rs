// -*- coding:utf-8-unix -*-

#[allow(dead_code)]
mod fenwick_tree {
    pub struct FenwickTree<T>
    where T: std::ops::AddAssign + Copy {
        n: usize,
        data: Vec<T>,
        e: T,
    }

    impl<T> FenwickTree<T>
    where T: std::ops::AddAssign + Copy {
        pub fn new(n: usize, e: T) -> Self {
            Self {
                n,
                data: vec![e; n],
                e,
            }
        }

        /// idx に v を加える
        pub fn add(&mut self, mut idx: usize, v: T) {
            idx += 1;
            while idx <= self.n {
                self.data[idx - 1] += v;
                idx += idx & idx.wrapping_neg();
            }
        }

        /// [0, idx) の和
        pub fn sum(&self, mut idx: usize) -> T {
            let mut res = self.e;
            while idx > 0 {
                res += self.data[idx - 1];
                idx &= idx - 1;
            }
            res
        }

        /// [l, r) の和
        pub fn range_sum(&self, l: usize, r: usize) -> T
        where T: std::ops::Sub<Output = T> {
            self.sum(r) - self.sum(l)
        }
    }
}

fn main() {
    // ## Point Add Range Sum
    // https://judge.yosupo.jp/problem/point_add_range_sum
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    s.trim_end().to_owned();
    let mut ws = s.split_whitespace();
    let n: usize = ws.next().unwrap().parse().unwrap();
    let q: usize = ws.next().unwrap().parse().unwrap();
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    s.trim_end().to_owned();
    let mut ws = s.split_whitespace();
    let a = (0..n)
        .map(|_| {
            let a: u64 = ws.next().unwrap().parse().unwrap();
            a
        })
        .collect::<Vec<_>>();
    enum Query {
        Q0(usize, u64),
        Q1(usize, usize),
    }
    let querys = (0..q)
        .map(|_| {
            let mut s = String::new();
            std::io::stdin().read_line(&mut s).unwrap();
            s.trim_end().to_owned();
            let mut ws = s.split_whitespace();
            let flag: usize = ws.next().unwrap().parse().unwrap();
            if flag == 0 {
                let i: usize = ws.next().unwrap().parse().unwrap();
                let x: u64 = ws.next().unwrap().parse().unwrap();
                Query::Q0(i, x)
            } else {
                let l: usize = ws.next().unwrap().parse().unwrap();
                let r: usize = ws.next().unwrap().parse().unwrap();
                Query::Q1(l, r)
            }
        })
        .collect::<Vec<_>>();
    /* -------------------------------------- */
    let mut fw = fenwick_tree::FenwickTree::new(n, 0u64);
    for (i, b) in a.into_iter().enumerate() {
        fw.add(i, b);
    }
    for query in querys {
        match query {
            Query::Q0(i, x) => fw.add(i, x),
            Query::Q1(l, r) => println!("{}", fw.range_sum(l, r)),
        }
    }
}
