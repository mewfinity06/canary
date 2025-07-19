CC=zig cc
CFLAGS= \
	-Wall \
	-Werror \
	-Wextra
OUTPUT=./build/canary

SOURCE= \
	./source/main.c \
	./source/canary.c \
	./source/lexer/lexer.c \
	./source/lexer/token.c \
	./source/parser/parser.c

VENDOR= \
	./vendor/flag.c

build:
	$(CC) -o $(OUTPUT) $(SOURCE) $(VENDOR) $(CFLAGS)

all: build clean

.PHONY: 
	all

clean:
	rm -rf $(OUTPUT)
