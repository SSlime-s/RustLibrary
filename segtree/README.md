<!-- -*- coding:utf-8-unix -*- -->
# SegTree
## 使い方
### 構築
```rust
let mut rmq = seg_tree::SegTree::<_, _>::from(&a, |&a, &b| -> i64 {a + b}, 0);
```
### 作用
```rust
rmq.update(i, x);
```
### 上書き
```rust
rmq.change(i, x);
```
### 取得
```rust
rmq.get(l, r);
```
