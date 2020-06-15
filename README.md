# atCoder

## commit message

- add: <コンテスト名> (AC: <問題名>)
  - コンテストが終わった時点でのコミット
- <コンテスト名>
  - README.mdを追加したときのコミット
- Refact: <問題名>
  - Refact
- AC: <問題名>
  - コンテスト後にACしたものを書く

## abc

### abc159

- [A - The Number of Even Pairs](/abc159/a.rs)
  - 偶数奇数の足し算
- [B - String Palindrome](/abc159/b.rs)
  - 回文の探索
  - 文字列長を図る
- [C - Maximum Volume](/abc159/c.rs)
  - 体積の最大値を求める
  - 長さを実際にaと置いて上限させ，すべてa置いた場合と比較するとすべてaの時に最大となることがわかる
  - 小数点のフォーマット方法

### abc164

- [A - Sheep and Wolves](/abc164/a.rs)
  - 2数の大小比較
- [B - Battle](/abc164/b.rs)
  - 4変数をもとに順番に相手の体力を削りあう
- [C - gacha](/abc164/c.rs)
  - 文字列をもとにHashMapを作成
- [D - Multiple of 2019](/abc164/d.rs)
  - 数字の列から数値を作成し2019の倍数であるものを探す

### abc165

- [A - We Love Golf](/abc165/a.rs)
  - 値の最小倍数を求める
- [B - 1%](/abc165/b.rs)
  - 利子計算
- [C - Many Requirements](/abc165/c.rs)
  - 再帰総当たり順列計算
- [D - Floor Function](/abc165/d.rs)
  - floor関数の最大値計算
  - `f(x) = px + q`の形

### abc166

- [A - A?C](/abc166/a.rs)
  - 文字列入力，比較
- [B - Trick or Treat](/abc166/b.rs)
  - 複雑な入力，ループとか
  - ベクタのソートと重複削除，要素の確認
- [C - Peaks](/abc166/c.rs)
  - ベクタのフィルタと数え上げ
- [D - I hate Factorization](/abc166/d.rs)
  - 式の答えを出すことができる範囲を求めて総当たり計算させる
  - 制約よりループの範囲を求めることでできる

### abc167
- [A - Registration](/abc167/a.rs)
  - 文字列一致の問題
- [B - Easy Linear Programming](/abc167/b.rs)
  - 線形計画問題
- [C - Skill Up](/abc167/c.rs)
  - 全探索
- [D - Teleporter](/abc167/d.rs)
  - 部分集合の探索
  - 01で集合を分ける

### abc168

- [A - ∴ (Therefore)](/abc168/a.rs)
  - switch(match)
- [B - ... (Triple Dots)](/abc168/b.rs)
  - 文字列処理(スライス)
- [C - : (Colon)](/abc168/c.rs)
  - 三角関数 数学
  - 2乗, sqrt
- [D - .. (Double Dots)](/abc168/d.rs)
  - BFS: 幅優先探索
  - キューの作成

### abc170

- [A - Five Variables](/abc170/a.rs)
- [B - Crane and Turtle](/abc170/b.rs)
  - つるかめ算
- [C - Forbidden List](/abc170/c.rs)
  - 数列中に含まれない値のなかで最も入力に近い値を求める
- [D - Not Divisible](/abc170/d.rs)
  - エラトステネスの篩もどき
  - 約数がないことを探索する

## others

### nomura2020

- [A - Study Scheduling](/nomura2020/a.rs)
  - 時間の計算
- [B - Postdocs](/nomura2020/b.rs)
  - "?"を置き換えて点数を最大化する問題
  - 全部Dに置き換えても最大になることに気が付かなった自分を殴りたい
- [C - Folia](/nomura2020/c.rs)
  - 2分木の葉の数についての問題
  - 深さdの最大葉でない頂点の数はA_d+1 + A_d+2.. A_d+n だぁぁぁ泣いた
