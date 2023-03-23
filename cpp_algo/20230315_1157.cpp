#include <iostream>
#include <string>

using namespace std;

int main() {

    int cnt[26]{0}, upper{0};

    int max_idx{0}, max_cnt{0};

    string str;

    cin >> str;

    for (int i = 0 ; i < str.length(); i++) {
        if (str[i] < 97) {
            cnt[str[i] - 65]++;
        }
        else {
            cnt[str[i] - 97]++;
        }
    }

    for (int i = 0 ; i < 26; i++) {
        if (max_cnt < cnt[i]) {
            max_cnt = cnt[i];
            max_idx = i;
        }
    }

    for (int i = 0 ; i < 26; i++) {
        if (cnt[i] == max_cnt) {
            upper++;
        }
    }

    upper >= 2 ? cout << "?" << "\n" : cout << (char)(max_idx + 65) << "\n";

    return 0;
}