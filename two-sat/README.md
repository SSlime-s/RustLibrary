<!-- -*- coding:utf-8-unix -*- -->
# Two Sat
## 使い方
### 構築
```rust
let mut ts = TwoSat::new(n)
```
#### 条件の追加
$(x == f) \lor (y == g)$ なる条件を追加
```rust
ts.add_clause(x, f, y, g);
```
### 計算
```rust
ts.satisfiable();
```
