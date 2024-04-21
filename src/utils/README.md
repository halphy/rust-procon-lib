# `utils`
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
