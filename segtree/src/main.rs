// -*- coding:utf-8-unix -*-

#[allow(dead_code)]
mod segtree {
    pub trait Monoid {
        type S: Clone;
        fn op(a: &Self::S, b: &Self::S) -> Self::S;
        fn id() -> Self::S;
    }

    pub struct SegTree<M>
    where M: Monoid {
        datas: Vec<M::S>,
        n: usize,
        size: usize,
        log: usize,
    }
    impl<M: Monoid> SegTree<M> {
        pub fn new(n: usize) -> Self {
            vec![M::id(); n].into()
        }

        pub fn set(&mut self, mut p: usize, x: M::S) {
            assert!(p < self.n);
            p += self.size;
            self.datas[p] = x;
            for i in 1..=self.log {
                self.update(p >> i);
            }
        }

        pub fn get(&self, p: usize) -> M::S {
            assert!(p < self.n);
            self.datas[p + self.size].clone()
        }

        pub fn prod(&self, l: usize, r: usize) -> M::S {
            assert!(l <= r && r <= self.n);
            if l == r {
                M::id()
            } else {
                self.prod_recu(l, r, 1, 0, self.size)
            }
        }

        fn prod_recu(&self, l: usize, r: usize, node: usize, now_l: usize, now_r: usize) -> M::S {
            if l >= now_r || r <= now_l {
                M::id()
            } else if l <= now_l && now_r <= r {
                self.datas[node].clone()
            } else {
                let left = self.prod_recu(l, r, node*2, now_l, (now_l + now_r)/2);
                let right = self.prod_recu(l, r, node*2 + 1, (now_l + now_r)/2, now_r);
                M::op(&left, &right)
            }
        }

        pub fn all_prod(&self) -> M::S {
            self.datas[1].clone()
        }

        // TODO: 二分探索 (max_right, max_left)

        fn update(&mut self, i: usize) {
            self.datas[i] = M::op(&self.datas[2*i], &self.datas[2*i + 1]);
        }
    }
    impl<M: Monoid> From<Vec<M::S>> for SegTree<M> {
        fn from(v: Vec<M::S>) -> Self {
            let n = v.len();
            let log = (32 - (n as u32).saturating_sub(1).leading_zeros()) as usize;
            let size = 1 << log;
            let mut datas = vec![M::id(); 2*size];
            datas[size..(size+n)].clone_from_slice(&v);
            let mut seg = Self {n, log, size, datas};
            for i in (1..size).rev() {
                seg.update(i)
            }
            seg
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
    // // ---------------------------
    // struct Min {};
    // impl segtree::Monoid for Min {
    //     type S = i64;
    //     fn op(a: &Self::S, b: &Self::S) -> Self::S {
    //         *a.min(b)
    //     }
    //     fn id() -> Self::S {
    //         i64::max_value()
    //     }
    // }
    // let seg: segtree::SegTree<Min> = a.clone().into();
    // for (l, r) in lr {
    //     println!("{}", seg.prod(l, r));
    // }

    // ## Point Add Range Sum
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
    // -----------------------------
    struct Sum {};
    impl segtree::Monoid for Sum {
        type S = u64;
        fn op(a: &Self::S, b: &Self::S) -> Self::S {
            *a + *b
        }
        fn id() -> Self::S {
            0
        }
    }
    let mut seg: segtree::SegTree<Sum> = a.clone().into();
    for query in querys {
        match query {
            Query::Q0(i, x) => seg.set(i, seg.get(i) + x),
            Query::Q1(l, r) => println!("{}", seg.prod(l, r)),
        }
    }

    // ## Point Set Range Composite
    // let mut s = String::new();
    // std::io::stdin().read_line(&mut s).unwrap();
    // s.trim_end().to_owned();
    // let mut ws = s.split_whitespace();
    // let n: usize = ws.next().unwrap().parse().unwrap();
    // let q: usize = ws.next().unwrap().parse().unwrap();
    // const MOD: u64 = 998244353;
    // let ab = (0..n)
    //     .map(|_| {
    //         let mut s = String::new();
    //         std::io::stdin().read_line(&mut s).unwrap();
    //         s.trim_end().to_owned();
    //         let mut ws = s.split_whitespace();
    //         let a: u64 = ws.next().unwrap().parse().unwrap();
    //         let b: u64 = ws.next().unwrap().parse().unwrap();
    //         (a, b)
    //     })
    //     .collect::<Vec<_>>();
    // enum Query {
    //     Q0(usize, u64, u64),
    //     Q1(usize, usize, u64),
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
    //             let c: u64 = ws.next().unwrap().parse().unwrap();
    //             let d: u64 = ws.next().unwrap().parse().unwrap();
    //             Query::Q0(i, c, d)
    //         } else {
    //             let l: usize = ws.next().unwrap().parse().unwrap();
    //             let r: usize = ws.next().unwrap().parse().unwrap();
    //             let x: u64 = ws.next().unwrap().parse().unwrap();
    //             Query::Q1(l, r, x)
    //         }
    //     })
    //     .collect::<Vec<_>>();
    // // -----------------------------
    // struct Func {};
    // impl segtree::Monoid for Func {
    //     type S = (u64, u64);
    //     fn op(a: &Self::S, b: &Self::S) -> Self::S {
    //         (a.0 * b.0 % MOD, (b.0 * a.1 % MOD + b.1) % MOD)
    //     }
    //     fn id() -> Self::S {
    //         (1, 0)
    //     }
    // }
    // let mut seg: segtree::SegTree<Func> = ab.clone().into();
    // for query in querys {
    //     match query {
    //         Query::Q0(i, c, d) => seg.set(i, (c, d)),
    //         Query::Q1(l, r, x) => {
    //             let (a, b) = seg.prod(l, r) as (u64, u64);
    //             println!("{}", (a * x % MOD + b) % MOD);
    //         }
    //     }
    // }
}
