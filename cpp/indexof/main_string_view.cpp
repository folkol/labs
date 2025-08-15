#include <iostream>
#include <vector>
#include <fstream>

using namespace std;

vector<string> read_lines(const char *filename) {
    vector<string> words;

    ifstream f(filename);
    for (string line; getline(f, line);) {
        words.push_back(line);
    }
    return words;
}

int main(int argc, char *argv[]) {
    auto words = read_lines("words.txt");
    auto shuffled = read_lines("shuffled_words.txt");

    auto begin = chrono::system_clock::now();

    for (const auto& word : shuffled) {
        auto it = find(words.begin(), words.end(), word);
        cout << "Found " << *it << " at " << (it - words.begin()) << "\n";
    }

    chrono::duration<double> elapsed = chrono::system_clock::now() - begin;

    printf("Took %.2f seconds\n", elapsed.count());
    return 0;
}
