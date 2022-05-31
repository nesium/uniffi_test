all: build

build:
	# Inpired by https://betterprogramming.pub/from-rust-to-swift-df9bde59b7cd
	# Other useful links:
	# - https://developer.apple.com/documentation/apple-silicon/building-a-universal-macos-binary

	# macOS (Intel)
	cargo build --release --target=x86_64-apple-darwin
	# macOS (Apple Silicon)
	cargo build --release --target=aarch64-apple-darwin

	# iOS
	#cargo build --release --target aarch64-apple-ios
	# iOS Simulator
	#cargo build --release --target x86_64-apple-ios
	#cargo build --release --target aarch64-apple-ios-sim

	# Mac Catalyst
	#cargo build --release --target x86_64-apple-ios-macabi
	#cargo build --release --target aarch64-apple-ios-macabi
