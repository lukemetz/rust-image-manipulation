all: main

SRC = src/main.rs

test: $(SRC)
	rustc src/main.rs --test -o test -L lib/rust-png
main: $(SRC)
	rustc src/main.rs -o main -L lib/rust-png --opt-level=3

.PHONY: clean

clean:
	rm -f test
	rm -f main
