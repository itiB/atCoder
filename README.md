# atCoder

<!-- @import "[TOC]" {cmd="toc" depthFrom=1 depthTo=6 orderedList=false} -->

<!-- code_chunk_output -->

- [atCoder](#atcoder)
  - [commit message](#commit-message)
  - [DPまとめコンテスト](#dpまとめコンテスト)
    - [typical DP Contest](#typical-dp-contest)
  - [atc](#atc)
    - [atc001](#atc001)
  - [arc](#arc)
    - [arc038](#arc038)
    - [arc047](#arc047)
    - [arc058](#arc058)
    - [arc059](#arc059)
    - [arc068](#arc068)
    - [arc092](#arc092)
    - [arc093](#arc093)
    - [arc098](#arc098)
    - [arc105](#arc105)
    - [arc106](#arc106)
    - [arc109](#arc109)
    - [arc110](#arc110)
  - [agc](#agc)
    - [agc007](#agc007)
    - [agc009](#agc009)
    - [agc017](#agc017)
    - [agc019](#agc019)
    - [agc028](#agc028)
    - [agc034](#agc034)
    - [agc035](#agc035)
    - [agc048](#agc048)
  - [abc](#abc)
    - [abc023](#abc023)
    - [abc026](#abc026)
    - [abc045](#abc045)
    - [abc048](#abc048)
    - [abc051](#abc051)
    - [abc054](#abc054)
    - [abc057](#abc057)
    - [abc065](#abc065)
    - [abc075](#abc075)
    - [abc076](#abc076)
    - [abc077](#abc077)
    - [abc081](#abc081)
    - [abc082](#abc082)
    - [abc087](#abc087)
    - [abc093](#abc093)
    - [abc094](#abc094)
    - [abc114](#abc114)
    - [abc120](#abc120)
    - [abc122](#abc122)
    - [abc129](#abc129)
    - [abc131](#abc131)
    - [abc136](#abc136)
    - [abc144](#abc144)
    - [abc146](#abc146)
    - [abc148](#abc148)
    - [abc149](#abc149)
    - [abc150](#abc150)
    - [abc152](#abc152)
    - [abc153](#abc153)
    - [abc155](#abc155)
    - [abc156](#abc156)
    - [abc157](#abc157)
    - [abc158](#abc158)
    - [abc159](#abc159)
    - [abc162](#abc162)
    - [abc164](#abc164)
    - [abc165](#abc165)
    - [abc166](#abc166)
    - [abc167](#abc167)
    - [abc168](#abc168)
    - [abc169](#abc169)
    - [abc170](#abc170)
    - [abc171](#abc171)
    - [abc173](#abc173)
    - [abc174](#abc174)
    - [abc175](#abc175)
    - [abc176](#abc176)
    - [abc177](#abc177)
    - [abc179](#abc179)
    - [abc178](#abc178)
    - [abc180](#abc180)
    - [abc181](#abc181)
    - [abc182](#abc182)
    - [abc183](#abc183)
    - [abc184](#abc184)
    - [abc185](#abc185)
    - [abc192](#abc192)
    - [abc200](#abc200)
  - [others](#others)
    - [joi2008ho](#joi2008ho)
    - [code-festival-2017-quala](#code-festival-2017-quala)
    - [CADDi 2018](#caddi-2018)
    - [keyence2019](#keyence2019)
    - [みんなのプロコン2019](#みんなのプロコン2019)
    - [keyence2020](#keyence2020)
    - [nomura2020](#nomura2020)
    - [aising2020](#aising2020)
    - [Codefestival_2016_final](#codefestival_2016_final)
    - [hhkb2020](#hhkb2020)

<!-- /code_chunk_output -->

## commit message

- add: <コンテスト名> (AC: <問題名>)
  - コンテストが終わった時点でのコミット
- <コンテスト名>
  - README.mdを追加したときのコミット
- Refact: <問題名>
  - Refact
- AC: <問題名>
  - コンテスト後にACしたものを書く

## DPまとめコンテスト

- [A - Frog 1](/dp/a.rs)
  - dpを用いる問題初心者編
  - どこから足場がくるかをもとにループを回す
- [B - Frog 2](/dp/b.rs)
  - aでやった内容をもとに今度はdpを求める範囲を決めたもの
  - 今のところからいくつ前までは計算対象に入るから~~~でdpを作る
- [C - Vacation](/dp/c.rs)
  - 3択を選んでいって最大となる組み合わせを探す問題
  - 1日進むたびにどの選択肢からきたものがそれぞれのルートで最大の幸福値を得られるかを記録するDP
- [D - Knapsack 1](/dp/d.rs)
  - ナップサックdp
  - どの荷物を入れれば重さ以内で最大の価値を作ることができるか
  - i番目のものが重さj以内ならば入れる[n個のもの][重さ]の2次元配列を作る
  - ものを入れれ場合は `max(入れなかった場合の重さ, 入れる前 + 入れるものの重さ)`
  - 参考: https://qiita.com/drken/items/a5e6fe22863b7992efdb
- [F - LCS](/dp/f.rs)
  - 部分文字列を求める問題、2次元のDPを作る
  - 一致する文字な場合は前のDP(x, yそれぞれのDP)+1
  - 逆にたどる時は一致する文字列が着たら斜めに戻る
  - それ以外の時は左に戻ってチェック、上にいってチェックを繰り返す

### typical DP Contest

- [A - コンテスト](/typicalDp/a.rs)
  - コンテストので獲得しうる点数パターンをすべて列挙する問題
  - 0問正解で0点を初期値として一問目正解した場合、しなかった場合で取りうる点数列挙
  - 2問目正解したら取りうる点数を足していってーーーを繰り返す
  - 最後の問題で取りうる点数パターンがすべて並ぶのでいくつが取れるかを数え上げる

## atc

### atc001

- [B - Union Find](/atc001/b.rs)
  - UnionFindを用いて入力された値をグループに分ける

## arc

### arc038

- [C - 億マス計算](/arc038/c.rs)
  - a*b の値の中でN番目に小さい値を探すもの
  - つまりN-1個が決め打ちした値以下存在するものを2分探索する問題
  - 決めうちする値とループさせたa, limit以下になるbを2分探索する2重の2分探索が必要

### arc047

- [C - 一次元リバーシ](/arc047/c.rs)
  - リバーシを単一色にするには何手でできるかを求める問題
  - まとまりの個数を数えて - 1 するだけ、発想の問題

### arc058

- [C - 怪文書](/arc071/c.rs)
  - 文字列の辞書(HashMap)を作成してそれぞれいくつあるかを調べていく
  - それぞれの文字が含まれる最小の値を取っていけばすべての文字列に含まれる最大の文字数を調べられる。

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

### arc092

- [C - 2D Plane 2N Points](/arc092/c.rs)
  - 青X軸、y軸ともに小さいペアの個数を数える問題
  - 貪欲法を用いて青のX軸が一番大きいものに対して対応することができる赤色のうちY軸が最も大きいものを選んでいくを繰り返す
  - タプルのソートを作ることができた！ `tuple.sort_by_key(|item| item.1);`

### arc093

- [C - Traveling Plan](/arc093.c.rs)
  - 旅行する系問題
  - 目的地として行かなくなる場所ができた場合にいくらのコストがかかるようになるかを調べる問題
  - 全体のコストと置き換わるコストを調べて入れ替えればOK

### arc098

- [C - Attention](/arc098/c.rs)
  - 人々が向いている方向を統一する
  - 数列の左半分で目標と違う方向を向いてる人数、右半分で向いていない人数を調べるために累積和を用いることができる形にする

### arc105

- [A - Fourtune Cookies](/arc105/a.rs)
  - すべての組み合わせを列挙して総当たりで試した、美しくない...
- [B - MAX-=min](/arc105/b.rs)
  - 一番大きいカードから一番小さいカードを引くことを繰り返す
  - 最終的に出来上がる値はすべての数の最大公約数であるから最大公約数を求めて解ける

### arc106

- [A - 106](/arc106/a.rs)
  - 3^A + 5^b = N のとき、ABを求める問題
  - 3の累乗の値か調べたりとか作った
  - N - 5^bにして出てきた値が3 ^ Aで表せるなら解
- [B - Values](/arc106/b.rs)
  - 値をひっくり返す系は総和がかわらないことを生かす
  - 差をとってグループ分けし、それが0ならOKにする
  - UnionFindを用いた

### arc109

- [A - Hands](/arc109/a.rs)
  - 100階建ての建物があってiとi階, iとi+1階がつながっているときにb棟のn階に行く最短経路を求める
  - A棟だけで移動したとき、A棟からB棟へ移動するだけの時、B棟からA棟に移動することがあるときの3パターンでDPを作って求めた
- [B - log](/arc109/b.rs)
  - 丸太を切る系問題
  - n+1mまでの丸太があるので1~nまでの丸太をいくらで買えるかを調べる
  - つまりは n+1 の丸太をいくつに割れるか問題
  - ひたすら下から切っていけば logN で求めることができる
  - 2分探索してもいいらしい

### arc110

## agc

### agc007

- [A - Shik and Stone](/agc007/a.rs)
  - 駒が動いた軌跡を与えられるのでその駒は右下のみの移動をして右下までたどり着いたかを調べる
  - 駒が動いた軌跡だから軌跡の個数を調べれば良かったらしい
  - そこまで頭が回らずループで右、下に移動したかを記録しながら調べた
### agc009

- [A - Multiple Array](/agc009/a.rs)
  - ボタンを何回か押してすべての数を別で与えられた数列の倍数にする問題
  - 後ろから順番に貪欲法で足していく

### agc017

- [A - Biscuits](/agc017/a.rs)
  - 偶数と奇数の袋がそれぞれいくつあるか数える
  - 奇数の袋を奇数個(nC2k+1)または偶数個(nC1k)選ぶ組み合せがいくつあるか調べる関数
  - 合計が偶数になるには (偶数の袋をいくつでも選ぶ組 x 奇数の袋を偶数選ぶ組)
  - 合計が奇数になるには (偶数の袋をいくつでも選ぶ組 x 奇数の袋を奇数選ぶ組)

### agc019
- [A - Ice Tea Store](/agc019/a.rs)
  - 0.25, 0.5, 1, 2lの値段が渡されるのでちょうどNlを買うためにはいくら必要かを求める
  - 2Lを買うのに最小の値段 + 1Lを買うのに最小の値段でNLを達成すればOK

### agc028

- [A - Two Abbreviations](/agc028/a.rs)
  - range.all()でif文を各サンプルがおいてある
  - 最大公約数と最小公倍数をもちいて特定の法則で縮めた文字列が一致するかを求める問題

### agc034

- [A - Kenken Race](/agc034/a.rs)
  - 飛び越えて目的地にたどり着けるかを調べる
  - みぎにしか移動できない制約に気づいていなかったのでめちゃWA
  - 調べる範囲をうまく指定できなくてめちゃWA
  - 泣いた

### agc035

- [A - XOR Circle](/agc035/a.rs)
  - XORの性質を用いたもの
  - 円形上のラクダに帽子をかぶせられるか

### agc048

- [A - atcoder < S](/agc048/a.rs)
  - Stringの中に指定した文字が含まれるか探索
  - はじめがaでそのつぎがそれ以外なら....を考えて説く問題

## abc

### abc023

- [D - 射撃王](/abc023/d.rs)
  - 順番に風船を撃ち落としていく問題
  - 目標タイムを決めて二分探索する

### abc026

- [D - 高橋君ボール1号](/abc026/d.rs)
  - 2分探索を各問題
  - sinの入った式が与えられてその答えが100になるところを見つける
  - これは式が連続なことを生かすと2分探索で常に正に増えたりするわけでないけど特定の範囲内に必ず答えがあるから2分探索できる
  - 複数答えが考えられる問題

### abc045

- [C - たくさんの数式](/abc045/c.rs)
  - 数列を与えられるのでそれらの間に + を好きなようにはさんでできる値の合計値を調べる問題
  - ビット列を作って全探索する問題
  - 挟まるところを探索、挟まった場合には10を累乗する値を0に戻して足していく

### abc048

- [B - Between a and b ...](/abc048/b.rs)
  - aからbの範囲でxで割り切れる個数を数える
  - 終わりのbから0までの間で割り切れる個数 - 始めのaから0までの間で割り切れる個数 + はじめを含むか含まないか

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

### abc057

- [C - Digits in Multiplication](/abc057/c.rs)
  - ひたすらNを作るA x Bを求めていく問題
  - A x A までしか存在しえないので n < 10^10でも大丈夫
  - それぞれAとBの桁数を数えて大きいほうを現在のAnsと比較して小さいほうをAnsに入れる

### abc065

- [C - Reconciled?](/abc065/c.rs)
  - 犬とサルを整列させるパターンを数え上げる問題
  - 階乗計算はすぐにオーバーフローするので逐一modをとる必要がある
  - 並び方のパターンを調べ、それぞれについて並び変えた際の式を立てる

### abc075

- [C - Bridge](/abc075/c.rs)
  - UnionFind問題
  - UnionFindしてどれかの辺を挿入しなかった場合に辺のサイズが変わるものを探す

### abc076

- [C - Dubious Document 2](/abc076/c.rs)
  - 文字列 穴埋め
  - 文字列と??で埋められた文字列に対してキーとなる文字列が入りうるかを求める問題
  - 入るなかで辞書順で一番小さいものを出力するのでうしろから入るところを探せばOK

### abc077

- [C - Snuke Festival](/abc077/c.rs)
  - 2文探索を用いて指定した値を超えてしまう場所を探す
  - いくつ越えないものがあるかを調べてansに足していく
### abc081

- [B - Shift only](/abc081/b.rs)
  - 数列すべてが2で割り切れるときに割る
  - 各値が何回2で割り切れるかを求める
  - 一番割り切れる量が少ないものを出力する

### abc082

- [C - Good Sequence](/abc082/c.rs)
  - 文字の出現する数を数えてHashMapに入れる
  - 現れる回数とキーを比較して計算するだけ

### abc087

- [C - Candies](/abc087/c.rs)
  - 飴を集められる最大のパターンを求める問題
  - 何度目で下に移動するかを決めて選ぶ解き方とそれぞれの場所に最大でいくつの飴をもった状態で行けるかを記録していく2つのパターンが存在する

### abc093

- [B - Small and Large Integers](/abc093/b.rs)
  - 差が範囲内のもののみ出力する

### abc094

- [C - Many Medians](/abc094/c.rs)
  - ひたすら中央値を繰り返し求める問題
  - 入力されたベクタとともにソートされたベクタを作り、
  指定した値を抜いた場合にはソートされたベクタの何番目が中央値となるかを計算することで解くことができる
  
### abc114

- [C - 755](/abc114/c.rs)
  - 7, 5, 3だけでできる数字がいくつあるかを調べる
  - 数字からcharのHashSetに変換，ぜんぶが含まれているか調べる
  - 再帰を用いてLimitを超えていないか，すべての753が含まれているかを調べてあれば +1 で再帰を返す....を繰り返す

### abc120

- [C - Unification](/abc120/c.rs)
  - スタックを用いて同じキューブが隣り合った際に消す
  - 複雑なことはどうやらいらないらしい、証明でできるみたい

### abc122

- [C - GeT AC](/abc122/c.rs)
  - 範囲内に含まれる `AC` の文字列を数える問題
  - 全部数えてたらTLEになるくらいの範囲になりうる
    - それまでにいくつ `AC` の文字列が含まれていたかを数えておいたmapを作成する
    - 終点 - 始点 で範囲内に含まれる数がわかってループさせなくて済む

- [D - Flipping Signs](/abc125/d.rs)
  - Flipすると2ずつ数字が変わるため+, -のそれぞれ合計数の偶数奇数は変わらない
  - 奇数が偶数個なら奇数を0個(偶数個)にできる
  - 奇数個なら一番小さいものを-にすればいい

### abc129

- [C - Typical Stairs](/abc129/c.rs)
  - 穴の空いた階段を踏まないように階段を上るパターンの総数を求める問題
  - 穴が開いていなかったらフィボナッチ数列でもとまることを生かして
空いていたら値を0にすればO(N)で解ける

### abc131

- [C - Anti-Division](/abc131/c.rs)
  - a-bの範囲でc,dで割り切れない値の個数を調べる
  - b - b/c - b/d + b/lcm(c,d) - aの分とすればできる
  - 割り切れない値を列挙するために最小公倍数を足さなきゃいけないことに気づくのに時間かかった
- [D - Megalomania](/abc131/d.rs)
  - タプルを指定した要素でソートする
  - 期限までにすべての仕事が終わらせられるかを出力する

### abc136

- [D - Gathering Children](/abc136/d.rs)
  - 無限に近い回数動かすと場所が集約するR, Lのパネルを用いた問題
  - Rのパネルの値がすべてRに、Lのパネルの値がすべてLに移動する
  - このときRから偶数個めの値は偶数回動かした時にRの位置、
  - Lから偶数個目の値が偶数回動かしたときにLの位置にいることを用いて答えを求める

### abc144

- [B - 81](/abc144/b.rs)
  - Nを 1~9 の a, b の積で表せるかを調べる問題
  - forで回して総当たりすればOK
- [C - Walk on Multiplication Table](/abc144/c.rs)
  - a * b = n となる a + b - 2 のうち最小のものを求める問題
  - n ^ 12 なので aを a*a < nの範囲において b = n/aで求めることによって O(n^6)で探索できる

### abc146

- [A - Can't Wait for Holiday](/abc146/a.rs)
  - 日曜日まであと何日かを求める
  - 配列に曜日の一覧を入れちゃえばインデックスを出力するだけ
- [B - ROT N](/abc146/b.rs)
  - 文字列があたえられるのでRODを求めるやつ
  - `u8` にして65たしたり26でmodとったりたりする
- [C - Buy an Integer](/abc146/c.rs)
  - 2分探索を作る問題
  - 条件に当てはまる値を2文探索で探す
  - 数値の桁数を調べる関数

### abc148

- [E - Double Factorial](/abc148/e.rs)
  - それまでの数字をどんどんかけて0がいくつつくかを調べる問題
  - 10 を作るには5 x 2 が必要なことを生かして5x2で割れる数、5x5x2で割れる数...をどんどん足していくと解ける

### abc149

- [D - Prediction and Restriction](/abc149/d.rs)
  - じゃんけんバトル、特定回数前と同じ手を出すことはできない

### abc150

- [A - 500 Yen Coins](/abc150/a.rs)
  - K枚500円玉を持っているときの総額がx円以上か調べる
- [B - Count ABC](/abc140/b.rs)
  - 文字列の中に'ABC' の文字列がいくつ含まれているか調べる
- [C - Count Order](/abc150/c.rs)
  - 数列を入れ替えて特定の数列になった大きさを調べる
  - Permutation の使い方 Itertools を使えるようにすると使える

### abc152

- [C - Low Elements](/abc152/c.rs)
  - これまでにでてきた数字の最小の値をメモしておいてそれより小さかったら+=1して更新

### abc153

- [C - Fennec vs Monster](/abc153/c.rs)
  - フェネックが使える必殺技の回数が決まっており、それでつよい敵たちを倒す
  - のこりの敵を倒すために必要な攻撃回数を数える
- [D - Caracal vs Monster](/abc153/d.rs)
  - 攻撃するとHPが半分に分裂していく敵を倒すゲーム
  - 再帰的に半分にしていって何体の敵がいるかに合わせてたす数を変えていけばいい

### abc155

- [A - Poor](/abc155/a.rs)
  - 仲間外れの数字があればかわいそうな組として表示
- [B - Papers, Please](/abc155/b.rs)
  - かつ、またはの否定を求める問題
  - 偶数で3か5で割り切れない数字があったらFalse
- [C - Poll](/abc155/c.rs)
  - 文字列がたくさん与えられるので一番多く出てきた文字列を辞書順ですべて表示する問題
  - HashMapに突っ込んでいって一番多く出現したものをベクタに突っ込む、ソートして出力

### abc156

- [B - Digits](/abc156/b.rs)
  - 進数変換したら何桁になるかを求める
  - 進数変換はひたすら割り算するだけ！
- [C - Rally](/abc156/c.rs)
  - 総当たりで住民の減る体力が一番小さいものをみつけるだけ

### abc157

- [B - Bingo](/abc157/b.rs)
  - ビンゴに答える問題
  - マスに書かれた数字、言われた番号から穴の空いたとこマップを作成
  - 縦、横をループさせ斜めをみて列ができるか調べる
- [C - Guess The Number](/abc157/c.rs)
  - 何桁目が何という情報が与えられるのでその数字ができるか、最も小さいものを求める
  - 条件
    - 1桁目が0じゃないこと
    - 数字が上書きされないこと
    - 全体が1桁の時は0が最小なこと

### abc158

- [B - Count Balls](abc158/b.rs)
  - ボールを並べていくつ指定したボールがあるか
  - パターンが何回現れるかを数えて計算するタイプ、
  - 漏れた分は小さいものをえらんで
- [D - String Formation](/abc158/d.rs)
  - 文字列を入れ替えたりするのは大変なのでどちらが先頭化を記憶しておく
  - また前後に挿入する場合はVec_dequeをつかうようにしましょうという
  - あとそれをpirntするときはループで回すと遅いので `queue.iter().rev().collect::<String>()` がおすすめ
- [C - Tax Increase](/abc158/c.rs)
  - 消費税計算の問題
  - 8%のときと10%のときの消費税からもとの値段を予想する

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
- [D - Banned K](/abc159/d.rs)
  - N個のボールの組み合わせを数え上げる問題
  - HashMapに各値がいくつ存在したかを記録
  - マップからValueのイテレータをとってすべてにfoldで足し算を行うおしゃれテクを身に着けた

### abc162

- [A - Lucky 7](/abc162/a.rs)
  - 与えられたすうのいずれかの桁に7が含まれるかを調べる問題
- [B - FizzBuzz Sum](/abc162/b.rs)
  - FizzBuzz(3, 5)で割り切れない値のみの合計を出力する
- [C - Sum of gcd of Tuples (Easy)](/abc162/c.rs)
  - 最大公約数を返す関数を用いて3つの値の最大公約数を求める
- [D - RGB Triplets](/abc162/d.rs)
  - 文字列のなかから3つの文字を一つづつ選ぶパターンを数え上げる(r * g * b)
  - さらにそこから条件(j - i != k - j)の組み合わせを引く(j - i == k - jからkを求める二次式を作る)

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

### abc169

- [A Multiplication 1](/abc169/a.rs)
- [B Multiplication 2](/abc169/b.rs)
  - ひたすら掛け算した値を作成する
  - ベクタに0が含まれているかを探す `.find(|x| x == &&0)`
  - なければひたすら掛け算 & 境界チェックを行う
- [C Multiplication 3](/abc169/c.rs)
  - 掛け算結果の小数点以下を切り捨てて出力する問題
  - // 1.19 -> 118.999 ... + 0.5 -> 119.222
  - // 1.18 -> 118.000
  - let bb = (b * 100.0 + 0.5).floor() as u64;
  - dobule値のずれのことも考えて +0.5 とかしてあげる
- [D Div Game](/abc169/d.rs)
  - 素因数分解していく問題
  - HashMapで素因数分解を返してそれをもとに解く

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
- [D - Replacing](/abc171/d.rs)
  - 指定した値を指定した値に入れ替えていく問題だが全部いれかえて計算すると間に合わない
  - まずすべての合計を調べておいていくつの数字が入れ替わったかで合計を計算で求める
- [E - Red Scarf](/abc171/e.rs)
  - XORを同じ値に対して奇数回しても変わらないことを生かして全体のXORをとってその他の値とXORをとることで自身の値がわかる
  - 数学的問題

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

### abc179

- [A - Plural Form](/abc179/a.rs)
  - 文字列の末端をみて付け足して出力する
- [B - Go to Jail](/abc179/b.rs)
  - モノポリー問題
  - さいころを3回続けてぞろ目にしたときに表示する
- [C - A x B + C](/abc179/c.rs)
  - 計算パズル系
  - 3変数で特定の値を作ろうとしてるように見えて実は1変数で何個の組み合わせを作れるかはわかる
- [D - Leaping Tak](/abc179/d.rs)


### abc178

- [A - Not](/abc178/a.rs)
- [B - Product Max](/abc178/b.rs)
  - 最大値を作る組み合わせを考える問題
  - 考えられる最大値のパターンが少ないことを利用して総当たりする
- [C - Ubiquity](/abc178/c.rs)
  - 計算で求める式を求め実装する
  - ネックなのは大きくなったら 10^9+7 でmodをとるところ
  - 一番大きなものだけ取っちゃうと符号が-になることを利用してその分 10^9+7 を足して補正する

### abc180

- [A - box](/abc180/a.rs)
- [B - Various distances](/abc180/b.rs)
- [C - Cream puff](/abc180/c.rs)
- [D - Takahashi Unevolved](/abc180/d.rs)

### abc181

- [A - Heavy Rotation](/abc181/a.rs)
  - 白服、黒服を交互に切ることになるのでN日後が偶数後日目か奇数後日目かを求めて出力
- [B - Trapezoid Sum](/abc181/b.rs)
  - A~Bの値を順番に書いていくをN回繰り返す
  - A~Bの整数の合計は (A + B) * (B - A + 1) / 2
- [C - Collinearity](/abc181/c.rs)
  - 2次元上に配置された点たちのなかで3点が同一直線状に乗っているものがあるかを調べる
  - 同一直線状 -> 2点のベクトルを求めて n倍したらもう一つの頂点にたどり着く
  - 比にして割り算をしないように掛け算の形にすること x, y の n が一致するならばOK
- [D - Hachi](/abc181/d.rs)
  - 与えられた数字を並び変えて8の倍数を作ることができるか
  - 8の倍数は 下三桁が8の倍数ならばすべてが8の倍数
  - 与えられた数字をHashmapにいれて数字の出現回数を最大3で数える
  - Char型と数字の変換にc2uとかいうの使ったけど `char.to_digit(n)` でn進数に変換できるみたい強い
  - 組み合わせ列挙 
    - ```
      numbers.into_iter().permutations(3).any(|perm| {
          let num = perm[0] + perm[1] * 10 + perm[2] * 100;
      }
      ```
- [E - Transformable Teacher](/abc181/e.rs)
  - 先生の最適な身長を2文探索で求める
  - それまでの生徒の身長差合計とその後の身長差合計をあらかじめ累積和で求めておいて時短化させる

### abc182

- [A - twiblr](/abc182/a.rs)
  - フォロー数に制限があってフォロワー数からどうやって計算するか
- [B - Almost GCD](/abc182/b.rs)
  - 日本語が難しい問題
  - 与えられた数列をできるだけたくさん割り切れる値を探す問題
- [C - To 3](/abc182/c.rs)
  - 与えられた数字列から数字をいくつかけして3の倍数を作る
  - 3の倍数は与えられた数字の各桁合計が3の倍数ならよいことを生かしあまり1なら1あまりのものか2のもの2つを消し～～2なら2のものまたは1のもの二つ
- [D - Wandering](/abc182/.rs)
  - ロボットが移動していくのでいかに遠い場所に行けるか問題
  - 各工程でどこにたどり着くか、各回で一番遠いところはどこかをメモしておくことでOを下げることができる

### abc183

- [A - ReLU](/abc183/a.rs)
  - 0以上の値ならそのまま出力、以下ならば0を出力する関数を作る
- [B - Billiards](/abc183/b.rs)
  - ビリヤードで反射させて目的地を通す問題
  - Y軸の比から間の座標を求めて出力する
- [C - Travel](/abc183/c.rs)
  - 都市間を移動してすべての年を通る際のコストを見積もる問題
  - 順列を作ってくれる `permutations` を用いて実装した
- [D - Water Heater](/abc183/d.rs)
  - お湯を使える量が決まっており、すべての人にお湯が供給できるか求める問題
  - どの時間にどれだけお湯がなくなって増えるかをまずすべて一つのベクタにまとめ
  - 最初から見ていって使う量が使える量を越えた場合にはNo

### abc184

- [A - Determinant](/abc184/a.rs)
  - 行列式を計算する問題
- [B - Quizzes](/abc184/b.rs)
  - 初期得点が与えられてクイズに正解したら点数が増えて不正解で減る
  - 0よりは得点が小さくならない
- [C - Super Ryuma](/abc184/c.rs)
  - 条件3つを使って動く角みたいな駒がある
  - 何回で目的の場所まで移動できるか
  - 1つ1つ場合分けをしっかりできるかの問題
  - 斜め移動は飛ばすところがいくつかでてくるからそこはmodが一致するか次第でわかる

### abc185

- [A - ABC Preparation](/abc185/a.rs)
- [B - Smartphone Addiction](/abc185/b.rs)
- [C - Duodecim Ferra ](/abc185/c.rs)
- [D - Stamp](/abc185/d.rs)
- [E - Sequence Matching](/abc185/e.rs)

### abc192

- [A - Discount](/abc192/a.rs)
  - 何割引きか計算する問題
- [B - Play Snuke](/abc192/a.rs)
  - どの店に行くのが一番安くものが買えるか調べる問題
- [C - Unexpressed](/abc192/a.rs)
  - 整数Nのうちa^bで表せない値の数を調べる問題
  - a^bで表せないものがすくないことを生かしてHashSet使っていい

### abc200

- [A - Century](/abc200/a.rs)
  - 切り捨ての位置をずらすために割る数-1を足してから割り算を行う
- [B - 200th ABC-200](/abc200/b.rs)
  - 処理を繰り返すのみ
- [C - Ringo's Favorite Numbers 2](/abc200/c.rs)
  - 10^9個の数字が与えられるのでそのうち組み合わせとして差が200の倍数になるものを探す
  - それまでに何回modが同じになる値が来ているかを数える
  - 新しいあまりができたらHashMapに1をいれてまた同じのが来たら+1していって数えた
  - 別にHashMapにする必要は200程度だしなかったなと...
- [D - Happy Birthday!](/abc200/d.rs)
  - 余りが被る組み合わせを見つける
  - 2^8-1 通り(255通り)調べたら絶対にmod200の世界では被りが発生するよね??
  - 8個目までをbit全探索していけばOK
  - dp復元する実装があるらしい...


## others

### joi2008ho

- [C - ダーツ](/joi2008ho/c.rs)
  - 4本までダーツを投げて得点Mを越えない最大得点を計算する問題
  - 2本総当たり順列を作って各値についてもう2本選んで最もMに近くなるものを2分探索で求める

### code-festival-2017-quala

- [B - fLIP](/code-festival-2017-quala/b.rs)
  - オセロ的なひっくり返していくゲーム
  - どちらの列を選択したらいくつひっくり返るかを式にすると解ける

### CADDi 2018

- [C - Product and GCD](/caddi2018/c.rs)
  - 与えられた数列全部の掛け算した値がわかっているのでそこから数列の最大公約数(GCD)を求める問題
  - Factor(素因数分解)を求めて数列の数ぶん以上あるものをどんどんかけていけば求まる

### keyence2019

- [C - Exam and Wizard](/keyence2019/c.rs)
  - できるだけ少ない数の準備度を入れ替えて求められている試験度に達成する
  - 点数がたりないところを埋めるだけの点数をどこから用意するかを最小にする
  - 点数の余っているところを配列に入れてって大きいものから必要な点数をもらっていけばいい

### みんなのプロコン2019

- [C - When I hit my pocket...](/yahoo-procon2019-qual/c.rs)
  - クッキーを割っては変換する問題
  - ただただ計算式を思いつけるかどうかが問われる

### keyence2020

- [C - Subarray Sum](/keyence2020/c.rs)
  - 問題の意味がわかればいいゲーム
  - 制約をもとに埋める値を変えたりしないといけないのは難しいところ
  - 10^9が与えられた時だけ制約を超えた値を入れないようにしたりとか
    - 出力長制限超過OLE とかいうのを見たらこれを疑え！

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

### hhkb2020

- [A - Keyboard](/hhkb2020/a.rs)
  - 指定された方法で文字列を変更して出力する、
  - 文字を大文字にしたり小文字のまま出力したり
  - もっとおしゃれにかけよってかんじの反省
- [B - Futon](/hhkb2020/b.rs)
  - 与えられたMapのうち2つ並んで空いている場所をさがして布団をしく
  - 右だけまたは下だけを見るを繰り返す
  - 最後の行と列は別で足す
- [C - Neq Min](/hhkb2020/c.rs)
  - 使ってはいけない値をHashMapに加えていく、
  - もしminが使ってはいけないリストに入っていたなら+1して入っていない状態までもっていく
- [D - Squares](/hhkb2020/d.rs)
  - 正方形の置くパターンがいくつあるかを数え上げる問題
  - シグマのループが (1 ~ Nまでの総和) x R と気づいたらこれはO(1)で求められるという
  - またこれが二つあったとしても (x) x (y) で分けられているならこれは後からかけてもいいという
  - 配置をするときは置く場所を区切って考えるとよい、
    - どこかに縦線を引いて右に1つあってもう一つは左にある的な
