# `math`

## `binomial`
### `BinomialCoefficient<T>`
素数 $p$ に対して二項係数 $C(n, r)\pmod p$ を計算する．

#### 注意
- 型 `T` は [ac-library-rs](https://github.com/rust-lang-ja/ac-library-rs) の `ModIntBase` トレイトを実装している必要がある．
  - 例えば `ModInt998244353`, `ModInt1000000007` はこの条件を満たす．

#### `new`
```rust
fn new(size: usize) -> Self
```
- 初期化を行う．
- この後に呼ぶ `factorial(n)` および `binomial_coefficient(n, r)` において，引数 `n` は `size` 未満でなければならない．

##### 計算量
- $O({\rm size})$

#### `factorial`
```rust
fn factorial(&self, n: usize) -> T
```
- $n!$ の値を返す．

##### 制約
- $0\leq n < {\rm size}$

##### 計算量
- $O(1)$

#### `binomial_coefficient`
```rust
fn binomial_coefficient(&self, n: usize, r: usize) -> T
```
- $0 \leq r\leq n$ の場合
$$ C(n, r)\equiv \frac{n!}{r!(n - r)!}$$
の値を返す．
- $r > n$ の場合は $0$ を返す．

##### 制約
- $0\leq r\leq n < {\rm size}$ または $r > n$

##### 計算量
- $O(1)$

#### Verification Code
- https://atcoder.jp/contests/abc021/submissions/52697217
