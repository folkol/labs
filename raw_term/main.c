#include <stdlib.h>
#include <unistd.h>
#include <stdio.h>
#include <termios.h>


struct termios original;

void reset_termios() {
    tcsetattr(STDIN_FILENO, TCSAFLUSH, &original);
}

int main() {
    struct termios raw;
    tcgetattr(STDIN_FILENO, &original);
    atexit(reset_termios);
    raw = original;
    raw.c_lflag &= ~(ECHO);
    tcsetattr(STDIN_FILENO, TCSAFLUSH, &raw);

    char c;
    while (read(STDIN_FILENO, &c, 1) == 1) {
        if(c == 'q') {
            break;
        }
        printf("%c\r\n> ", c);
    }
    return 0;
}
