#include <iostream>
#include <vector>

using namespace std;

int solve(int sum, const vector<int> &arr);

int main() {
    int N, S, input;

    vector<int> vec;

    cin >> N >> S;

    for (int i = 0 ; i < N; i++) {
        cin >> input;

        vec.push_back(input);
    }

    int ans = solve(S, vec);

    cout << ans << "\n";

    return 0;

}

int solve(int sum, const vector<int> &arr) {
    int ele_size = arr.size();
    int subset = 0;

    vector<int> subvec;

    for (int i = 0; i < (1 << (ele_size)); i++) {
        int check = 0;
        for (int j = 0 ; j < ele_size; j++) {
            if (i & (1 << j)) {
                check += arr[j];
                subvec.push_back(arr[j]);
            }
        }

        if (check == sum && subvec.size() >= 1) {
            subset++;
        }

        subvec.clear();
    }

    return subset;
}