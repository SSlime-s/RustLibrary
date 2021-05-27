// -*- coding:utf-8-unix -*-

#[allow(dead_code)]
mod seg_tree {
    pub struct SegTree<T, F>
    where
        F: Fn(&T, &T) -> T,
    {
        datas: Vec<T>,
        op: F,
        zero: T,
        n: usize,
    }
    impl<T, F> SegTree<T, F>
    where
        T: Clone + Copy,
        F: Fn(&T, &T) -> T,
    {
        pub fn new(n_: usize, op: F, zero: T) -> SegTree<T, F>
        where
            T: Clone,
        {
            let mut n = 1;
            while n < n_ {
                n <<= 1
            }
            SegTree::<T, F> {
                datas: vec![zero; 2 * n + 1],
                op,
                zero,
                n,
            }
        }

        pub fn from(a: &Vec<T>, op: F, zero: T) -> SegTree<T, F> {
            let mut n = 1;
            while n < a.len() {
                n <<= 1
            }
            let a = (0..n)
                .map(|idx| if idx < a.len() { a[idx] } else { zero })
                .collect::<Vec<_>>();
            let mut datas = vec![zero; 2 * n + 1];
            for i in 0..n {
                datas[i + n] = a[i];
            }
            for i in (1..n).rev() {
                datas[i] = op(&datas[2 * i], &datas[2 * i + 1]);
            }
            SegTree::<T, F> {
                datas,
                op,
                zero,
                n,
            }
        }

        pub fn update(&mut self, idx: usize, x: T) {
            let mut i = idx + self.n;
            self.datas[i] = (self.op)(&self.datas[i], &x);
            loop {
                i >>= 1;
                if i <= 0 {
                    break;
                }
                self.datas[i] = (self.op)(&self.datas[2 * i], &self.datas[2 * i + 1]);
            }
        }

        pub fn change(&mut self, idx: usize, x: T) {
            let mut i = idx + self.n;
            self.datas[i] = x;
            loop {
                i >>= 1;
                if i <= 0 {
                    break;
                }
                self.datas[i] = (self.op)(&self.datas[2 * i], &self.datas[2 * i + 1]);
            }
        }

        pub fn get(&self, l: usize, r: usize) -> T {
            if l == r {
                self.zero
            } else if r > l {
                self.get_recu(l, r, 1, 0, self.n)
            } else {
                self.get_recu(r, l, 1, 0, self.n)
            }
        }

        fn get_recu(&self, l: usize, r: usize, node: usize, now_l: usize, now_r: usize) -> T {
            if l >= now_r || r <= now_l {
                return self.zero;
            }
            if l <= now_l && now_r <= r {
                return self.datas[node];
            }
            let left = self.get_recu(l, r, node * 2, now_l, (now_l + now_r) / 2);
            let right = self.get_recu(l, r, node * 2 + 1, (now_l + now_r) / 2, now_r);
            (self.op)(&left, &right)
        }
    }
}
#[allow(unused_must_use)]
fn main() {
    // ## static RMQ
    // let mut s = String::new();
    // std::io::stdin().read_line(&mut s).unwrap();
    // s.trim_end().to_owned();
    // let mut ws = s.split_whitespace();
    // let n: usize = ws.next().unwrap().parse().unwrap();
    // let q: usize = ws.next().unwrap().parse().unwrap();
    // let mut s = String::new();
    // std::io::stdin().read_line(&mut s).unwrap();
    // s.trim_end().to_owned();
    //
    // let mut ws = s.split_whitespace();
    // let a = (0..n)
    //     .map(|_| {
    //         let a: i64 = ws.next().unwrap().parse().unwrap();
    //         a
    //     })
    //     .collect::<Vec<_>>();
    // let lr = (0..q)
    //     .map(|_| {
    //         let mut s = String::new();
    //         std::io::stdin().read_line(&mut s).unwrap();
    //         s.trim_end().to_owned();
    //         let mut ws = s.split_whitespace();
    //         let l: usize = ws.next().unwrap().parse().unwrap();
    //         let r: usize = ws.next().unwrap().parse().unwrap();
    //         (l, r)
    //     })
    //     .collect::<Vec<_>>();
    // let rmq = seg_tree::SegTree::<_, _>::from(&a, |&a, &b| -> i64 {a.min(b)}, 1_000_000_001i64);
    // for (l, r) in lr {
    //     println!("{}", rmq.get(l, r))
    // }

    // ## Point Add Range Sum
    // let mut s = String::new();
    // std::io::stdin().read_line(&mut s).unwrap();
    // s.trim_end().to_owned();
    // let mut ws = s.split_whitespace();
    // let n: usize = ws.next().unwrap().parse().unwrap();
    // let q: usize = ws.next().unwrap().parse().unwrap();
    // let mut s = String::new();
    // std::io::stdin().read_line(&mut s).unwrap();
    // s.trim_end().to_owned();
    // let mut ws = s.split_whitespace();
    // let a = (0..n)
    //     .map(|_| {
    //         let a: i64 = ws.next().unwrap().parse().unwrap();
    //         a
    //     })
    //     .collect::<Vec<_>>();
    // enum Query {
    //     Q0(usize, i64),
    //     Q1(usize, usize),
    // }
    // let querys = (0..q)
    //     .map(|_| {
    //         let mut s = String::new();
    //         std::io::stdin().read_line(&mut s).unwrap();
    //         s.trim_end().to_owned();
    //         let mut ws = s.split_whitespace();
    //         let flag: usize = ws.next().unwrap().parse().unwrap();
    //         if flag == 0 {
    //             let i: usize = ws.next().unwrap().parse().unwrap();
    //             let x: i64 = ws.next().unwrap().parse().unwrap();
    //             Query::Q0(i, x)
    //         } else {
    //             let l: usize = ws.next().unwrap().parse().unwrap();
    //             let r: usize = ws.next().unwrap().parse().unwrap();
    //             Query::Q1(l, r)
    //         }
    //     })
    //     .collect::<Vec<_>>();
    // // -----------------------------
    // let mut rmq = seg_tree::SegTree::<_, _>::from(&a, |&a, &b| -> i64 {a + b}, 0);
    // for query in querys {
    //     match query {
    //         Query::Q0(i, x) => rmq.update(i, x),
    //         Query::Q1(l, r) => println!("{}", rmq.get(l, r))
    //     }
    // }

    // ## Point Set Range Composite
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    s.trim_end().to_owned();
    let mut ws = s.split_whitespace();
    let n: usize = ws.next().unwrap().parse().unwrap();
    let q: usize = ws.next().unwrap().parse().unwrap();
    const MOD: u64 = 998244353;
    let ab = (0..n)
        .map(|_| {
            let mut s = String::new();
            std::io::stdin().read_line(&mut s).unwrap();
            s.trim_end().to_owned();
            let mut ws = s.split_whitespace();
            let a: u64 = ws.next().unwrap().parse().unwrap();
            let b: u64 = ws.next().unwrap().parse().unwrap();
            (a, b)
        })
        .collect::<Vec<_>>();
    enum Query {
        Q0(usize, u64, u64),
        Q1(usize, usize, u64),
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
                let c: u64 = ws.next().unwrap().parse().unwrap();
                let d: u64 = ws.next().unwrap().parse().unwrap();
                Query::Q0(i, c, d)
            } else {
                let l: usize = ws.next().unwrap().parse().unwrap();
                let r: usize = ws.next().unwrap().parse().unwrap();
                let x: u64 = ws.next().unwrap().parse().unwrap();
                Query::Q1(l, r, x)
            }
        })
        .collect::<Vec<_>>();
    // -----------------------------
    let mut rmq = seg_tree::SegTree::<_, _>::from(
        &ab,
        |&(a, b), &(c, d)| -> (u64, u64) { (a * c % MOD, (c * b % MOD + d) % MOD) },
        (1, 0),
    );
    for query in querys {
        match query {
            Query::Q0(i, c, d) => rmq.change(i, (c, d)),
            Query::Q1(l, r, x) => {
                let (a, b) = rmq.get(l, r);
                println!("{}", (a * x % MOD + b) % MOD)
            }
        }
    }
}
