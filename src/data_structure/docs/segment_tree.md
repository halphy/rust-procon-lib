---
title: セグメント木
documentation_of: ../segment_tree.rs
---

# `segment_tree`

## `SegmentTree`
数列を管理しながら1点更新・区間演算を高速に行うデータ構造．

### 前提条件
集合 $S$ がある演算 $\cdot : S\times S\to S$ に関してモノイドをなすとする．すなわち

1. （結合律）任意の $x, y, z\in S$ に対して
$$`(xy)z=x(yz)`$$
2. （単位元の存在）ある $e\in S$ が存在して任意の $x\in S$ に対して $$`ex=xe=x`$$

が成り立つと仮定する．

このとき，$S$ の元からなる列 $a_0,\cdots a_{N-1}\; (a_i\in S)$ に対して

- 要素の1点更新
- 区間の要素の総積の取得

をいずれも $O(\log N)$ で行うことができる．

### `new`
```rust
fn new(n: usize, id: T, op: F) -> Self
```
- `n` 個の要素からなる数列を構築する．
- `id: T` は $S$ の単位元
- `op: F` は $S$ に対して定義された二項演算
    - `F` は `Fn(T, T) -> T` トレイトを実装している．
- 初期値はすべて `id` である．

#### 制約
- $n\geq 0$

#### 計算量
- $O(n)$

### `construct`
```rust
fn construct(&mut self, seq: &Vec<T>)
```
- 数列の値を `seq` の要素で置き換える．

#### 制約
- `seq.len()`は数列の長さ $n$ に等しい

#### 計算量
- $O(n)$

### `update`
```rust
fn update(&mut self, mut i: usize, x: T)
```
- 数列の `i` 番目の値を `x` で置き換える．

#### 制約
- $0\leq i < n$

#### 計算量
- $O(\log n)$

### `prod`
```rust
fn prod(&self, mut l: usize, mut r: usize) -> T
```
- 区間 `[l, r)` のすべての要素にわたる積を返す．

#### 制約
- $0\leq l \leq r \leq n$

#### 計算量
- $O(\log n)$

### `get`
```rust
fn get(&self, i: usize) -> T
```
- 数列の `i` 番目の値を返す．

#### 制約
- $0 \leq i < n$

#### 計算量
- $O(1)$
