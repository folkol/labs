#include <ctype.h>
#include <errno.h>
#include <stdlib.h>
#include <unistd.h>
#include <stdio.h>
#include <termios.h>
#include <sys/ioctl.h>


#define CTRL_KEY(k) ((k) & 0x1f)

struct cfg {
    struct termios original;
    int screen_rows;
    int screen_cols;
} cfg;

void clear_screen() {
    write(STDOUT_FILENO, "\x1b[2J", 4);
    write(STDOUT_FILENO, "\x1b[H", 3);
}

void die(const char *s) {
    clear_screen();

    perror(s);
    exit(1);
}

void redraw() {
    for(int y = 0; y < 24; y++) {
        write(STDOUT_FILENO, "~\r\n", 3);
    }
}

void refresh_screen() {
    write(STDOUT_FILENO, "\x1b[?25l", 6);
    clear_screen();
    redraw();
    write(STDOUT_FILENO, "\x1b[H", 3);
    write(STDOUT_FILENO, "\x1b[?25h", 6);
}

void reset_termios() {
    fprintf(stderr, "Resetting termios\n");
    if(tcsetattr(STDIN_FILENO, TCSAFLUSH, &cfg.original) == -1)
        die("tcsetattr");
}

void configure_termios() {
    fprintf(stderr, "Configuring termios\n");
    struct termios raw;
    if(tcgetattr(STDIN_FILENO, &cfg.original) == -1)
        die("tcgetattr");
    atexit(reset_termios);
    raw = cfg.original;
    raw.c_lflag &= ~(ICRNL | IXON);
    raw.c_lflag &= ~(OPOST);
    raw.c_lflag &= ~(ECHO | ICANON | IEXTEN | ISIG);
    raw.c_cc[VMIN] = 0;
    raw.c_cc[VTIME] = 1;
    if(tcsetattr(STDIN_FILENO, TCSAFLUSH, &raw) == -1)
        die("tcsetattr");
}

int get_wnd_size(int *rows, int *cols) {
    struct winsize ws;
    if(ioctl(STDOUT_FILENO, TIOCGWINSZ, &ws) == -1 || ws.ws_col == 0) {
        return -1;
    } else {
        *cols = ws.ws_col;
        *rows = ws.ws_row;
        return 0;
    }
}

void init() {
    if(get_wnd_size(&cfg.screen_rows, &cfg.screen_cols) == -1)
        die("get_wnd_size");
    printf("%d x %d\r\n", cfg.screen_rows, cfg.screen_cols);
}

int main() {
    configure_termios();
    init();
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
                    clear_screen();
                    running = 0;
                    break;
                case CTRL_KEY('k'):
                    refresh_screen();
                    break;
                default:
                    printf("%c\r\n", c);
            }
        }
    }
    return 0;
}
