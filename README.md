# Rubyst
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

A Ruby Interpreter written in Rust 🦄

## 対応している文法

|型||
| :---:  | :---:|
| bool型 | Bool | 
| int型  | Int  |

|演算子||
| :---:  | :---:|
| 掛け算 | + | 
| 足し算 | - |
| 掛け算 | * |
| 割り算 | / |
| 余り | % |
| 乗算 | ** |

|比較演算子||
| :---:  | :---:|
|イコール|==| 
|大なりイコール|>=|
|小なりイコール|<=|
|大なり|>|
|小なり|<|

### 関数
#### if

```ruby
i = 10
if i == 10
  i = i + 1
else
  i = i - 1
end
p(i)
```

#### while

```ruby
i = 10
while i > 0
  p(i)
  i = i - 1
end
```

#### begin

```ruby
i = 10
begin
  p(i)
  i = i - 1
end while i > 0
```


## 記述例

```main.eld
(x = 1)
y = x + 1

if y == 2
  p(x + 2)
end

if y < 1
  p(x * 3)
end

p(y % 6)

x = 0
if x < 0
else
  while x < 10
    p(x)
  end
end
```

## 開発指針
コードはできるだけ綺麗にするけど開発の速さのためには妥協もする
## このrubystインタプリタで用いられる抽象構文木において

### 望ましいこと
tree.rootがType::Nilであるときtree.leftがOption::Noneであること

&rarr; 実行速度が遅くなるため

### アンチパターン
tree.rootがType::Nilであるときtree.rightがOption::Noneでないこと

&rarr; 複雑度が上がり、開発者が潜在的なバグを予測できなくなるため

## 参考
遠藤侑介 RubyでつくるRuby ラムダノート株式会社
