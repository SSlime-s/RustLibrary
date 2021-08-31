mod union_find {
    pub struct UnionFind {
        parent: Vec<usize>,
        size: Vec<usize>,
        rank: Vec<u8>,
    }
    impl UnionFind {
        pub fn new(n: usize) -> Self {
            UnionFind {
                parent: (0..n).collect::<Vec<usize>>(),
                size: vec![1; n],
                rank: vec![0; n],
            }
        }

        pub fn find(&mut self, x: usize) -> usize {
            assert!(x < self.parent.len());
            let mut now = x;
            loop {
                let p = self.parent[now];
                if now == p {
                    break;
                }
                now = p;
            }
            self.parent[x] = now;
            now
        }

        pub fn equiv(&mut self, x: usize, y: usize) -> bool {
            assert!(x < self.parent.len() && y < self.parent.len());
            self.find(x) == self.find(y)
        }

        pub fn union(&mut self, x: usize, y: usize) -> bool {
            assert!(x < self.parent.len() && y < self.parent.len());
            let xp = self.find(x);
            let yp = self.find(y);
            if xp == yp {
                return false
            }
            match self.rank[xp].cmp(&self.rank[yp]) {
                std::cmp::Ordering::Less => {
                    self.parent[xp] = yp;
                    self.size[yp] += self.size[xp];
                },
                std::cmp::Ordering::Greater => {
                    self.parent[yp] = xp;
                    self.size[xp] += self.size[yp];
                },
                std::cmp::Ordering::Equal => {
                    self.parent[yp] = xp;
                    self.size[xp] += self.size[yp];
                    self.rank[xp] += 1;
                }
            }
            true
        }

        pub fn size(&mut self, x: usize) -> usize {
            assert!(x < self.parent.len());
            let xp = self.find(x);
            self.size[xp]
        }
    }
}
