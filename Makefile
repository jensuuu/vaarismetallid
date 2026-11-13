# Väärismetallide programm 


NIMI := vaarismetallid

.PHONY: help build run test linux windows paketid install uninstall clean

help: 
	@echo "Käsud:"
	@grep -E '^[a-z]+:.*## ' $(MAKEFILE_LIST) | awk -F':.*## ' '{printf "  make %-10s %s\n", $$1, $$2}'

build: 
	cargo build --release

run: 
	cargo run --release

test: 
	cargo test

linux: 
	bash scripts/build-linux.sh

windows: 
	bash scripts/build-win.sh

paketid: 
	bash scripts/deps.sh

install: 
	bash scripts/install.sh

uninstall: 
	rm -f $(HOME)/.local/bin/$(NIMI)
	@echo "Eemaldatud."

clean: 
	cargo clean
	rm -rf dist


zip: 
	bash scripts/compress.sh
