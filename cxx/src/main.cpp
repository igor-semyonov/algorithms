#include <cstdint>
#include <iostream>
#include <span>
#include <vector>

int main() {
    std::vector<int> data = {1, 2, 3, 4, 15};

    for (int element : data) {
        std::cout << element << "\n";
    }

    return 0;
}

void insertion_sort(std::span<int> input) {
    for (int &element : input) {
        element += 1;
    }
}
