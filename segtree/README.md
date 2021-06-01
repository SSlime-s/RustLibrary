<!-- -*- coding:utf-8-unix -*- -->
# SegTree
## 使い方
### 構築
```rust
let mut seg: Segtree<M> = v.clone().into();
```
or
```rust
let mut seg = Segtree::<M>::new(n);
```

### 更新
```rust
seg.set(i, x);
```
### 取得
```rust
seg.prod(l, r);
```

## Monoid Example
### RSQ
```rust
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
```
### RMQ
```rust
struct Min {};
impl segtree::Monoid for Min {
    type S = i64;
    fn op(a: &Self::S, b: &Self::S) -> Self::S {
        *a.min(b)
    }
    fn id() -> Self::S {
        i64::max_value()
    }
}
```
