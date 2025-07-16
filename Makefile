CC=zig cc
CFLAGS=
OUTPUT=./build/canary

SOURCE= \
	./source/main.c \
	./source/canary.c \
	./source/lexer/lexer.c \
	./source/lexer/token.c

VENDOR= \
	./vendor/flag.c

build:
	$(CC) -o $(OUTPUT) $(SOURCE) $(VENDOR)

all: build clean

.PHONY: 
	all

clean:
	rm -rf $(OUTPUT)
