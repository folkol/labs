#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>
#include <sys/time.h>

const int N = 1000000;

char *words[N];
char *shuffled_words[1000];

int main(void)
{
    struct timeval t;

    int j = 0;

    FILE *fp;
    char *line;
    size_t len;
    ssize_t read;
    int num_words;

    fp = fopen("words.txt", "r");
    while ((read = getline(&line, &len, fp)) != -1) {
        words[j++] = strdup(line);
    }
    fclose(fp);
    num_words = j;

    fp = fopen("shuffled_words.txt", "r");
    j = 0;
    while ((read = getline(&line, &len, fp)) != -1) {
        shuffled_words[j++] = strdup(line);
    }
    fclose(fp);

    long pos = 0;
    gettimeofday(&t, NULL);
    long begin = t.tv_sec * 1000000 + t.tv_usec;
    for(int i = 0; i < 1000; i++) {
        for(int j = 0; j < num_words; j++) {
            if(strcmp(shuffled_words[i], words[j]) == 0) {
                pos += j;
                break;
            }
        }
    }
    gettimeofday(&t, NULL);
    long elapsed = (t.tv_sec * 1000000 + t.tv_usec) - begin;

    printf("%f\n", elapsed / 1e6);

    exit(EXIT_SUCCESS);
}
