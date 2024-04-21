# `utils`
## `binary_search`
### `binary_search`
```rust
fn binary_search<F: Fn(usize) -> bool>(initial_t: usize, initial_f: usize, f: F) -> usize
```

- `initial_t < initial_f` の場合
  - 区間 `[initial_t, initial_f)` において `f(x) = true` かつ `f(x + 1) = false` となるような `x` を返す．特に `f` が単調なら，`f(x) = true` となるような最大の `x` を返す．
- `initial_t > initial_f` の場合
  - 区間 `(initial_f, initial_t]` において `f(x - 1) = false` かつ `f(x) = true` となるような `x` を返す．特に `f` が単調なら，`f(x) = true` となるような最小の `x` を返す．

##### 制約
- `initial_t >= 0`
- `initial_f >= 0`
- `f(initial_t) = true`
- `f(initial_f) = false`

##### 計算量
- 区間幅の初期値 `|initial_t - initial_f|` を $m$ として $O(\log m)$

### `lower_bound`
```rust
fn lower_bound<T: PartialOrd>(a: &Vec<T>, x: T) -> usize
```

- 昇順にソートされた要素数 $n$ の配列 $(a_i)$ に対して $a_i\geq x$ となる最小の $i$ を返す．そのような $i$ が存在しない場合は $n$ を返す．

##### 制約
- $a_0\leq a_1\leq \cdots \leq a_{n - 1}$

##### 計算量
- $O(\log n)$

## `compression`

### `Compression<T>` 構造体
$n$ 個の要素からなる集合において，前計算 $O(n\log n)$ ののちに，要素 $x$ が小さいほうから数えて何番目に位置するかを $O(\log n)$ で求めることができる（いわゆる座標圧縮）．

#### 注意
- `get_index` および `get_value` は `compress` メソッドを実行してから呼ぶこと．
- 集合の要素の型`T`は`Ord`トレイト，`Hash`トレイト，`Copy`トレイトを実装している必要がある．

#### `new`
```rust
fn new() -> Self
```
- 空の集合を作成する．

#### `add`
```rust
// [1]
fn add(&mut self, x: T)

// [2]
fn add_from_vec(&mut self, vec: &Vec<T>)
```
- [1] 集合に要素 $x$ を追加する．
- [2] 集合にベクタ `vec` の要素をすべて追加する．

#### `compress`
```rust
fn compress(&mut self)
```
- 座標圧縮を行う．

##### 計算量
- 要素数を $n$ として $O(n\log n)$

#### `get_index`
```rust
fn get_index(&self, x: T) -> Option<&usize>
```
- 集合の要素を昇順にソートしたとき，要素 `x` が先頭から何番目かを返す．
- `x` が集合の要素でない場合は `None` を返す．

##### 計算量
- 要素数を $n$ として $O(\log n)$

#### `get_value`
```rust
fn get_value(&self, idx: usize) -> T
```
- 集合の要素を昇順にソートしたとき，先頭から `idx` 番目の要素を返す（0-indexed）．

##### 制約
- 要素数を $n$ として $0 \leq {\rm idx} < n$

##### 計算量
- $O(1)$

#### Verification Code
- https://atcoder.jp/contests/abc036/submissions/52655063
