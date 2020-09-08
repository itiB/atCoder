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

## atc

### atc001

- [B - Union Find](/atc001/b.rs)
  - UnionFindを用いて入力された値をグループに分ける

## arc

### arc059

- [C - いっしょ](/arc059/c.rs)
  - すべての値について総当たりで差の合計が最小になるものを探す
  - `vec -> .iter().fold()` 使ってみている

### arc068

- [C - X: Yet Another Die Game](/arc068/c.rs)
  - さいころを回して目標の数を超えるのがいつかを調べる問題
  - 計算で求められる
  - HashMapの長さやそのなかのキーがそれぞれいくつ含まれてるかとか求めたりとかした
  - また特定の値が含まれたりとかしらべたしHashMapで困ったらみるべき

### agc

### agc035

- [A - XOR Circle](/agc035/a.rs)
  - XORの性質を用いたもの
  - 円形上のラクダに帽子をかぶせられるか

## abc

### abc051

- [A - Haiku](/abc051/a.rs)
  - 文字列の指定した文字を置換する .replace("a", "b")->aをbに置換できる
- [B - Sum of Three Integers](/abc051/b.rs)
  - 3変数の合計値が指定値となるものを数え上げる
  - 3重ループだとTLEするため2重ループまでの情報から3変数目の値を逆算しなりたつかを調べる
- [C - Back and Forth](/abc051/c.rs)
  - 最短経路を4つ、かぶらないように作る問題
  - スタート地点とゴールが1対1対応することをもちいて解いた

### abc054

- [B - Template Matching](/abc054/b.rs)
  - 入力された画素列のなかに別の入力された画素列が含まれているか調べる問題
  - 2重ループをbreakで抜けたりしつつ総当たりで調べた

### abc087

- [C - Candies](/abc087/c.rs)
  - 飴を集められる最大のパターンを求める問題
  - 何度目で下に移動するかを決めて選ぶ解き方とそれぞれの場所に最大でいくつの飴をもった状態で行けるかを記録していく2つのパターンが存在する

### abc094

- [C - Many Medians](/abc094/c.rs)
  - ひたすら中央値を繰り返し求める問題
  - 入力されたベクタとともにソートされたベクタを作り、
  指定した値を抜いた場合にはソートされたベクタの何番目が中央値となるかを計算することで解くことができる

### abc149

- [D - Prediction and Restriction](/abc149/d.rs)
  - じゃんけんバトル、特定回数前と同じ手を出すことはできない

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
- [D - Chat in a Circle](/abc173/d.rs)
  - 2分木的なもの
  - 条件から法則を見つければ解ける系問題
  - 環状にならぶのはわかりにくいからスタート地点を両端においた線をつくるとわかる

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

### abc176

- [A - Takoyaki](/abc176/.rs)
  - たこやきを指定数焼くには何分ほしいか
  - あまり切り上げ算
- [B - Multiple](/abc176/.rs)
  - めちゃ大きい数をたしていって9の倍数か判定する
  - 文字列化してやれば行ける
- [C - Step](/abc176/.rs)
  - 踏み台の高さが合計いくらほしいか
  - それまでの高さを保存してそれを超えられるように踏み台を足していく
- [D - Wizard](/abc176/.rs)
  - 完全に読み間違えて行ける範囲をすべて1増やしていた
  - ダイクストラ法的な行ける範囲を何回で行けるかで塗りつぶしていくゲーム

### abc177

- [A - Don't be late](/abc177/a.rs)
  - 時間と速さの計算、予定時間に間に合うか
- [B - Substring](/abc177/b.rs)
  - 何文字書き換えると部分文字列が一致するかを調べる
  - forで総当たりして一番短くて済むものを出力させた
- [C - Sum of product of pairs](/abc177/c.rs)
  - 二十シグマの計算、
  - なんか式変形するとシグマをひとつに減らせて時間内に収まる
- [D - Friends](/abc177/d.rs)
  - AとBが友達、BとCが友達ならAとCも友達とするグループ分け
  - UnionFindなるものを実装した
    - どのグループか、グループには何個のノードがあるかがわかる
  - これ便利そう

## others

### code-festival-2017-quala

- [B - fLIP](/code-festival-2017-quala/b.rs)
  - オセロ的なひっくり返していくゲーム
  - どちらの列を選択したらいくつひっくり返るかを式にすると解ける

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

### Codefestival_2016_final

- [B - Exactly N points](/codefestival_2016_final/b.rs)
  - ちょうど目的の点数をとるためにはどのように問題を解けばいいか。
  - このときの一番点数が高い問題が最小になるような場合を求める
  - 累積和を求めておき、合計値が目標点を超えた場合にそれまでの値を出力する
  - この際に目標値との差分を除外して出力する
