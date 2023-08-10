# Cargo binary
CARGO = cargo

# Cargo flags
FLAGS = --release --all-features

all:
	$(CARGO) build $(FLAGS)

bootstrap:
	git submodule update --init --recursive

test:
	$(CARGO) test $(FLAGS)

clean:
	$(CARGO) clean

.PHONY: all bootstrap clean test
