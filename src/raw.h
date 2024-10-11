#include <errno.h>
#include <ctype.h>
#include <stdio.h>
#include <stdlib.h>
#include <termios.h>
#include <unistd.h>

void die(const char *s);
void enableRawMode();
void disableRawMode();
char read_char();
