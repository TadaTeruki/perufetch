
all:
	cargo run

install_for_unix:
	cargo build --release
	sudo cp target/release/perufetch /usr/local/bin

uninstall_for_unix:
	sudo rm /usr/local/bin/perufetch
