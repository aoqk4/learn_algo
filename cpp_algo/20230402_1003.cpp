#include <iostream>
#include <vector>

using namespace std;

int main()
{
    ios::sync_with_stdio(false);
    cin.tie(NULL);

    vector<long long int> res;

    int T;
    cin >> T;

    int n;

    for (int i = 0; i < 41; i++)
    {
        if (i == 0)
        {
            res.push_back(0);
        }
        else if (i == 1)
        {
            res.push_back(1);
        }
        else
        {
            res.push_back(res[n - 1] + res[n - 2]);
        }
    }

    for (int i = 0; i < T; i++)
    {
        cin >> n;

        if (n == 0)
        {
            cout << "1 0"
                 << "\n";
        }
        else
        {
            cout << res[n - 1] << ' ' << res[n] << "\n";
        }
    }

    return 0;
}