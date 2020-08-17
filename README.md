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

### abc171

- [A - αlphabet](/abc171/a.rs)
  - 文字の範囲 a-z A-Z の判別
- [B - Mix Juice](/abc171/b.rs)
  -ソートして小さいものから順番に足すだけ
- [C - One Quadrillion and One Dalmatians](/abc171/c.rs)
  - 26進数の計算、ずれる分は商-1することでケアする

### abc173

- [A - Payment](/abc173/a.rs)
  - 1000円札のみで支払った際のお釣りを求める
- [B - Judge Status Summary](/abc173/b.rs)
  - 文字列の数え上げ
  - AC, TLE, WA
- [C - H and V](/abc173/c.rs)
  - オセロのひっくり返すやつ的な
  - 選んだ列を塗りつぶしてのこりの色は何個あるか調べる

### abc174

- [A - Air Conditioner](/abc174/a.rs)
  - 数字の大きさ比較
- [B - Distance ](/abc174/b.rs)
  - 座標の計算、距離の計算
  - sqrtは大変だから2乗されたままで計算することがミソ
- [C - Repsept](/abc174/c.rs)
  - 倍数が登場するのは何回目か調べる
  - modの世界で余りがループし始めたら以降も割り切れるものは登場しない
- [D - Alter Altar](/abc174/d.rs)
  - 石を入れ替えていって条件にあう形にするまで何回かかるか探索
  - 入れ替え回数を数え上げる
- [E - Logs](/abc174/e.rs)
  - 木を最小に切る問題
  - いくつの大きさに切れば条件の回数以内に切れるかを2分探索する

### abc175

- [A - Rainy Season](/abc175/a.rs)
  - 同じ文字が連続で現れる最大個数を数え上げる
- [B - Making Triangle](/abc175/b.rs)
  - 三角形を作ることができる棒を選択する組み合わせを総当たりで調べる問題
- [C - Walking Takahashi](/abc175/c.rs)
  - 数直線を移動して原点からの距離をとる -> 絶対値ですべて正の空間で考える
  - 移動を数式化する, 原点までたどり着けるパターン, できないパターン

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

### aising2020

- [A - Number of Multiples](/aising2020/a.rs)
  - 倍数の範囲を調べる、下限が倍数に含まれていた場合に備えて-1して倍数の数を求めている
- [B - An Odd Problem](/aising2020/b.rs)
  - 奇数番目の箱の中身が奇数のもののみを取り出す
- [C - XYZ Triplets](/aising2020/c.rs)
  - 式の答えが特定の値と一致するかの探索
  - 変数一つで最大値になる場合を求め、それを最大値としてループを回す
  - 一致するかはあらかじめ答えとして取りうる範囲のベクタを作っておいて一致するとこにどんどん代入する