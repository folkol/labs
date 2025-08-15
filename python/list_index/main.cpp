#include <iostream>
#include <vector>
#include <fstream>

using namespace std;

vector<string> read_lines(const char *filename) {
    vector<string> words;

    ifstream f(filename);
    for (string line; getline(f, line);) {
        words.emplace_back(line);
    }
    return words;
}

int main(int argc, char *argv[]) {
    auto words = read_lines("words.txt");
    auto shuffled = read_lines("shuffled_words.txt");

    vector<long> indexes;
    auto begin = chrono::system_clock::now();

    for (const auto &word : shuffled) {
        auto iter = find(words.begin(), words.end(), word);
//        printf("%ld\n", iter - words.begin());
//        indexes.emplace_back(iter-words.begin());
    }

    chrono::duration<double> elapsed = chrono::system_clock::now() - begin;

    for (const auto &i : indexes) {
        printf("%ld\n", i);
    }

    printf("%.2f\n", elapsed.count());
}
