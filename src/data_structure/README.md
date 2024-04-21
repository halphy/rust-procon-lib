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
- $a_i=f(i) \; (i=0, \cdots, n - 1)$ で定義される数列 $(a_i)$ に対して，その累積和を計算する．
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
- 区間 `[l, r)` に対する和 $a_l+ \cdots +a_{r - 1}$ を計算する．$l = r$ のときは `e` を返す．

##### 計算量
- $O(1)$

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
