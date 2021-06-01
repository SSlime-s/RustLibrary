<!-- -*- coding:utf-8-unix -*- -->
# Lazy SegTree
## 使い方
### 構築
```rust
let mut lz: LazySegtree::<M> = v.clone().into()
```
or
```rust
let mut lz = LazySegtree::<M>::new(n);
```

### 更新
```rust
lz.apply_range(l, r, f);
```

### 取得
```rust
lz.prod(l, r);
```
## Monoid Example
### RMQ + RUQ
```rust
struct Max;
impl lazy_segtree::Monoid for Max {
   type S = u64;
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
   type F = Option<u64>;
   fn id_map() -> Self::F {
      None
   }
   fn mapping(&f: &Self::F, x: &<Self::M as lazy_segtree::Monoid>::S) -> <Self::M as lazy_segtree::Monoid>::S {
      match f as Option<u64> {
            Some(v) => v,
            None => *x,
      }
   }
   fn composition(&f: &Self::F, &g: &Self::F) -> Self::F {
      match f as Option<u64> {
            Some(v) => Some(v),
            None => g,
      }
   }
}
```
### RMQ + RAQ
<!-- TODO: verify -->
```rust
struct Max;
impl lazy_segtree::Monoid for Max {
   type S = u64;
   fn op(a: &Self::S, b: &Self::S) -> Self::S {
      *a.max(b)
   }
   fn id() -> Self::S {
      0
   }
}
struct MaxAdd;
impl lazy_segtree::MapMonoid for MaxSet {
   type M = Max;
   type F = u64;
   fn id_map() -> Self::F {
      0
   }
   fn mapping(&f: &Self::F, x: &<Self::M as lazy_segtree::Monoid>::S) -> <Self::M as lazy_segtree::Monoid>::S {
      f + x
   }
   fn composition(&f: &Self::F, &g: &Self::F) -> Self::F {
      f + g
   }
}
```

### RSQ + RUQ
<!-- TODO: verify -->
```rust
#[derive(Clone)]
struct Element {
   val: u64,
   size: usize,
}
struct Sum;
impl lazy_segtree::Monoid for Sum {
   type S = Element;
   fn op(a: &Self::S, b: &Self::S) -> Self::S {
      Element {
            val: a.val + b.val,
            size: a.size + b.size
      }
   }
   fn id() -> Self::S {
      Element {
            val: 0,
            size: 0,
      }
   }
}
struct SumSet;
impl lazy_segtree::MapMonoid for SumSet {
   type M = Sum;
   type F = Option<u64>;
   fn id_map() -> Self::F {
      None
   }
   fn mapping(&f: &Self::F, x: &<Self::M as lazy_segtree::Monoid>::S) -> <Self::M as lazy_segtree::Monoid>::S {
      Element {
            val: match f as Option<u64> {
               Some(v) => v * x.size as u64,
               None => x.val,
            },
            size: x.size
      }
   }
   fn composition(&f: &Self::F, &g: &Self::F) -> Self::F {
      match f as Option<u64> {
               Some(v) => Some(v),
               None => g,
      }
   }
}
```
