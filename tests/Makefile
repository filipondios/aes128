CC=gcc
CFLAGS=-Wall -Wextra -march=native -mtune=native
SRCS=../aes128.c ./test.c
OBJS=$(SRCS:.c=.o)
TARGET=exec

all: $(TARGET)

$(TARGET):
	$(CC) $(CFLAGS) $(SRCS) -o $(TARGET)

clean:
	rm -f $(OBJS) $(TARGET)

test: clean all
	./$(TARGET)

.PHONY: clean test


