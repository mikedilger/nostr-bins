CARGO:=$(shell which cargo)
export CARGO
RUSTC:=$(shell which rustc)
export RUSTC
RUSTUP:=$(shell which rustup)
export RUSTUP

-:
	@awk 'BEGIN {FS = ":.*?## "} /^[a-zA-Z_-]+:.*?##/ {printf "\033[36m%-15s\033[0m %s\n", $$1, $$2}' $(MAKEFILE_LIST)
help:## 	help
	@sed -n 's/^##//p' ${MAKEFILE_LIST} | column -t -s ':' |  sed -e 's/^/ /'
rustup-install:rustup-install-stable## 	rustup-install
rustup-install-stable:## 	rustup-install-stable
##rustup-install-stable:
##	install rustup && rustup default stable
	$(shell echo which rustup) || curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --no-modify-path --default-toolchain stable --profile default && . "$(HOME)/.cargo/env"
	$(shell echo which rustup) && rustup default stable
rustup-install-nightly:## 	rustup-install-nightly
##rustup-install-nightly:
##	install rustup && rustup default nightly
	$(shell echo which rustup) || curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --no-modify-path --default-toolchain nightly --profile default && . "$(HOME)/.cargo/env"
	$(shell echo which rustup) && rustup default nightly

cargo-b:## 	cargo-b
##cargo build
	[ -x "$(shell command -v $(RUSTUP))" ] || $(MAKE) rustup-install-stable
	[ -x "$(shell command -v $(CARGO))" ] && $(CARGO) build
cargo-b-release:## 	cargo-b-release
##cargo build --releae --path .
	[ -x "$(shell command -v $(RUSTUP))" ] || $(MAKE) rustup-install-stable
	[ -x "$(shell command -v $(CARGO))" ] && $(CARGO) build --release
cargo-c:## 	cargo-c
##cargo check
	[ -x "$(shell command -v $(RUSTC))" ] || $(MAKE) rustup-install-stable
	[ -x "$(shell command -v $(CARGO))" ] && $(CARGO) c
cargo-i:## 	cargo-i
##cargo install
	[ -x "$(shell command -v $(RUSTC))" ] || $(MAKE) rustup-install-stable
	[ -x "$(shell command -v $(CARGO))" ] && $(CARGO) install --force --path .

-include Makefile
