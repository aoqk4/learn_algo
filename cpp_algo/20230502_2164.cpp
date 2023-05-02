#include <iostream>
#include <stack>
#include <queue>

using namespace std;

int main() {
	queue<int> arr;
	int N;
	cin >> N;
	for (int i = 1; i <= N; i++) {
		arr.push(i);
	}

	while (arr.size() != 1) {
    	arr.pop();
		arr.push(arr.front());
		arr.pop();
	}

	cout << arr.front();

	return 0;
}