#include <ctype.h>
#include <errno.h>
#include <stdlib.h>
#include <unistd.h>
#include <stdio.h>
#include <termios.h>


#define CTRL_KEY(k) ((k) & 0x1f)

struct termios original;

void die(const char *s) {
    perror(s);
    exit(1);
}

void reset_termios() {
    fprintf(stderr, "Resetting termios\n");
    if(tcsetattr(STDIN_FILENO, TCSAFLUSH, &original) == -1)
        die("tcsetattr");
}

void configure_termios() {
    fprintf(stderr, "Configuring termios\n");
    struct termios raw;
    if(tcgetattr(STDIN_FILENO, &original) == -1)
        die("tcgetattr");
    atexit(reset_termios);
    raw = original;
    raw.c_lflag &= ~(ICRNL | IXON);
    raw.c_lflag &= ~(OPOST);
    raw.c_lflag &= ~(ECHO | ICANON | IEXTEN | ISIG);
    raw.c_cc[VMIN] = 0;
    raw.c_cc[VTIME] = 1;
    if(tcsetattr(STDIN_FILENO, TCSAFLUSH, &raw) == -1)
        die("tcsetattr");
}

int main() {
    configure_termios();
    char c;
    int running = 1, n;
    while (running) {
        if(n = read(STDIN_FILENO, &c, 1), n == -1 && errno != EAGAIN)
            die("read");
        if(n > 0) {
            switch(c) {
                case 3:
                    printf("lol no\n");
                    break;
                case 26:
                    // suspend
                case CTRL_KEY('q'):
                    running = 0;
                    break;
                default:
                    printf("%c\r\n> ", c);
            }
        }
    }
    return 0;
}
