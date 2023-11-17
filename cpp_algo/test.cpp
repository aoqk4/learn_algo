#include <iostream>

void test();

int main() {
    test();
}

void test() {
    int a = 100;

    int* b = &a;

    delete b;


}