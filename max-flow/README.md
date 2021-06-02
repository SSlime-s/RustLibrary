<!-- -*- coding:utf-8-unix -*- -->
# Max Flow
## 使い方
### 構築
```rust
let mut flow = MfGraph::new(n)
```
#### 辺の追加
```rust
flow.add_edge(from, to, cap);
```
### 計算
```rust
flow.flow(start, gorl)
```
