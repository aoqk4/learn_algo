#include <iostream>
#include <string>
#include <vector>

using namespace std;

int main() {

    int alpha[26];

    string s;

    cin >> s;

    for (int i = 0 ; i < 26; i++) {
        alpha[i] = -1;
    }

    for (int i = 0; i < s.length(); i++) {
        if (alpha[(int)s[i] - 97] <= -1) {
            alpha[(int)s[i] - 97] = i;
        }
    }

    for (int i = 0 ; i < 26; i++) {
        cout << alpha[i] << ' ';
    }

    return 0;
}