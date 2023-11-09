#include <iostream>
#include <queue>

int main() {

    std::ios::sync_with_stdio(false);
    std::cin.tie(NULL);
    std::cout.tie(NULL);

    short n{0};

    std::cin >> n;

    std::priority_queue<int32_t, std::vector<int32_t>, std::greater<int32_t>>    pque;

    for (int row{0} ; row < n; row++) {
        for (int col{0} ; col < n; col++) {
            int32_t temp{0};

            std::cin >> temp;

            pque.push(temp);

            if (int32_t(pque.size()) > n) {
                pque.pop();
            }
        }
    }
    std::cout << pque.top() << "\n";

    return 0;
}