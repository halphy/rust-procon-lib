---
title: Union Find（素集合データ構造）
documentation_of: ../union_find.rs
---

# `union_find`

## `UnionFind`
無向グラフの連結成分を管理するデータ構造．

### `new`
```rust
fn new(n: usize) -> Self
```
- $n$ 頂点 $0$ 辺の無向グラフを作成する．
#### 制約
- $n\geq 0$

#### 計算量
- $O(n)$

### `unite`
```rust
fn unite(&mut self, mut u: usize, mut v: usize)
```
- 頂点 $u, v$ の間に辺を追加する．

#### 制約
- $0\leq u, v < n$

#### 計算量
- ならし $O(\alpha(n))$

### `is_same`
```rust
fn is_same(&mut self, u: usize, v: usize) -> bool
```
- 頂点 $u, v$ が同じ連結成分に属するかどうかを返す．

#### 制約
- $0\leq u, v < n$

#### 計算量
- ならし $O(\alpha(n))$

### `find`
```rust
fn find(&mut self, v: usize) -> usize
```
- 頂点 $v$ が属する連結成分の代表元を返す．

#### 制約
- $0\leq v < n$

#### 計算量
- ならし $O(\alpha(n))$

### `get_size`
```rust
fn get_size(&mut self, v: usize) -> usize
```
- 頂点 $v$ が属する連結成分の頂点数を返す．

#### 制約
- $0\leq v < n$

#### 計算量
- ならし $O(\alpha(n))$
