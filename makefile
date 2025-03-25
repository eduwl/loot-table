# ================================================================================ #
#                                 Cargo Clear                                      #
# ================================================================================ #

.PHONY: clean
clean:
	@echo "Cleaning up..."
	cargo clean

# ================================================================================ #
#                                 Cargo Build DEV                                  #
# ================================================================================ #

.PHONY: build-linux
build-linux:
	@echo "Building for Linux..."
	cargo build --target x86_64-unknown-linux-gnu

.PHONY: build-windows
build-windows:
	@echo "Building for Windows..."
	cargo build --target x86_64-pc-windows-gnu

.PHONY: build-all
build-all: build-linux build-windows

# ================================================================================ #
#                                 Cargo Release                                    #
# ================================================================================ #

.PHONY: release-linux
release-linux:
	@echo "Building release for Linux..."
	cargo build --release --target x86_64-unknown-linux-gnu

.PHONY: release-windows
release-windows:
	@echo "Building release for Windows..."
	cargo build --release --target x86_64-pc-windows-gnu

.PHONY: release-all
release-all: release-linux release-windows

# ================================================================================ #
#                                  Cargo Run                                       #
# ================================================================================ #

.PHONY: run
run:
	@echo "Running server..."
	cargo run
