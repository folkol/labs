#include <unistd.h>
#include <stdio.h>
#include <termios.h>


int main() {
    struct termios raw;
    tcgetattr(STDIN_FILENO, &raw);
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
