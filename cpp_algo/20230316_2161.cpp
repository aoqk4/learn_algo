#include <iostream>
#include <vector>

using namespace std;

int main()
{

    int N;

    vector<int> vec;

    cin >> N;

    for (int i = 1; i <= N; i++)
    {
        vec.push_back(i);
    }

    while(true) {
        if (vec.size() == 1) {
            cout << vec[0] << "\n";
            break;
        }

        cout << vec[0] << " ";
        vec.erase(vec.begin());

        int temp = vec.front();
        vec.push_back(temp);
        vec.erase(vec.begin());
    }

    return 0;
}