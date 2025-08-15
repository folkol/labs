#include <iostream>
#include <vector>
#include <fstream>

using namespace std;

vector<const char *> read_lines(const char *filename) {
    vector<const char *> words;

    ifstream f(filename);
    for (string line; getline(f, line);) {
        words.push_back(strdup(line.c_str()));
    }
    return words;
}

int main(int argc, char *argv[]) {
    auto words = read_lines("words.txt");
    auto shuffled = read_lines("shuffled_words.txt");

    auto begin = chrono::system_clock::now();

    auto counter = 0;
    for (const auto &word : shuffled) {
        int i;
        for (i = 0; i < words.size(); i++) {
            if (strcmp(word, words[i]) == 0) {
                //printf("%d\n", i);
                break;
            }
        }
        //if(i == words.size()) {
            //printf("WARNING: Couldn't find %s\n", word);
        //}
    }

    chrono::duration<double> elapsed = chrono::system_clock::now() - begin;

    printf("%.2f\n", elapsed.count());
    return 0;
}
