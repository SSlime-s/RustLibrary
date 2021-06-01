// -*- coding:utf-8-unix -*-

#[allow(dead_code)]
mod lazy_segtree {
    pub trait Monoid {
        type S: Clone;
        fn id() -> Self::S;
        fn op(a: &Self::S, b: &Self::S) -> Self::S;
    }
    pub trait MapMonoid {
        type M: Monoid;
        type F: Clone;
        fn id_element() -> <Self::M as Monoid>::S {
            Self::M::id()
        }
        fn op(a: &<Self::M as Monoid>::S, b: &<Self::M as Monoid>::S) -> <Self::M as Monoid>::S {
            Self::M::op(a, b)
        }
        fn id_map() -> Self::F;
        fn mapping(f: &Self::F, x: &<Self::M as Monoid>::S) -> <Self::M as Monoid>::S;
        fn composition(f: &Self::F, g: &Self::F) -> Self::F;
    }

    pub struct LazySegtree<F>
    where
        F: MapMonoid,
    {
        n: usize,
        size: usize,
        log: usize,
        datas: Vec<<F::M as Monoid>::S>,
        lazy: Vec<F::F>,
    }
    impl<F: MapMonoid> From<Vec<<F::M as Monoid>::S>> for LazySegtree<F> {
        fn from(v: Vec<<F::M as Monoid>::S>) -> Self {
            let n = v.len();
            let log = (32 - (n as u32).saturating_sub(1).leading_zeros()) as usize;
            let size = 1 << log;
            let mut datas = vec![F::id_element(); size * 2];
            let lazy = vec![F::id_map(); size * 2];
            datas[size..(size + n)].clone_from_slice(&v);
            let mut lz = Self {
                n,
                size,
                log,
                datas,
                lazy,
            };
            for i in (1..size).rev() {
                lz.update(i);
            }
            lz
        }
    }
    impl<F: MapMonoid> LazySegtree<F> {
        pub fn new(n: usize) -> Self {
            vec![F::id_element(); n].into()
        }

        pub fn set(&mut self, mut p: usize, x: <F::M as Monoid>::S) {
            assert!(p < self.n);
            p += self.size;
            for i in (1..=self.log).rev() {
                self.push(p >> i);
            }
            self.datas[p] = x;
            for i in 1..=self.log {
                self.update(p >> i);
            }
        }

        pub fn prod(&mut self, l: usize, r: usize) -> <F::M as Monoid>::S {
            assert!(l <= r && r <= self.n);
            if l == r {
                F::id_element()
            } else {
                self.prod_recu(l, r, 1, 0, self.size)
            }
        }
        fn prod_recu(
            &mut self,
            l: usize,
            r: usize,
            node: usize,
            now_l: usize,
            now_r: usize,
        ) -> <F::M as Monoid>::S {
            if l >= now_r || r <= now_l {
                F::id_element()
            } else if l <= now_l && now_r <= r {
                self.push(node);
                self.datas[node].clone()
            } else {
                self.push(node);
                let left = self.prod_recu(l, r, node*2, now_l, (now_l + now_r)/2);
                let right = self.prod_recu(l, r, node*2 + 1, (now_l + now_r)/2, now_r);
                F::op(&left, &right)
            }
        }

        pub fn apply_range(&mut self, l: usize, r: usize, f: F::F) {
            assert!(l <= r && r <= self.n);
            if l == r {
                return;
            }
            self.apply_range_recu(l, r, 1, 0, self.size, f);
        }
        fn apply_range_recu(&mut self, l: usize, r: usize, node: usize, now_l: usize, now_r: usize, f: F::F) {
            self.push(node);
            if r <= now_l || now_r <= l {
                return;
            }
            if l <= now_l && now_r <= r {
                self.all_apply(node, f);
                self.push(node);
            } else {
                self.apply_range_recu(l, r, node*2, now_l, (now_l + now_r) / 2, f.clone());
                self.apply_range_recu(l, r, node*2 + 1, (now_l + now_r) / 2, now_r, f);
                self.update(node);
            }
        }

        // TODO: 二分探索 (max_left, max_right)

        fn update(&mut self, i: usize) {
            if i >= self.size {
                return;
            }
            self.datas[i] = F::op(&self.datas[2 * i], &self.datas[2 * i + 1]);
        }
        fn all_apply(&mut self, i: usize, f: F::F) {
            self.lazy[i] = F::composition(&f, &self.lazy[i]);
        }
        fn push(&mut self, i: usize) {
            self.datas[i] = F::mapping(&self.lazy[i], &self.datas[i]);
            if i < self.size {
                self.all_apply(2*i, self.lazy[i].clone());
                self.all_apply(2*i + 1, self.lazy[i].clone());
            }
            self.lazy[i] = F::id_map();
        }
    }
}

use proconio::{input, marker::Usize1};

fn main() {
    // https://atcoder.jp/contests/typical90/tasks/typical90_ac
    input! {
        w: usize,
        n: usize,
        lr: [(Usize1, usize); n],
    }
    struct Max;
    impl lazy_segtree::Monoid for Max {
        type S = u32;
        fn op(a: &Self::S, b: &Self::S) -> Self::S {
            *a.max(b)
        }
        fn id() -> Self::S {
            0
        }
    }
    struct MaxSet;
    impl lazy_segtree::MapMonoid for MaxSet {
        type M = Max;
        type F = Option<u32>;
        fn id_map() -> Self::F {
            None
        }
        fn mapping(&f: &Self::F, x: &<Self::M as lazy_segtree::Monoid>::S) -> <Self::M as lazy_segtree::Monoid>::S {
            match f as Option<u32> {
                Some(v) => v,
                None => *x,
            }
        }
        fn composition(&f: &Self::F, &g: &Self::F) -> Self::F {
            match f as Option<u32> {
                Some(v) => Some(v),
                None => g,
            }
        }
    }
    let mut lz = lazy_segtree::LazySegtree::<MaxSet>::new(w);
    for (l, r) in lr {
        let m: u32 = lz.prod(l, r);
        println!("{}", m + 1);
        lz.apply_range(l, r, Some(m+1));
    }
}
