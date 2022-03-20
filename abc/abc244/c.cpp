
#include <bits/stdc++.h>

using namespace std;

int main(){
  int n;
  cin >> n;

  vector<bool> ans(2*n+1, false);
  for(int i = 0; i <= n; i++) {
      for (int j=1; j <= 2*n+1; j++) {
          if (ans[j] == false) {
              cout << j << endl;
              std::flush(std::cout);
              ans[j] = true;
              break;
          }
      }
      int a;
      cin >> a;
      ans[a] = true;
  }
  return 0;
}
