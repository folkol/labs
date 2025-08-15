#include <iostream>
#include <map>
#include <deque>

int main() {
    std::map<int, std::deque<char>> stacks;
    for (std::string line; std::getline(std::cin, line) && line.length();) {
        for (auto i = 0, stack = 1; i < line.length(); i += 4, stack += 1) {
            if (line[i] == '[') {
                stacks[stack].push_front(line[i + 1]);
            }
        }
    }

    while (std::cin.good()) {
        int n, from, to;
        std::cin.ignore(5) >> n;
        std::cin.ignore(6) >> from;
        std::cin.ignore(4) >> to;
        for (auto i = 0; i < n; i++) {
            auto item = stacks[from].back();
            stacks[from].pop_back();
            stacks[to].push_back(item);
        }
    }
    for (auto[k, v]: stacks) {
        std::cout << v.back();
    }
    std::cout << std::endl;
}
