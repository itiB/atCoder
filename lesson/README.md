# LESSON

## DPまとめコンテスト

- [A - Frog 1](./dp/a.rs)
  - dpを用いる問題初心者編
  - どこから足場がくるかをもとにループを回す
- [B - Frog 2](./dp/b.rs)
  - aでやった内容をもとに今度はdpを求める範囲を決めたもの
  - 今のところからいくつ前までは計算対象に入るから~~~でdpを作る
- [C - Vacation](./dp/c.rs)
  - 3択を選んでいって最大となる組み合わせを探す問題
  - 1日進むたびにどの選択肢からきたものがそれぞれのルートで最大の幸福値を得られるかを記録するDP
- [D - Knapsack 1](./dp/d.rs)
  - ナップサックdp
  - どの荷物を入れれば重さ以内で最大の価値を作ることができるか
  - i番目のものが重さj以内ならば入れる[n個のもの][重さ]の2次元配列を作る
  - ものを入れれ場合は `max(入れなかった場合の重さ, 入れる前 + 入れるものの重さ)`
  - 参考: https://qiita.com/drken/items/a5e6fe22863b7992efdb
- [E - Knapsack 2](./dp/e.rs)
  - ナップサックの容量最大が1_000_000_000だぁぁぁ無理、メモリ足りな
  - でも価格が1000で済んでいる、個数は100まで -> こっちをベクトル化すればよさそう
  - いくつめの荷物かと価格でDPを組んで内容は最小となる重さにした
- [F - LCS](./dp/f.rs)
  - 部分文字列を求める問題、2次元のDPを作る
  - 一致する文字な場合は前のDP(x, yそれぞれのDP)+1
  - 逆にたどる時は一致する文字列が着たら斜めに戻る
  - それ以外の時は左に戻ってチェック、上にいってチェックを繰り返す
- [G - Longest Path](./dp/g.rs)
  - 有効グラフが与えられ、その中で最も長い経路が作れる道を探す。
  - 深さ優先探索をして戻ることで依存関係を調べることができる(トポロジカルソート)
  - 戻る際に距離をdpに保存しておき、各葉を回る時にどの葉が最も距離が長いかを調べてDPにつないでいく
- [H - Grid 1](./dp/h.rs)
  - マスが与えられていけない箇所がある。
  - ゴールまで何通りの方法でたどり着けるか調べる問題
- [I - Coins](./dp/i.rs)
  - 確率DP
  - コインを投げて表の数が裏の数を越える確率を求める。
  - dpとしては投げた個数と何個表になったかの確率でDPを作る。
  - n個投げた行のうち表の数が指定数を越えてる列を足し算する
- [J - Sushi](./dp/j.rs)
  - 皿にのったSushiを食べて全部食べきるためのさいころ振る期待値を調べる問題
  - 1,2,3のった皿の個数を考え最初は0,0,0からはじめて1つ乗った皿を増やしまくる
  - 次に2つ乗ったのを考えて...な3次元DPを組む
  - この時各回のさいころの期待値を記録していく
  - 1,2,3つ乗った皿のそれぞれ0~n個で作ったDPテーブルから指定された場所をみれば答えがわかる
- [K - Stones](./dp/k.rs)
  - 交互に石をとっていって最初に操作できなくなったほうが敗けのゲーム
  - 取れる個数のリストが渡されるので先手が先に到達できるところをもとにTFのDPをする
  - 一手前が全部先手が到達するところならそこは絶対に後手しか先に到達できないところとなる
- [L - Deque](./dp/l.rs)
  - 区間DPと呼ばれるもの
  - 配列が与えられて前後どちらかを交互に取っていく
  - 交互に取っていった際の特典をできる限り短い範囲の状態から始めてとりうる点数を調べるDPを作る
- [M - Candies](./dp/m.rs)
  - 計算量がオーバーするタイプのDP
  - 繰り返し足すことになる区間がある → 累積和を用いる
  - 累積和は累積区間終点から累積区間の始まり-1を引くことに注意！

## typical DP Contest

- [A - コンテスト](./typicalDp/a.rs)
  - コンテストので獲得しうる点数パターンをすべて列挙する問題
  - 0問正解で0点を初期値として一問目正解した場合、しなかった場合で取りうる点数列挙
  - 2問目正解したら取りうる点数を足していってーーーを繰り返す
  - 最後の問題で取りうる点数パターンがすべて並ぶのでいくつが取れるかを数え上げる

## 競プロ典型90問

- [008 - AtCounter（★4）](./typical90/008.rs)
  - 与えられた文字列から指定の文字列が何通り作れるか調べる問題
  - DPでその文字を選ぶか選ばないかのDPを作る。
  - その際に文字列がどこまでできるかを記録していく
- [050 - Stair Jump（★3）](./typical90/050.rs)
  - DPで何通りその階段に行く術があるかを調べる問題

- [056 - Lucky Bag（★5）](./typical90/056.rs)
  - 2種類の商品のうちどちらかを購入してちょうどS円にする
  - どちらを購入したかDP復元して購入順に並べる
  - 復元用数列を作ると上書きされたタイミングでずれたりするので作らずにサーチする
  - 0円を下回ったら違う商品を買ったなとあたりをつける