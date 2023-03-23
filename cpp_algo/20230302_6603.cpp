#include<iostream>
#include<vector>

using namespace std;

void solve(int start, int depth, const vector<int> &arr, int (&result)[6]);

int main(void) {
    int k, input;
    vector<int> s;
    int result[6];
    while (true) {

        cin >> k;

        if (k == 0) {
            break;
        }
        
        for (int i = 0; i < k; i++) {
        cin >> input;
        s.push_back(input);
        }
        solve(0, 0, s, result);
        s.clear();
        cout << "\n";
    }

    return 0;    
}

void solve(int start, int depth, const vector<int> &arr, int (&result)[6]) {

    if (depth == 6) {
		for (int i = 0; i < 6; i++) {
			cout << result[i] << " "; 
		}
        cout << "\n";
        return;
	}
	for (int i = start; i < arr.size(); i++) { //백트래킹
        
        result[depth] = arr[i];
        solve(i + 1, depth + 1, arr, result);
	}
}