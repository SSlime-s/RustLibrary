<!-- -*- coding:utf-8-unix -*- -->
# FenwickTree
## 使い方
### 構築
```rust
let mut fw = FenwickTree::new(n, 0u64);
```

### 更新
```rust
fw.add(i, x);
```

### 取得
```rust
fw.range_sum(l, r);
```
