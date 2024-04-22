# `data_structure`

## `prefix_sum`
### `PrefixSum<T>`構造体
1次元累積和を管理するデータ構造．前計算のもとで，要素数 $n$ の配列に対してその区間和を $O(1)$ で計算することができる．

#### 注意
- 型`T`は`Add`トレイト，`Sub`トレイト，`Copy`トレイトを実装している必要がある．

#### `new`
```rust
fn new(n: usize, e: T, f: impl Fn(usize) -> T) -> Self
```
- $`a_i=f(i) \; (i=0, \cdots, n - 1)`$ で定義される数列 $(a_i)$ に対して，その累積和を計算する．
- `e` は型 `T` が表す代数系の加法単位元を表す．

##### 制約
- $n\geq 0$
- $i=0, \cdots, n - 1$ に対して $f(i)$ が定義されている
- 型が `T` であるような任意の `x` に対して `x + e` は `x` に等しい

##### 計算量
- $O(n)$

#### `query`
```rust
fn query(&self, l: usize, r: usize) -> T
```
- 区間 `[l, r)` に対する和 $`a_l+ \cdots +a_{r - 1}`$ を計算する．$`l = r`$ のときは `e` を返す．

##### 計算量
- $O(1)$

#### Verification Code
- https://atcoder.jp/contests/abc328/submissions/52659481

### `PrefixSum2D<T>`構造体
2次元累積和を管理するデータ構造．

#### 注意
- 型`T`は`Add`トレイト，`Sub`トレイト，`Copy`トレイトを実装している必要がある．

#### `new`
```rust
fn new(h: usize, w: usize, e: T, f: impl Fn(usize, usize) -> T) -> Self
```
- $`a_{ij}=f(i, j) \; (i=0, \cdots, h - 1; j=0,\cdots, w - 1)`$ で定義される数列 $(a_{ij})$ に対して，その累積和を計算する．
- `e` は型 `T` が表す代数系の加法単位元を表す．

##### 制約
- $h, w\geq 0$
- $i=0, \cdots, h - 1; j=0,\cdots, w - 1$ に対して $f(i, j)$ が定義されている
- 型が `T` であるような任意の `x` に対して `x + e` は `x` に等しい

##### 計算量
- $O(hw)$

#### `query`
```rust
fn query(&self, lx: usize, rx: usize, ly: usize, ry: usize) -> T
```
- 区間 `[lx, rx)` $`\times`$ `[ly, ry)` に対する和
$$\sum_{{\rm lx}\leq i < {\rm rx}}\sum_{{\rm ly}\leq j < {\rm ry}}a_{ij}$$
を計算する．

##### 計算量
- $O(1)$

#### Verification Code
- https://atcoder.jp/contests/arc025/submissions/52662103

## `range_set`
### `RangeSet`
区間をsetで管理するデータ構造．

集合 $`S \subset \mathbb{Z}`$ に対して，$`S`$ を区間の和集合として

$$S=\bigcup_i [l_i, r_i) \\ (l_0 < r_0 < l_1 < \cdots < r_i < l_{i + 1} < \cdots)$$

のように表すことができる（この表示は一意的である）．

このとき，$`S`$ の代わりに区間の集合 $`T\equiv \{[l_i, r_i)\mid i\in \mathbb{N}\}`$ を管理することで，集合 $`S`$ に対する要素の追加・削除・存在判定に加えて，集合のMEXを求める操作 $`O(\log n)`$（$`n`$は区間の数）で行うことができる．

#### `new`
```rust
pub fn new() -> Self
```
- 空の集合を作成する．

#### `contains`
```rust
fn contains(&self, x: i64) -> bool
```
- $x$ が集合に属しているかを返す．

##### 計算量
- 区間の数を $n$ として $O(\log n)$

#### `get_range`
```rust
fn get_range(&self, x: i64) -> Option<(i64, i64)>
```
- $x \in [l, r)$ となるような区間 $[l, r) \in T$ が存在するならば，$`l, r`$ のタプルを返す．存在しなければ `None` を返す．

##### 計算量
- 区間の数を $n$ として $O(\log n)$

#### `insert`
```rust
fn insert(&mut self, x: i64)
```
- $x$ を集合に追加する．すでに $x$ が集合に属する場合は何もしない．

##### 計算量
- 区間の数を $n$ として $O(\log n)$

#### `remove`
```rust
fn remove(&mut self, x: i64)
```
- $x$ を集合から削除する．$`x`$ が集合に属さない場合は何もしない．

##### 計算量
- 区間の数を $n$ として $O(\log n)$

#### `mex`
```rust
fn mex(&self, x: i64) -> i64
```
- $y\geq x$ かつ $y \notin S$ となる最小の $y \in \mathbb{Z}$ を返す．
- 特に $S \subset \mathbb{N}$ かつ $x=0$ の場合，これは集合 $S$ のMEX (minimum excluded value) を求めることに相当する．

##### 計算量
- 区間の数を $n$ として $O(\log n)$

#### Verification Code
- https://atcoder.jp/contests/hhkb2020/submissions/52692061

## `union_find`

### `UnionFind`構造体
無向グラフの連結成分を管理するデータ構造．

#### `new`
```rust
fn new(n: usize) -> Self
```
- $n$ 頂点 $0$ 辺の無向グラフを作成する．
##### 制約
- $n\geq 0$

##### 計算量
- $O(n)$

#### `unite`
```rust
fn unite(&mut self, mut u: usize, mut v: usize)
```
- 頂点 $u, v$ の間に辺を追加する．

##### 制約
- $0\leq u, v < n$

##### 計算量
- ならし $O(\alpha(n))$

#### `is_same`
```rust
fn is_same(&mut self, u: usize, v: usize) -> bool
```
- 頂点 $u, v$ が同じ連結成分に属するかどうかを返す．

##### 制約
- $0\leq u, v < n$

##### 計算量
- ならし $O(\alpha(n))$

#### `find`
```rust
fn find(&mut self, v: usize) -> usize
```
- 頂点 $v$ が属する連結成分の代表元を返す．

##### 制約
- $0\leq v < n$

##### 計算量
- ならし $O(\alpha(n))$

#### `get_size`
```rust
fn get_size(&mut self, v: usize) -> usize
```
- 頂点 $v$ が属する連結成分の頂点数を返す．

##### 制約
- $0\leq v < n$

##### 計算量
- ならし $O(\alpha(n))$

#### Verify
- https://atcoder.jp/contests/practice2/submissions/52654022
