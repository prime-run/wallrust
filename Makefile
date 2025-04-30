prog := wallrust
local_bin := ./bin
user_bin := ~/.local/bin

release := --release
target := release

.PHONY: build install clean all help

build:
	cargo build $(release)

$(local_bin):
	mkdir -p $(local_bin)

local-install: build $(local_bin)
	cp target/$(target)/$(prog) $(local_bin)/$(prog)

install: local-install
	mkdir -p $(user_bin)
	cp $(local_bin)/$(prog) $(user_bin)/$(prog)

clean:
	cargo clean
	rm -rf $(local_bin)

all: build install

help:
	@echo "Usage: make [target]"
	@echo ""
	@echo "Targets:"
	@echo "  build         - Build the $(prog) binary (release mode)"
	@echo "  local-install - Copy binary to ./bin directory"
	@echo "  install       - Copy binary to ~/.local/bin"
	@echo "  clean         - Clean cargo build artifacts (keeps binaries in ./bin)"
	@echo "  all           - Build and install (default)"
	@echo "  help          - Show this help message"

.DEFAULT_GOAL := all
